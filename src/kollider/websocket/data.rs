use crate::kollider::api::{KeyPrice, MarginType, OrderSide, OrderType, SettlementType, Symbol};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use log::*;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use sha2::Sha256;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum KolliderMsg {
    Subscribe {
        #[serde(rename = "type")]
        _type: SubscribeTag,
        symbols: Vec<Symbol>,
        channels: Vec<ChannelName>,
    },
    Unsubscribe {
        #[serde(rename = "type")]
        _type: UnsubscribeTag,
        symbols: Vec<Symbol>,
        channels: Vec<ChannelName>,
    },
    UserAuth {
        #[serde(rename = "type")]
        _type: AuthenticateTag,
        /// API Key
        token: String,
        /// API passphrase
        passphrase: String,
        /// Signature BASE64(HMAC_SHA256(timestamp + "authentication", API_SECRET))
        signature: String,
        /// Timestamp
        timestamp: String,
    },
    Error {
        #[serde(rename = "type")]
        _type: ErrorTag,
        message: String,
    },
    Order {
        #[serde(rename = "type")]
        _type: OrderTag,
        price: u64,
        quantity: u64,
        symbol: Symbol,
        leverage: u64,
        side: OrderSide,
        margin_type: MarginType,
        order_type: OrderType,
        settlement_type: SettlementType,
        ext_order_id: String,
    },
    CancelOrder {
        #[serde(rename = "type")]
        _type: CancelOrderTag,
        order_id: u64,
        symbol: String,
        settlement_type: SettlementType,
    },
    FetchOpenOrders {
        #[serde(rename = "type")]
        _type: FetchOpenOrdersTag,
    },
    FetchPositions {
        #[serde(rename = "type")]
        _type: FetchPositionsTag,
    },
    GetTicker {
        #[serde(rename = "type")]
        _type: GetTickerTag,
        symbol: String,
    },
    TradableProducts {
        #[serde(rename = "type")]
        _type: TradableProductsTag,
    },
    Tagged(KolliderTaggedMsg),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum SubscribeTag {
    #[serde(rename = "subscribe")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum UnsubscribeTag {
    #[serde(rename = "unsubscribe")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum AuthenticateTag {
    #[serde(rename = "authenticate")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderTag {
    #[serde(rename = "order")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum CancelOrderTag {
    #[serde(rename = "cancel_order")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum FetchOpenOrdersTag {
    #[serde(rename = "fetch_open_orders")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum FetchPositionsTag {
    #[serde(rename = "fetch_positions")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum GetTickerTag {
    #[serde(rename = "get_ticker")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum TradableProductsTag {
    #[serde(rename = "fetch_tradable_products")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ErrorTag {
    #[serde(rename = "error")]
    Tag,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct WrappedPrice(#[serde(deserialize_with = "deserialize_number_from_string")] f64);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase", tag = "type", content = "data")]
pub enum KolliderTaggedMsg {
    #[serde(rename = "index_values")]
    IndexValues(IndexValue),
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "success")]
    Success(String),
    #[serde(rename = "level2state")]
    OrderBookLevel2(OrderBookLevel2),
    #[serde(rename = "authenticate")]
    Authenticate { message: String },
    #[serde(rename = "received")]
    Received {
        uid: u64,
        order_id: u64,
        price: u64,
        quantity: u64,
        symbol: Symbol,
        leverage: u64,
        order_type: OrderType,
        ext_order_id: String,
        timestamp: u64,
    },
    #[serde(rename = "balances")]
    Balances {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        cash: f64,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        cross_margin: f64,
        isolated_margin: HashMap<Symbol, WrappedPrice>,
        order_margin: HashMap<Symbol, WrappedPrice>,
    },
    #[serde(rename = "open")]
    Open {
        order_id: u64,
        price: u64,
        quantity: u64,
        symbol: Symbol,
        leverage: u64,
        side: OrderSide,
        margin_type: MarginType,
        order_type: OrderType,
        settlement_type: SettlementType,
        ext_order_id: String,
        timestamp: u64,
        filled: u64,
    },
    #[serde(rename = "user_advanced_orders")]
    AdvancedOrders {
        // TODO: {"orders":{}}
    },
    #[serde(rename = "open_orders")]
    OpenOrders {
        open_orders: HashMap<Symbol, Vec<OpenOrder>>,
    },
    #[serde(rename = "positions")]
    Positions {
        positions: HashMap<Symbol, Position>,
    },
    #[serde(rename = "withdrawal_limit_info")]
    WithdrawalLimitInfo {
        daily_withdrawal_limits: HashMap<String, u64>,
        daily_withdrawal_volumes: HashMap<String, u64>,
    },
    #[serde(rename = "done")]
    Done {
        orde_type: OrderType, // TODO: Report typo
        order_id: u64,
        reason: String,
        symbol: Symbol,
        timestamp: u64,
    },
    #[serde(rename = "order_not_found")]
    OrderNotFound {
        order_id: u64,
        symbol: Symbol,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OpenOrder {
    // "advanced_order_type": null,
    ext_order_id: String,
    filled: u64,
    leverage: u64,
    margin_type: MarginType,
    order_id: u64,
    order_type: OrderType,
    price: u64,
    quantity: u64,
    settlement_type: SettlementType,
    side: OrderSide,
    symbol: Symbol,
    timestamp: u64,
    // "trigger_price_type": null,
    uid: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Position {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    adl_score: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    bankruptcy_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    entry_price: f64,
    entry_time: Option<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    entry_value: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    funding: f64,
    is_liquidating: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    liq_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    mark_value: f64,
    open_order_ids: Vec<u64>,
    position_id: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    quantity: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    real_leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    rpnl: f64,
    side: Option<OrderSide>,
    symbol: Symbol,
    timestamp: u64,
    uid: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    upnl: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ChannelName {
    #[serde(rename = "index_values")]
    IndexValues,
    #[serde(rename = "orderbook_level1")]
    OrderBookLevel1,
    #[serde(rename = "orderbook_level2")]
    OrderBookLevel2,
    #[serde(rename = "orderbook_level3")]
    OrderBookLevel3,
    #[serde(rename = "ticker")]
    Ticker,
    #[serde(rename = "matches")]
    Matches,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownChannelName(String);

impl std::error::Error for UnknownChannelName {}

impl fmt::Display for UnknownChannelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given ChannelName '{}' is unknown, valid are: index_values, orderbook_level1, orderbook_level2, orderbook_level3, ticker, matches",
            self.0
        )
    }
}

impl FromStr for ChannelName {
    type Err = UnknownChannelName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "index_values" => Ok(ChannelName::IndexValues),
            "orderbook_level1" => Ok(ChannelName::OrderBookLevel1),
            "orderbook_level2" => Ok(ChannelName::OrderBookLevel2),
            "orderbook_level3" => Ok(ChannelName::OrderBookLevel3),
            "ticker" => Ok(ChannelName::Ticker),
            "matches" => Ok(ChannelName::Matches),
            _ => Err(UnknownChannelName(s.to_owned())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct IndexValue {
    pub denom: String,
    pub symbol: Symbol,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum UpdateType {
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "snapshot")]
    Snapshot,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct OrderBookLevel2 {
    pub asks: HashMap<KeyPrice, u64>,
    pub bids: HashMap<KeyPrice, u64>,
    pub seq_number: u64,
    pub symbol: Symbol,
    pub update_type: UpdateType,
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to parse API secret from base64: {0}")]
    ApiSecretDecode(#[from] base64::DecodeError),
    #[error("Invalid length of API secret: {0}")]
    ApiSecretLength(#[from] crypto_common::InvalidLength),
}

/// Make user auth message for WebSocket
pub fn make_user_auth(
    api_secret: &str,
    api_key: &str,
    passphrase: &str,
) -> Result<KolliderMsg, AuthError> {
    let mut mac = HmacSha256::new_from_slice(&base64::decode(api_secret)?)?;
    let mut payload = vec![];
    let timestamp = format!("{}", Utc::now().timestamp());
    payload.extend(timestamp.as_bytes());
    payload.extend("authentication".bytes());
    trace!("HMAC payload: {}", std::str::from_utf8(&payload).unwrap());
    mac.update(&payload);
    let signature = base64::encode(mac.finalize().into_bytes());
    trace!("Signagure {}", signature);

    Ok(KolliderMsg::UserAuth {
        _type: AuthenticateTag::Tag,
        token: api_key.to_owned(),
        passphrase: passphrase.to_owned(),
        signature,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_orders_msg() {
        let data = r#"
        {
            "data": {
                "open_orders": {
                    "BTCUSD.PERP": [
                        {
                            "advanced_order_type": null,
                            "ext_order_id": "029893fe-dcd7-4c78-848c-ddf37468df94",
                            "filled": 0,
                            "leverage": 100,
                            "margin_type": "Isolated",
                            "order_id": 9951519,
                            "order_type": "Limit",
                            "price": 474500,
                            "quantity": 1,
                            "settlement_type": "Instant",
                            "side": "Ask",
                            "symbol": "BTCUSD.PERP",
                            "timestamp": 0,
                            "trigger_price_type": null,
                            "uid": 7051
                        }
                    ]
                }
            },
            "seq": 647,
            "type": "open_orders"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::OpenOrders {
                open_orders: hashmap! {
                    "BTCUSD.PERP".to_owned() => vec![
                        OpenOrder {
                            ext_order_id: "029893fe-dcd7-4c78-848c-ddf37468df94".to_owned(),
                            filled: 0,
                            leverage: 100,
                            margin_type: MarginType::Isolated,
                            order_id: 9951519,
                            order_type: OrderType::Limit,
                            price: 474500,
                            quantity: 1,
                            settlement_type: SettlementType::Instant,
                            side: OrderSide::Ask,
                            symbol: "BTCUSD.PERP".to_owned(),
                            timestamp: 0,
                            uid: 7051,
                        }
                    ]
                },
            }
        );
    }
}
