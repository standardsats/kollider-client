use serde::{Serialize, Deserialize};
use crate::kollider::api::Symbol;
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum KolliderMsg {
    Subscribe(SubscribeMsg),
    Unknown(serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SubscribeMsg {
    #[serde(rename = "type")]
    pub _type: SubscribeType,
    pub symbols: Vec<Symbol>,
    pub channels: Vec<ChannelName>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum SubscribeType {
    #[serde(rename = "subscribe")]
    Subscribe,
    #[serde(rename = "unsubscribe")]
    Unsubscribe,
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