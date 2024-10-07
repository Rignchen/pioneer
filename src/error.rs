use crate::database::types::*;
use std::process::exit;

/// Enum to represent the different types of errors that can occur in the application
pub enum PioneerError {
    ConnectionError(DatabaseType),
}

/// Result type which wither take a type T or a PioneerError
pub type PioneerResult<T> = std::result::Result<T, PioneerError>;

/// Function to handle the different types of errors that can occur in the application
pub fn handle_error(error: PioneerError) {
    match error {
        PioneerError::ConnectionError(db) => {
            match db {
                DatabaseType::Postgres(Postgres { host, port, .. }) => {
                    eprintln!("Error connecting to Postgres database at {}:{}", host, port);
                }
                DatabaseType::Sqlite(Sqlite { file }) => {
                    eprintln!("Error connecting to Sqlite database at {}", file);
                }
                DatabaseType::Mysql(Mysql { host, port, .. }) => {
                    eprintln!("Error connecting to Mysql database at {}:{}", host, port);
                }
            }
            exit(1);
        }
    }
}
