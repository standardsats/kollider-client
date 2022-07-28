use super::client::kollider_websocket;
use super::data::{
    make_user_auth, AuthError, BalancesCash, FetchBalancesTag, KolliderMsg, KolliderTaggedMsg,
};
use crate::kollider::api::Symbol;
use crate::kollider::client::env::KolliderAuth;
use futures::future::Future;
use futures::StreamExt;
use futures_channel::mpsc::{TrySendError, UnboundedSender};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug)]
pub struct Balances {
    pub cash: BalancesCash,
    pub cross_margin: f64,
    pub isolated_margin: HashMap<Symbol, f64>,
    pub order_margin: HashMap<Symbol, f64>,
}
use std::time::Duration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to make auth data: {0}")]
    Auth(#[from] AuthError),
    #[error("Failed to communicate via channel: {0}")]
    Channel(#[from] TrySendError<KolliderMsg>),
    #[error("There is no response from the server")]
    NoResponse,
}

/// Helper to create oneshot sync requests via websocket. Open socket, request, wait for response, close.
pub async fn oneshot_ws_request<F, Fut, T>(auth: &KolliderAuth, body: F) -> Result<T, Error>
where
    F: FnOnce(UnboundedSender<KolliderMsg>, KolliderMsg) -> Fut + Copy,
    Fut: Future<Output = Result<Option<T>, Error>>,
{
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let (msg_sender, mut msg_receiver) = futures_channel::mpsc::unbounded();
    let secret_str = base64::encode(&auth.api_secret);
    let auth_msg = make_user_auth(&secret_str, &auth.api_key, &auth.password)?;
    stdin_tx.unbounded_send(auth_msg)?;
    tokio::spawn(kollider_websocket(stdin_rx, msg_sender));

    let listen_fut = async move {
        loop {
            let message = msg_receiver.next().await;
            match message {
                Some(msg) => match body(stdin_tx.clone(), msg).await {
                    Ok(Some(v)) => return Ok(v),
                    Ok(None) => (),
                    Err(e) => return Err(e),
                },
                None => return Err(Error::NoResponse),
            }
        }
    };

    let res = tokio::time::timeout(Duration::from_secs(60), listen_fut).await;
    match res {
        Err(_) => Err(Error::NoResponse),
        Ok(e) => e,
    }
}

/// Open websocket and request balances as synchronous request
pub async fn fetch_balances(auth: &KolliderAuth) -> Result<Balances, Error> {
    oneshot_ws_request(auth, |stdin_tx, message| async move {
        match message {
            KolliderMsg::Tagged(KolliderTaggedMsg::Authenticate { message })
                if message == "success" =>
            {
                stdin_tx.unbounded_send(KolliderMsg::FetchBalances {
                    _type: FetchBalancesTag::Tag,
                })?;
                Ok(None)
            }
            KolliderMsg::Tagged(KolliderTaggedMsg::Balances {
                cash,
                cross_margin,
                isolated_margin,
                order_margin,
            }) => {
                return Ok(Some(Balances {
                    cash,
                    cross_margin,
                    isolated_margin: isolated_margin.into_iter().map(|(k, v)| (k, v.0)).collect(),
                    order_margin: order_margin.into_iter().map(|(k, v)| (k, v.0)).collect(),
                }))
            }
            _ => Ok(None),
        }
    })
    .await
}
