#[derive(clap::ValueEnum, Debug, Clone)]
pub enum DatabaseType {
    Postgres,
    Sqlite,
    Mysql,
}
