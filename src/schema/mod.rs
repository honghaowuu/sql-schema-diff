use std::collections::HashMap;

/// Top-level snapshot of all schemas from one MySQL instance.
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaSnapshot {
    /// Key: database (schema) name.
    pub databases: HashMap<String, DatabaseSchema>,
}

/// All objects inside one MySQL database/schema.
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseSchema {
    pub name: String,
    pub tables: HashMap<String, TableDef>,
    pub views: HashMap<String, ViewDef>,
    pub procedures: HashMap<String, RoutineDef>,
    pub functions: HashMap<String, RoutineDef>,
    pub triggers: HashMap<String, TriggerDef>,
}

/// A single base table (excludes views).
#[derive(Debug, Clone, PartialEq)]
pub struct TableDef {
    pub name: String,
    /// Ordered by ordinal_position.
    pub columns: Vec<ColumnDef>,
    /// Ordered by index name.
    pub indexes: Vec<IndexDef>,
    /// Ordered by constraint name.
    pub foreign_keys: Vec<ForeignKeyDef>,
    /// Ordered by constraint name. Empty on MySQL < 8.0.
    pub checks: Vec<CheckDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    /// Full type string from MySQL, e.g. "varchar(255)", "int unsigned".
    pub column_type: String,
    pub is_nullable: bool,
    pub column_default: Option<String>,
    /// e.g. "auto_increment", "on update CURRENT_TIMESTAMP", or empty string.
    pub extra: String,
    pub ordinal_position: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexDef {
    pub name: String,
    /// BTREE or HASH.
    pub index_type: String,
    pub is_unique: bool,
    /// Ordered by seq_in_index.
    pub columns: Vec<IndexColumn>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexColumn {
    pub column_name: String,
    pub seq_in_index: u32,
    /// Prefix length for partial indexes (e.g. on TEXT columns). None = full column.
    pub sub_part: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyDef {
    pub name: String,
    /// Columns in this table, in key order.
    pub columns: Vec<String>,
    pub ref_table: String,
    /// Corresponding columns in the referenced table.
    pub ref_columns: Vec<String>,
    pub on_delete: String,
    pub on_update: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckDef {
    pub name: String,
    pub clause: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewDef {
    pub name: String,
    /// Normalized (whitespace-collapsed) SQL definition.
    pub definition: String,
}

/// A stored procedure or function.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutineDef {
    pub name: String,
    /// "PROCEDURE" or "FUNCTION".
    pub routine_type: String,
    /// Normalized body.
    pub definition: String,
    /// "YES" or "NO".
    pub is_deterministic: String,
    /// "CONTAINS SQL", "NO SQL", "READS SQL DATA", or "MODIFIES SQL DATA".
    pub sql_data_access: String,
    /// "DEFINER" or "INVOKER".
    pub security_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerDef {
    pub name: String,
    /// "INSERT", "UPDATE", or "DELETE".
    pub event: String,
    /// "BEFORE" or "AFTER".
    pub timing: String,
    pub table_name: String,
    /// Normalized trigger body.
    pub statement: String,
    pub action_order: u32,
}
