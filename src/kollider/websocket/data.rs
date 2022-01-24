use crate::kollider::api::{MarginType, OrderSide, OrderType, SettlementType, Symbol};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use log::*;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};
use sha2::Sha256;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[cfg(feature = "openapi")]
use rweb::Schema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum SubscribeTag {
    #[serde(rename = "subscribe")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum UnsubscribeTag {
    #[serde(rename = "unsubscribe")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum AuthenticateTag {
    #[serde(rename = "authenticate")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum OrderTag {
    #[serde(rename = "order")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum CancelOrderTag {
    #[serde(rename = "cancel_order")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum FetchOpenOrdersTag {
    #[serde(rename = "fetch_open_orders")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum FetchPositionsTag {
    #[serde(rename = "fetch_positions")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum GetTickerTag {
    #[serde(rename = "get_ticker")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum TradableProductsTag {
    #[serde(rename = "fetch_tradable_products")]
    Tag,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum ErrorTag {
    #[serde(rename = "error")]
    Tag,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct WrappedPrice(#[serde(deserialize_with = "deserialize_number_from_string")] pub f64);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
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
    OrderNotFound { order_id: u64, symbol: Symbol },
    Fill {
        ext_order_id: String,
        is_maker: bool,
        is_selftrade: bool,
        leverage: u64,
        margin_type: MarginType,
        order_id: u64,
        partial: bool,
        price: u64,
        quantity: u64,
        side: OrderSide,
        symbol: String,
        user_id: u64,
    },
    #[serde(rename = "trade")]
    Trade {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        fees: f64,
        is_liquidation: bool,
        is_maker: bool,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        leverage: f64,
        margin_type: MarginType,
        order_id: u64,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        price: f64,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        quantity: u64,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        rpnl: f64,
        settlement_type: SettlementType,
        side: OrderSide,
        symbol: String,
        timestamp: u64,
    },
    #[serde(rename = "settlement_request")]
    SettlementRequest {
        #[serde(deserialize_with = "deserialize_number_from_string")]
        amount: u64,
        lnurl: String,
        request_id: String,
        side: OrderSide,
        symbol: Symbol,
    },
    #[serde(rename = "change_leverage_info")]
    ChangeLeverageInfo {
        error: Option<String>,
        #[serde(deserialize_with = "deserialize_option_number_from_string")]
        liquidation_price: Option<f64>,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        order_margin: f64,
        #[serde(deserialize_with = "deserialize_number_from_string")]
        position_margin: f64,
        symbol: Symbol,
    },
    #[serde(rename = "change_leverage_success")]
    ChangeLeverageSuccess { symbol: Symbol },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct OpenOrder {
    // "advanced_order_type": null,
    pub ext_order_id: String,
    pub filled: u64,
    pub leverage: u64,
    pub margin_type: MarginType,
    pub order_id: u64,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: u64,
    pub settlement_type: SettlementType,
    pub side: OrderSide,
    pub symbol: Symbol,
    pub timestamp: u64,
    // "trigger_price_type": null,
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct Position {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub adl_score: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bankruptcy_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub entry_price: f64,
    pub entry_time: Option<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub entry_value: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub funding: f64,
    pub is_liquidating: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub liq_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub mark_value: f64,
    pub open_order_ids: Vec<u64>,
    pub position_id: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub quantity: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub real_leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub rpnl: f64,
    pub side: Option<OrderSide>,
    pub symbol: Symbol,
    pub timestamp: u64,
    pub uid: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub upnl: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
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
pub struct UnknownChannelName(pub String);

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
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct IndexValue {
    pub denom: String,
    pub symbol: Symbol,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum UpdateType {
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "snapshot")]
    Snapshot,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct OrderBookLevel2 {
    pub asks: HashMap<String, u64>,
    pub bids: HashMap<String, u64>,
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

    #[test]
    fn test_fill_msg() {
        let data = r#"
        {
            "data": {
                "ext_order_id": "200f3530-473f-4ee6-8b56-c194c4ec8776",
                "is_maker": false,
                "is_selftrade": false,
                "leverage": 100,
                "margin_type": "Isolated",
                "order_id": 14792108,
                "partial": false,
                "price": 419005,
                "quantity": 1,
                "side": "Bid",
                "symbol": "BTCUSD.PERP",
                "user_id": 7051
            },
            "seq": 10379,
            "type": "fill"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::Fill {
                ext_order_id: "200f3530-473f-4ee6-8b56-c194c4ec8776".to_owned(),
                is_maker: false,
                is_selftrade: false,
                leverage: 100,
                margin_type: MarginType::Isolated,
                order_id: 14792108,
                partial: false,
                price: 419005,
                quantity: 1,
                side: OrderSide::Bid,
                symbol: "BTCUSD.PERP".to_owned(),
                user_id: 7051,
            }
        );
    }

    #[test]
    fn test_trade_msg() {
        let data = r#"
        {
            "data": {
                "fees": "1.7899547738093817496211250000",
                "is_liquidation": false,
                "is_maker": false,
                "leverage": "1.00",
                "margin_type": "Isolated",
                "order_id": 14792108,
                "order_type": "Market",
                "price": "41900.5",
                "quantity": "1",
                "rpnl": "0",
                "settlement_type": "Delayed",
                "side": "Bid",
                "symbol": "BTCUSD.PERP",
                "timestamp": 1642633795546
            },
            "seq": 10380,
            "type": "trade"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::Trade {
                fees: 1.7899547738093817,
                is_liquidation: false,
                is_maker: false,
                leverage: 1.00,
                margin_type: MarginType::Isolated,
                order_id: 14792108,
                price: 41900.5,
                quantity: 1,
                rpnl: 0.0,
                settlement_type: SettlementType::Delayed,
                side: OrderSide::Bid,
                symbol: "BTCUSD.PERP".to_owned(),
                timestamp: 1642633795546,
            }
        );
    }

    #[test]
    fn test_settlement_request_msg() {
        let data = r#"
        {
            "data": {
                "amount": "4766",
                "lnurl": "lnurl1dp68gurn8ghj7ctsdyhxkmmvd35kgetj9eu8j730wccj7mrfva58gmnfdenj7amfw35xgunpwaskchmjv4ch2etnwslhz0txxv6x2vfjvycz6d3ex5uj6dpkvv6j6c34vsuz6dfnx43rqvf5vgmrgvrpyqsrm9",
                "request_id": "f34e12a0-6959-46c5-b5d8-535b014b640a",
                "side": "Ask",
                "symbol": "BTCUSD.PERP"
            },
            "seq": 11053,
            "type": "settlement_request"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::SettlementRequest {
                amount: 4766,
                lnurl: "lnurl1dp68gurn8ghj7ctsdyhxkmmvd35kgetj9eu8j730wccj7mrfva58gmnfdenj7amfw35xgunpwaskchmjv4ch2etnwslhz0txxv6x2vfjvycz6d3ex5uj6dpkvv6j6c34vsuz6dfnx43rqvf5vgmrgvrpyqsrm9".to_owned(),
                request_id: "f34e12a0-6959-46c5-b5d8-535b014b640a".to_owned(),
                side: OrderSide::Ask,
                symbol: "BTCUSD.PERP".to_owned(),
            }
        );
    }

    #[test]
    fn test_change_leverage_info_msg_01() {
        let data = r#"
        {
            "data": {
                "error": null,
                "liquidation_price": null,
                "order_margin": "1000.000",
                "position_margin": "0",
                "symbol": "BTCUSD.PERP"
            },
            "seq": 12441,
            "type": "change_leverage_info"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::ChangeLeverageInfo {
                error: None,
                liquidation_price: None,
                order_margin: 1000.000,
                position_margin: 0.0,
                symbol: "BTCUSD.PERP".to_owned(),
            }
        );
    }

    #[test]
    fn test_change_leverage_success_msg() {
        let data = r#"
        {
            "data": {
                "symbol": "BTCUSD.PERP"
            },
            "seq": 12523,
            "type": "change_leverage_success"
        }
        "#;

        let v: KolliderTaggedMsg = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderTaggedMsg::ChangeLeverageSuccess {
                symbol: "BTCUSD.PERP".to_owned(),
            }
        );
    }
}
