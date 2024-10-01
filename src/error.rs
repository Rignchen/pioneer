use crate::database::DatabaseType;
use std::process::exit;

/// Enum to represent the different types of errors that can occur in the depot.
pub enum PioneerError {
    ConnectionError(DatabaseType),
}

/// Result type which wither take a type T or a DepotError.
pub type DepotResult<T> = std::result::Result<T, PioneerError>;

/*
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
*/

pub fn handle_error(error: PioneerError) {
    match error {
        PioneerError::ConnectionError(db) => {
            match db {
                DatabaseType::Postgres { host, port, .. } => {
                    eprintln!("Error connecting to Postgres database at {}:{}", host, port);
                },
                DatabaseType::Sqlite { file } => {
                    eprintln!("Error connecting to Sqlite database at {}", file);
                },
                DatabaseType::Mysql { host, port, .. } => {
                    eprintln!("Error connecting to Mysql database at {}:{}", host, port);
                },
            }
            exit(1);
        }
    }
}
