use clap::Parser;
use pioneer::{
    cli::Args,
    error::{handle_error, PioneerError},
};

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
