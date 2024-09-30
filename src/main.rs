use clap::Parser;
use pioneer::{
    cli::Args,
    error::{PioneerError, handle_error},
};

fn main() {
    let args = Args::parse();
    handle_error(PioneerError::ConnectionError(args));
}

