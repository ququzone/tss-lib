use anyhow::{anyhow, Context, Result};
use futures::{SinkExt, StreamExt, TryStreamExt};
use round_based::async_runtime::AsyncProtocol;
use round_based::Msg;

use curv::arithmetic::Converter;
use curv::BigInt;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::{
    party_i::SignatureRecid,
    state_machine::sign::{OfflineStage, SignManual},
};

use super::common::join_computation;

#[tokio::main]
pub async fn run(
    server_url: &str,
    room: &str,
    local_share: &[u8],
    parties: Vec<u16>,
    data: &[u8],
) -> Result<SignatureRecid> {
    let local_share = serde_json::from_slice(&local_share).context("parse local share")?;
    let number_of_parties = parties.len();

    let (i, incoming, outgoing) =
        join_computation(server_url.clone(), &format!("{}-offline", room))
            .await
            .context("join offline computation")?;

    let incoming = incoming.fuse();
    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let signing = OfflineStage::new(i, parties, local_share)?;
    let completed_offline_stage = AsyncProtocol::new(signing, incoming, outgoing)
        .run()
        .await
        .map_err(|e| anyhow!("protocol execution terminated with error: {}", e))?;

    let (i, incoming, outgoing) = join_computation(server_url, &format!("{}-online", room))
        .await
        .context("join online computation")?;

    tokio::pin!(incoming);
    tokio::pin!(outgoing);

    let (signing, partial_signature) =
        SignManual::new(BigInt::from_bytes(data), completed_offline_stage)?;

    outgoing
        .send(Msg {
            sender: i,
            receiver: None,
            body: partial_signature,
        })
        .await?;

    let partial_signatures: Vec<_> = incoming
        .take(number_of_parties - 1)
        .map_ok(|msg| msg.body)
        .try_collect()
        .await?;
    let signature = signing
        .complete(&partial_signatures)
        .context("online stage failed")?;

    Ok(signature)
}
