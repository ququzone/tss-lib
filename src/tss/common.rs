use std::convert::TryInto;

use anyhow::{Context, Result};
use futures::{Sink, Stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surf::Url;
use round_based::Msg;

pub async fn join_computation<M>(
    address: &str,
    room_id: &str,
) -> Result<(
    u16,
    impl Stream<Item = Result<Msg<M>>>,
    impl Sink<Msg<M>, Error = anyhow::Error>,
)>
where
    M: Serialize + DeserializeOwned,
{
    let url = Url::parse(address)?;
    let client = SmClient::new(url, room_id).context("construct SmClient")?;

    // Construct channel of incoming messages
    let incoming = client
        .subscribe()
        .await
        .context("subscribe")?
        .and_then(|msg| async move {
            serde_json::from_str::<Msg<M>>(&msg).context("deserialize message")
        });

    // Obtain party index
    let index = client.issue_index().await.context("issue an index")?;

    // Ignore incoming messages addressed to someone else
    let incoming = incoming.try_filter(move |msg| {
        futures::future::ready(
            msg.sender != index && (msg.receiver.is_none() || msg.receiver == Some(index)),
        )
    });

    // Construct channel of outgoing messages
    let outgoing = futures::sink::unfold(client, |client, message: Msg<M>| async move {
        let serialized = serde_json::to_string(&message).context("serialize message")?;
        client
            .broadcast(&serialized)
            .await
            .context("broadcast message")?;
        Ok::<_, anyhow::Error>(client)
    });

    Ok((index, incoming, outgoing))
}

pub struct SmClient {
    http_client: surf::Client,
}

impl SmClient {
    pub fn new(address: surf::Url, room_id: &str) -> Result<Self> {
        let config = surf::Config::new()
            .set_base_url(address.join(&format!("rooms/{}/", room_id))?)
            .set_timeout(None);
        Ok(Self {
            http_client: config.try_into()?,
        })
    }

    pub async fn issue_index(&self) -> Result<u16> {
        let response = self
            .http_client
            .post("issue_unique_idx")
            .recv_json::<IssuedUniqueIdx>()
            .await
            .map_err(|e| e.into_inner())?;
        Ok(response.unique_idx)
    }

    pub async fn broadcast(&self, message: &str) -> Result<()> {
        self.http_client
            .post("broadcast")
            .body(message)
            .await
            .map_err(|e| e.into_inner())?;
        Ok(())
    }

    pub async fn subscribe(&self) -> Result<impl Stream<Item = Result<String>>> {
        let response = self
            .http_client
            .get("subscribe")
            .await
            .map_err(|e| e.into_inner())?;
        let events = async_sse::decode(response);
        Ok(events.filter_map(|msg| async {
            match msg {
                Ok(async_sse::Event::Message(msg)) => Some(
                    String::from_utf8(msg.into_bytes())
                        .context("SSE message is not valid UTF-8 string"),
                ),
                Ok(_) => {
                    // ignore other types of events
                    None
                }
                Err(e) => Some(Err(e.into_inner())),
            }
        }))
    }
}

#[derive(Deserialize, Debug)]
struct IssuedUniqueIdx {
    unique_idx: u16,
}
