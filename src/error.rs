use crate::cli::Args;
use std::process::exit;

/// Enum to represent the different types of errors that can occur in the depot.
pub enum PioneerError {
    ConnectionError(Args),
}

/// Result type which wither take a type T or a DepotError.
pub type DepotResult<T> = std::result::Result<T, PioneerError>;

pub fn handle_error(error: PioneerError) {
    match error {
        PioneerError::ConnectionError(args) => {
            eprintln!("Error: Could not connect to the {:?} database on the {} server", args.db, args.server);
            exit(1);
        }
    }
}

