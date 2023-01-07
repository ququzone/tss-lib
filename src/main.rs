use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::tss::server;

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        // Constants
        Subcommands::Server => {
            _ = server::run();
        }
    }

    Ok(())
}
