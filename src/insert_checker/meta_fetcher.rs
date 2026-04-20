use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySqlPool, Row};

use super::parser::SqlValue;

#[derive(Debug, Clone)]
pub struct ColumnMeta {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub has_default: bool,
    pub char_max_length: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct FkMeta {
    pub columns: Vec<String>,
    pub referenced_database: String,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TableMeta {
    pub columns: Vec<ColumnMeta>,
    pub primary_key: Vec<String>,
    pub unique_keys: HashMap<String, Vec<String>>,
    pub foreign_keys: Vec<FkMeta>,
}

pub struct MetaFetcher {
    pool: MySqlPool,
    cache: HashMap<(String, String), TableMeta>,
}

impl MetaFetcher {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool, cache: HashMap::new() }
    }

    pub async fn table_exists(&self, db: &str, table: &str) -> Result<bool> {
        let row = sqlx::query(
            "SELECT 1 FROM information_schema.TABLES \
             WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? LIMIT 1",
        )
        .bind(db)
        .bind(table)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.is_some())
    }

    pub async fn get_meta(&mut self, db: &str, table: &str) -> Result<&TableMeta> {
        let key = (db.to_string(), table.to_string());
        if !self.cache.contains_key(&key) {
            let meta = self.fetch_meta(db, table).await?;
            self.cache.insert(key.clone(), meta);
        }
        Ok(self.cache.get(&key).unwrap())
    }

    async fn fetch_meta(&self, db: &str, table: &str) -> Result<TableMeta> {
        let columns = self.fetch_columns(db, table).await?;
        let (primary_key, unique_keys) = self.fetch_keys(db, table).await?;
        let foreign_keys = self.fetch_fks(db, table).await?;
        Ok(TableMeta { columns, primary_key, unique_keys, foreign_keys })
    }

    async fn fetch_columns(&self, db: &str, table: &str) -> Result<Vec<ColumnMeta>> {
        let rows = sqlx::query(
            "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_DEFAULT, EXTRA, \
                    CHARACTER_MAXIMUM_LENGTH \
             FROM information_schema.COLUMNS \
             WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? \
             ORDER BY ORDINAL_POSITION",
        )
        .bind(db)
        .bind(table)
        .fetch_all(&self.pool)
        .await?;

        let mut result = vec![];
        for row in rows {
            let is_nullable: String = row.try_get("IS_NULLABLE")?;
            let col_default: Option<String> = row.try_get("COLUMN_DEFAULT")?;
            let extra: String = row.try_get("EXTRA")?;
            let has_default = col_default.is_some() || extra.contains("auto_increment");
            result.push(ColumnMeta {
                name: row.try_get("COLUMN_NAME")?,
                data_type: row.try_get("DATA_TYPE")?,
                is_nullable: is_nullable == "YES",
                has_default,
                char_max_length: row.try_get("CHARACTER_MAXIMUM_LENGTH")?,
            });
        }
        Ok(result)
    }

    async fn fetch_keys(
        &self,
        db: &str,
        table: &str,
    ) -> Result<(Vec<String>, HashMap<String, Vec<String>>)> {
        let rows = sqlx::query(
            "SELECT INDEX_NAME, COLUMN_NAME, NON_UNIQUE, SEQ_IN_INDEX \
             FROM information_schema.STATISTICS \
             WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? \
             ORDER BY INDEX_NAME, SEQ_IN_INDEX",
        )
        .bind(db)
        .bind(table)
        .fetch_all(&self.pool)
        .await?;

        let mut index_map: HashMap<String, (bool, Vec<(u32, String)>)> = HashMap::new();
        for row in rows {
            let index_name: String = row.try_get("INDEX_NAME")?;
            let col_name: String = row.try_get("COLUMN_NAME")?;
            let non_unique: i32 = row.try_get("NON_UNIQUE")?;
            let seq: u32 = row.try_get("SEQ_IN_INDEX")?;
            let entry = index_map.entry(index_name).or_insert((non_unique == 0, vec![]));
            entry.1.push((seq, col_name));
        }

        let mut primary_key = vec![];
        let mut unique_keys = HashMap::new();

        for (name, (is_unique, mut cols)) in index_map {
            cols.sort_by_key(|(seq, _)| *seq);
            let col_names: Vec<String> = cols.into_iter().map(|(_, c)| c).collect();
            if name == "PRIMARY" {
                primary_key = col_names;
            } else if is_unique {
                unique_keys.insert(name, col_names);
            }
        }

        Ok((primary_key, unique_keys))
    }

    async fn fetch_fks(&self, db: &str, table: &str) -> Result<Vec<FkMeta>> {
        let rows = sqlx::query(
            "SELECT kcu.CONSTRAINT_NAME, kcu.COLUMN_NAME, kcu.ORDINAL_POSITION, \
                    kcu.REFERENCED_TABLE_SCHEMA, kcu.REFERENCED_TABLE_NAME, \
                    kcu.REFERENCED_COLUMN_NAME \
             FROM information_schema.KEY_COLUMN_USAGE kcu \
             WHERE kcu.TABLE_SCHEMA = ? AND kcu.TABLE_NAME = ? \
               AND kcu.REFERENCED_TABLE_NAME IS NOT NULL \
             ORDER BY kcu.CONSTRAINT_NAME, kcu.ORDINAL_POSITION",
        )
        .bind(db)
        .bind(table)
        .fetch_all(&self.pool)
        .await?;

        let mut fk_map: HashMap<String, (String, String, Vec<String>, Vec<String>)> =
            HashMap::new();
        for row in rows {
            let constraint: String = row.try_get("CONSTRAINT_NAME")?;
            let col: String = row.try_get("COLUMN_NAME")?;
            let ref_db: String = row.try_get("REFERENCED_TABLE_SCHEMA")?;
            let ref_table: String = row.try_get("REFERENCED_TABLE_NAME")?;
            let ref_col: String = row.try_get("REFERENCED_COLUMN_NAME")?;
            let entry = fk_map
                .entry(constraint)
                .or_insert_with(|| (ref_db, ref_table, vec![], vec![]));
            entry.2.push(col);
            entry.3.push(ref_col);
        }

        Ok(fk_map
            .into_values()
            .map(|(ref_db, ref_table, cols, ref_cols)| FkMeta {
                columns: cols,
                referenced_database: ref_db,
                referenced_table: ref_table,
                referenced_columns: ref_cols,
            })
            .collect())
    }

    pub async fn check_row_exists(
        &self,
        db: &str,
        table: &str,
        conditions: &[(&str, &SqlValue)],
    ) -> Result<bool> {
        use sqlx::Arguments;
        let where_parts: Vec<String> =
            conditions.iter().map(|(col, _)| format!("`{}` = ?", col)).collect();
        let sql = format!(
            "SELECT 1 FROM `{}`.`{}` WHERE {} LIMIT 1",
            db,
            table,
            where_parts.join(" AND ")
        );

        let mut args = sqlx::mysql::MySqlArguments::default();
        for (_, val) in conditions {
            match val {
                SqlValue::Null => args.add(Option::<String>::None),
                SqlValue::Number(s) | SqlValue::Str(s) | SqlValue::Other(s) => {
                    args.add(s.clone())
                }
                SqlValue::Bool(b) => args.add(*b),
            }
        }

        let row = sqlx::query_with(&sql, args).fetch_optional(&self.pool).await?;
        Ok(row.is_some())
    }
}
