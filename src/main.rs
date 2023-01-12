use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::tss::{keygen, tx};
use tss_cli::tss::{server, sign};
use web3::signing::Signature;
use web3::types::H256;

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
            let t = tx::Transaction::from( 
                "0",
                "0x173553c179bbf5af39D8Db41F0B60e4Fc631066a",
                "100",
                "10000",
                "1000000000000",
                vec![],
            );
            let sighash = t.sighash(4690);

            println!("sighash {}", hex::encode(sighash));

            let signature = sign::run(&server_url, &room, &local_share, parties, &sighash).unwrap();

            println!(
                "signature {{ r:{}, s:{}, v:{} }}",
                hex::encode(signature.r.to_bytes().as_ref()),
                hex::encode(signature.s.to_bytes().as_ref()),
                signature.recid,
            );

            let v = signature.recid as u64 + 35 + 4690*2;
            
            let signed = t.encode(4690, Some(&Signature {
                r: H256::from_slice(signature.r.to_bytes().as_ref()),
                s: H256::from_slice(signature.s.to_bytes().as_ref()),
                v,
            }));

            println!("raw transaction: 0x{}", hex::encode(signed))
        }
    }

    Ok(())
}
