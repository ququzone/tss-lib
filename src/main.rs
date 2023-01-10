use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::tss::keygen;
use tss_cli::tss::{server, sign};

fn main() -> eyre::Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        // Server
        Subcommands::Server => {
            _ = server::run();
        }
        // Keygen
        Subcommands::Keygen {
            server_url,
            room,
            index,
            threshold,
            number_of_parties,
            output,
        } => {
            _ = keygen::run(
                &server_url,
                &room,
                index,
                threshold,
                number_of_parties,
                &output,
            );
        }
        // Sign transaction
        Subcommands::SignTx {
            server_url,
            room,
            parties,
            local_share,
            output,
        } => {
            let data = output.as_bytes();
            let signature = sign::run(&server_url, &room, &local_share, parties, data);
            let signature = serde_json::to_string(&signature.expect("sign data fail"));
            println!(
                "Signature: {}",
                signature.expect("serialize signature fail")
            );
        }
    }

    Ok(())
}
