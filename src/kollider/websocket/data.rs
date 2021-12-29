use crate::kollider::api::{Symbol, KeyPrice};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

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
    Tagged(KolliderTaggedMsg),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum SubscribeTag { #[serde(rename = "subscribe")] Tag }
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum UnsubscribeTag { #[serde(rename = "unsubscribe")] Tag }

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