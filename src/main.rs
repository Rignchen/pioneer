pub mod cli;
pub mod database;
pub mod error;

use clap::Parser;
use cli::Args;
use error::{handle_error, PioneerError};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => handle_error(e),
    }
}

fn run() -> Result<(), PioneerError> {
    let args = Args::parse();
    args.db.connect()?;
    Ok(())
}
