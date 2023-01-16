use anyhow::{anyhow, Context, Result, Ok};
use futures::StreamExt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::state_machine::keygen::Keygen;
use round_based::async_runtime::AsyncProtocol;

use super::common::join_computation;

#[tokio::main]
pub async fn run(
    server_url: &str,
    room: &str,
    index: u16,
    threshold: u16,
    number_of_parties: u16
) -> Result<Vec<u8>> {
    let (_i, incoming, outgoing) = join_computation(server_url, room)
        .await
        .context("join computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let keygen = Keygen::new(index, threshold, number_of_parties)?;
    let output = AsyncProtocol::new(keygen, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;
    let output = serde_json::to_vec_pretty(&output).unwrap();

    Ok(output)
}
