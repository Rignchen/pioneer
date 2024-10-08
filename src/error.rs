use crate::database::types::*;
use std::process::exit;

structstruck::strike! {
    /// Enum to represent the different types of errors that can occur in the application
    pub struct PioneerError {
        pub _pioneer_error: pub enum PioneerErrorType {
            ConnectionError(DatabaseType),
        },
        pub message: String,
    }
}

/// Result type which wither take a type T or a PioneerError
pub type PioneerResult<T> = std::result::Result<T, PioneerError>;

/// Function to handle the different types of errors that can occur in the application
pub fn handle_error(error: PioneerError) {
    match error._pioneer_error {
        PioneerErrorType::ConnectionError(db) => {
            match db {
                DatabaseType::Postgres(Postgres { host, port, .. }) => {
                    eprintln!("Error connecting to Postgres database at {}:{} ({})", host, port, error.message);
                }
                DatabaseType::Sqlite(Sqlite { file }) => {
                    eprintln!("Error connecting to Sqlite database at {} ({})", file, error.message);
                }
                DatabaseType::Mysql(Mysql { host, port, .. }) => {
                    eprintln!("Error connecting to Mysql database at {}:{} ({})", host, port, error.message);
                }
            }
            exit(1);
        }
    }
}
