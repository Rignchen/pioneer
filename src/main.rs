use clap::Parser;
use pioneer::{
    cli::Args,
    error::{handle_error, PioneerError},
};

fn main() {
    let args = Args::parse();
    handle_error(PioneerError::ConnectionError(args.db));
}
