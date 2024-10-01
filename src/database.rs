use clap::Parser;

#[derive(Parser, Debug)]
pub enum DatabaseType {
    Postgres {
        host: String,
        port: u16,
        user: String,
        password: String,
    },
    Sqlite {
        file: String,
    },
    Mysql {
        host: String,
        port: u16,
        user: String,
        password: String,
    },
}

