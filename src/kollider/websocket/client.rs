use futures::{future, pin_mut, StreamExt, TryStreamExt};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use log::*;
use super::error::Result;
use super::data::KolliderMsg;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub const KOLLIDER_WEBSOCKET: &str = "wss://api.kollider.xyz/v1/ws/";

pub async fn kollider_websocket(msg_outcoming: UnboundedReceiver<KolliderMsg>, msg_incoming: UnboundedSender<KolliderMsg>) -> Result<()> {
    let url = url::Url::parse(KOLLIDER_WEBSOCKET)?;

    let (ws_stream, _) = connect_async(url).await?;
    debug!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = msg_outcoming.map(|msg| {
        let msg_str = serde_json::to_string(&msg).unwrap();
        debug!("Sending WS message: {}", msg_str);
        Ok(Message::text(msg_str))
    }).forward(write);
    let ws_to_stdout = {
        read.try_for_each(|message| async {
            match message {
                Message::Ping(data) => trace!("Ping {:?}", data),
                Message::Pong(data) => trace!("Pong {:?}", data),
                Message::Close(_) => {
                    debug!("Websocket is closed by remote side")
                }
                _ => {
                    let data = message.into_text()?;
                    match serde_json::from_str(&data) {
                        Err(e) => {
                            error!("Failed to decode WS message with error {}, body: {}", e, data);
                        }
                        Ok(msg) => {
                            debug!("Incoming WS message: {:?}", msg);
                            msg_incoming.unbounded_send(msg).unwrap();
                        }
                    }
                },
            }
            Ok(())
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
    debug!("Websocket worker exited");
    Ok(())
}
