use clap::Parser;
use crate::database::DatabaseType;

structstruck::strike! {
    #[strikethrough[derive(Parser, Debug)]]
    #[clap(verbatim_doc_comment)]
    #[command(version)]
    pub struct Args {
        pub db: DatabaseType,
        pub server: String,
        #[clap(short, long)]
        pub port: Option<u16>,
        #[clap(short, long)]
        pub user: Option<String>,
        #[clap(long)]
        pub password: Option<String>,
    }
}

