use crate::database::DatabaseType;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(verbatim_doc_comment)]
#[command(version)]
pub struct Args {
    #[clap(subcommand)]
    pub db: DatabaseType,
}
