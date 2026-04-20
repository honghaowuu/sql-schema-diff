use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "sqltool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Diff(DiffArgs),
    CheckInserts(CheckInsertsArgs),
}

#[derive(Args, Debug)]
pub struct DiffArgs {
    #[arg(long)]
    pub base_host: String,

    #[arg(long, default_value = "3306")]
    pub base_port: u16,

    #[arg(long)]
    pub base_user: String,

    #[arg(long)]
    pub base_password: String,

    #[arg(long)]
    pub check_host: String,

    #[arg(long, default_value = "3306")]
    pub check_port: u16,

    #[arg(long)]
    pub check_user: String,

    #[arg(long)]
    pub check_password: String,

    #[arg(long, value_delimiter = ',')]
    pub databases: Option<Vec<String>>,

    #[arg(long)]
    pub output: Option<String>,

    #[arg(long, default_value_t = false)]
    pub ignore_base_only_dbs: bool,
}

#[derive(Args, Debug)]
pub struct CheckInsertsArgs {
    #[arg(long)]
    pub host: String,

    #[arg(long, default_value = "3306")]
    pub port: u16,

    #[arg(long)]
    pub user: String,

    #[arg(long)]
    pub password: String,

    #[arg(long)]
    pub database: String,

    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub output: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Options {
    pub databases: Option<Vec<String>>,
    pub output: Option<String>,
    pub base_label: String,
    pub check_label: String,
    pub ignore_base_only_dbs: bool,
}

impl DiffArgs {
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
            ignore_base_only_dbs: self.ignore_base_only_dbs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn diff_cli(args: &[&str]) -> DiffArgs {
        let mut full = vec!["sqltool", "diff"];
        full.extend_from_slice(args);
        match Cli::parse_from(full).command {
            SubCommand::Diff(a) => a,
            _ => panic!("expected diff"),
        }
    }

    #[test]
    fn test_cli_parses_required_args() {
        let a = diff_cli(&[
            "--base-host", "10.0.0.1",
            "--base-user", "root",
            "--base-password", "secret",
            "--check-host", "10.0.0.2",
            "--check-user", "root",
            "--check-password", "secret2",
        ]);
        assert_eq!(a.base_host, "10.0.0.1");
        assert_eq!(a.base_port, 3306);
        assert_eq!(a.check_host, "10.0.0.2");
        assert_eq!(a.check_port, 3306);
        assert!(a.databases.is_none());
        assert!(a.output.is_none());
    }

    #[test]
    fn test_cli_parses_optional_databases() {
        let a = diff_cli(&[
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--databases", "db1,db2,db3",
        ]);
        assert_eq!(a.databases, Some(vec!["db1".into(), "db2".into(), "db3".into()]));
    }

    #[test]
    fn test_labels() {
        let a = diff_cli(&[
            "--base-host", "10.0.0.1", "--base-port", "3307",
            "--base-user", "u", "--base-password", "p",
            "--check-host", "10.0.0.2",
            "--check-user", "u", "--check-password", "p",
        ]);
        let opts = a.options();
        assert_eq!(opts.base_label, "10.0.0.1:3307");
        assert_eq!(opts.check_label, "10.0.0.2:3306");
    }

    #[test]
    fn test_ignore_base_only_dbs_flag() {
        let a = diff_cli(&[
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
            "--ignore-base-only-dbs",
        ]);
        assert!(a.options().ignore_base_only_dbs);
    }

    #[test]
    fn test_ignore_base_only_dbs_default_false() {
        let a = diff_cli(&[
            "--base-host", "h1", "--base-user", "u", "--base-password", "p",
            "--check-host", "h2", "--check-user", "u", "--check-password", "p",
        ]);
        assert!(!a.options().ignore_base_only_dbs);
    }
}
