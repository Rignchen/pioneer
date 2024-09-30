use clap::Parser;

structstruck::strike! {
    #[strikethrough[derive(Parser, Debug)]]
    #[clap(verbatim_doc_comment)]
    #[command(version)]
    pub struct Args {
        #[clap(subcommand)]
        pub cmd: pub enum Command {
        }
    }
}