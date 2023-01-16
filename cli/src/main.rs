use clap::Parser;

use tss_cli::opts::tss::{Opts, Subcommands};
use tss_cli::opts::tx;
use tss_lib::keygen;
use tss_lib::{server, sign};
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
        // Sign data
        Subcommands::Sign {
            server_url,
            room,
            parties,
            local_share,
            data,
        } => {
            let signature =
                sign::run(&server_url, &room, &local_share, parties, data.as_bytes()).unwrap();
            println!(
                "signature {{ r:0x{}, s:0x{}, v:{} }}",
                hex::encode(signature.r.to_bytes().as_ref()),
                hex::encode(signature.s.to_bytes().as_ref()),
                signature.recid,
            );
        }
        // Sign transaction
        Subcommands::SignTx {
            server_url,
            room,
            parties,
            local_share,
            chain_id,
            nonce,
            to,
            value,
            gas,
            gas_price,
            data,
        } => {
            let t = tx::Transaction::from(
                &nonce,
                &to,
                &value,
                &gas,
                &gas_price,
                data.as_bytes().to_vec(),
            );
            let sighash = t.sighash(chain_id);

            let signature = sign::run(&server_url, &room, &local_share, parties, &sighash).unwrap();

            println!(
                "signature {{ r:0x{}, s:0x{}, v:{} }}",
                hex::encode(signature.r.to_bytes().as_ref()),
                hex::encode(signature.s.to_bytes().as_ref()),
                signature.recid,
            );

            let v = match chain_id {
                0 => signature.recid as u64 + 27,
                _ => signature.recid as u64 + 35 + chain_id * 2,
            };

            let signed = t.encode(
                chain_id,
                Some(&Signature {
                    r: H256::from_slice(signature.r.to_bytes().as_ref()),
                    s: H256::from_slice(signature.s.to_bytes().as_ref()),
                    v,
                }),
            );

            println!("raw transaction: 0x{}", hex::encode(signed))
        }
    }

    Ok(())
}
