use clap::Parser;
use pioneer::cli::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
