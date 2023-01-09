use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "tss", version = "0.1.0")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Perform tss operations from your command line.",
    after_help = "Find more information can refer code: https://github.com/ququzone/tss-cli",
    next_display_order = None
)]
pub enum Subcommands {
    #[clap(name = "server")]
    #[clap(visible_aliases = &["srv"])]
    #[clap(about = "Start the tss http server.")]
    Server,

    #[clap(name = "keygen")]
    #[clap(visible_alias = "kg")]
    #[clap(about = "Generate secp256k1 keys.")]
    Keygen {
        #[clap(
            long,
            env = "SERVER_URL",
            value_name = "URL",
            default_value = "http://localhost:8000/"
        )]
        server_url: String,
        #[clap(
            long,
            value_name = "ROOM",
            default_value = "default-keygen"
        )]
        room: String,
        #[clap(long, short)]
        index: u16,
        #[clap(long, short)]
        threshold: u16,
        #[clap(long, short)]
        number_of_parties: u16,
        #[clap(value_name = "output")]
        output: String,
    },

    #[clap(name = "sign-tx")]
    #[clap(visible_alias = "st")]
    #[clap(about = "Sign ethereum transaction.")]
    SignTx {
        #[clap(
            long,
            env = "SERVER_URL",
            value_name = "URL",
            default_value = "http://localhost:8000/"
        )]
        server_url: String,
        #[clap(
            long,
            value_name = "ROOM",
            default_value = "default-keygen",
        )]
        room: String,
        #[clap(long, short, use_value_delimiter = true)]
        parties: Vec<u16>,
        #[clap(long, short)]
        local_share: String,
        #[clap(value_name = "output")]
        output: String,
    },
}
