use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::tss::{server, sign_tx};
use tss_cli::tss::keygen;

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
            _ = keygen::run(&server_url, &room, index, threshold, number_of_parties, &output);
        }
        Subcommands::SignTx {
            server_url,
            room,
            parties,
            local_share,
            output 
        } => {
            _ = sign_tx::run(&server_url, &room, &local_share, parties, output)
        },
    }

    Ok(())
}
