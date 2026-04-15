use clap::Parser;

/// MySQL schema diff tool — compares two MySQL instances and produces a Markdown report.
#[derive(Parser, Debug)]
#[command(name = "mysql-schema-diff")]
pub struct Cli {
    /// Base database host
    #[arg(long)]
    pub base_host: String,

    /// Base database port
    #[arg(long, default_value = "3306")]
    pub base_port: u16,

    /// Base database username
    #[arg(long)]
    pub base_user: String,

    /// Base database password
    #[arg(long)]
    pub base_password: String,

    /// Check database host
    #[arg(long)]
    pub check_host: String,

    /// Check database port
    #[arg(long, default_value = "3306")]
    pub check_port: u16,

    /// Check database username
    #[arg(long)]
    pub check_user: String,

    /// Check database password
    #[arg(long)]
    pub check_password: String,

    /// Comma-separated list of databases to compare.
    /// If omitted, all non-system databases are compared.
    #[arg(long, value_delimiter = ',')]
    pub databases: Option<Vec<String>>,

    /// Output file path. Defaults to ./schema-diff-YYYYMMDD-HHMMSS.md
    #[arg(long)]
    pub output: Option<String>,
}

/// Connection parameters for one MySQL instance.
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

/// Non-connection options parsed from the CLI.
#[derive(Debug)]
pub struct Options {
    /// If Some, only compare these database names. If None, compare all non-system databases.
    pub databases: Option<Vec<String>>,
    /// Output file path (may be None; caller generates the default name).
    pub output: Option<String>,
    /// Human-readable label for the base instance used in the report.
    pub base_label: String,
    /// Human-readable label for the check instance used in the report.
    pub check_label: String,
}

impl Cli {
    pub fn base_config(&self) -> ConnectionConfig {
        ConnectionConfig {
            host: self.base_host.clone(),
            port: self.base_port,
            user: self.base_user.clone(),
            password: self.base_password.clone(),
        }
    }

    pub fn check_config(&self) -> ConnectionConfig {
        ConnectionConfig {
            host: self.check_host.clone(),
            port: self.check_port,
            user: self.check_user.clone(),
            password: self.check_password.clone(),
        }
    }

    pub fn options(&self) -> Options {
        Options {
            databases: self.databases.clone(),
            output: self.output.clone(),
            base_label: format!("{}:{}", self.base_host, self.base_port),
            check_label: format!("{}:{}", self.check_host, self.check_port),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parses_required_args() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "10.0.0.1",
            "--base-user", "root",
            "--base-password", "secret",
            "--check-host", "10.0.0.2",
            "--check-user", "root",
            "--check-password", "secret2",
        ]);
        assert_eq!(cli.base_host, "10.0.0.1");
        assert_eq!(cli.base_port, 3306);
        assert_eq!(cli.check_host, "10.0.0.2");
        assert_eq!(cli.check_port, 3306);
        assert!(cli.databases.is_none());
        assert!(cli.output.is_none());
    }

    #[test]
    fn test_cli_parses_optional_databases() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--databases", "db1,db2,db3",
        ]);
        assert_eq!(cli.databases, Some(vec!["db1".into(), "db2".into(), "db3".into()]));
    }

    #[test]
    fn test_labels() {
        let cli = Cli::parse_from([
            "mysql-schema-diff",
            "--base-host", "10.0.0.1", "--base-port", "3307",
            "--base-user", "u", "--base-password", "p",
            "--check-host", "10.0.0.2",
            "--check-user", "u", "--check-password", "p",
        ]);
        let opts = cli.options();
        assert_eq!(opts.base_label, "10.0.0.1:3307");
        assert_eq!(opts.check_label, "10.0.0.2:3306");
    }
}
