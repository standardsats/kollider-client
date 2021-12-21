use serde::{Serialize, Deserialize};
use std::{fmt, str::FromStr};
use super::products::Symbol;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum MarginType {
    Isolated,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownMarginType(String);

impl std::error::Error for UnknownMarginType {}

impl fmt::Display for UnknownMarginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given MarginType '{}' is unknown, valid are: Isolated",
            self.0
        )
    }
}

impl FromStr for MarginType {
    type Err = UnknownMarginType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "isolated" => Ok(MarginType::Isolated),
            _ => Err(UnknownMarginType(s.to_owned())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownOrderType(String);

impl std::error::Error for UnknownOrderType {}

impl fmt::Display for UnknownOrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given OrderType '{}' is unknown, valid are: Limit, Market",
            self.0
        )
    }
}

impl FromStr for OrderType {
    type Err = UnknownOrderType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "limit" => Ok(OrderType::Limit),
            "market" => Ok(OrderType::Market),
            _ => Err(UnknownOrderType(s.to_owned())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum SettlementType {
    Instant,
    Delayed,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownSettlementType(String);

impl std::error::Error for UnknownSettlementType {}

impl fmt::Display for UnknownSettlementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given SettlementType '{}' is unknown, valid are: Instant, Delayed",
            self.0
        )
    }
}

impl FromStr for SettlementType {
    type Err = UnknownSettlementType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "instant" => Ok(SettlementType::Instant),
            "delayed" => Ok(SettlementType::Delayed),
            _ => Err(UnknownSettlementType(s.to_owned())),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderSide {
    Ask,
    Bid,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownOrderSide(String);

impl std::error::Error for UnknownOrderSide {}

impl fmt::Display for UnknownOrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given OrderSide '{}' is unknown, valid are: Ask, Bid",
            self.0
        )
    }
}

impl FromStr for OrderSide {
    type Err = UnknownOrderSide;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "ask" => Ok(OrderSide::Ask),
            "bid" => Ok(OrderSide::Bid),
            _ => Err(UnknownOrderSide(s.to_owned())),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct OrderDetails {
    // pub advanced_order_type: String, //: null,
    pub ext_order_id: String,    //: "07e10e56-bd45-4e3c-9981-688e6af7fc69",
    pub filled: f64,             //: 0,
    pub leverage: u64,           //: 100,
    pub margin_type: MarginType, //: "Isolated",
    pub order_id: u64,           //: 9317213,
    pub order_type: OrderType,   //: "Limit",
    pub price: u64,              //: 486950,
    pub quantity: u64,           //: 1016,
    pub settlement_type: SettlementType, //: "Delayed",
    pub side: OrderSide,         //: "Ask",
    pub symbol: Symbol,          //: "BTCUSD.PERP",
    pub timestamp: u64,          //: 0,
    // pub trigger_price_type: String, //: null,
    pub uid: u64, //: 1
}
