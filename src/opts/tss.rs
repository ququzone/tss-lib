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
    #[clap(name = "--server")]
    #[clap(visible_aliases = &["server"])]
    #[clap(about = "Start the tss http server.")]
    Server,
}
