use futures::{future, pin_mut, StreamExt, TryStreamExt};
use futures::channel::mpsc::UnboundedReceiver;
use log::*;
use super::error::Result;
use super::data::KolliderMsg;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::io::AsyncWriteExt;

pub const KOLLIDER_WEBSOCKET: &str = "wss://api.kollider.xyz/v1/ws/";

pub async fn kollider_websocket(msg_rx: UnboundedReceiver<KolliderMsg>) -> Result<()> {
    let url = url::Url::parse(KOLLIDER_WEBSOCKET)?;

    let (ws_stream, _) = connect_async(url).await?;
    info!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = msg_rx.map(|msg| Ok(Message::text(serde_json::to_string(&msg).unwrap())) ).forward(write);
    let ws_to_stdout = {
        read.try_for_each(|message| async {
            match message {
                Message::Close(_) => {
                    info!("Websocket is closed by remote side")
                }
                _ => (),
            }
            let data = message.into_data();
            info!("Incoming WS message: {}", std::str::from_utf8(&data)?);
            tokio::io::stdout().write_all(&data).await?;
            Ok(())
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
    info!("Websocket worker exited");
    Ok(())
}
