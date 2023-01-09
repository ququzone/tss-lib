use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::tss::server;
use tss_cli::tss::kengen;

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        // Server
        Subcommands::Server => {
            _ = server::run();
        },
        // Keygen
        Subcommands::Keygen { 
            server_url,
            room,
            index, threshold, number_of_parties,
            output,
        } => {
            _ = kengen::run(&server_url, &room, index, threshold, number_of_parties, &output);
        }
    }

    Ok(())
}
