use super::products::Symbol;
use serde::{
    de::{self, Deserializer},
    Deserialize,
};
use std::{collections::HashMap, str::FromStr};

/// Response item of the /market/orderbook
#[derive(Debug, PartialEq, Clone)]
pub struct OrderBookResp {
    pub level: OrderBookLevel,
    pub seq_number: u64,
    pub symbol: Symbol,
    pub book: OrderBook,
}

impl<'de> Deserialize<'de> for OrderBookResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;

        let seq_number: u64 = serde_json::from_value(
            json.get("seq_number")
                .ok_or_else(|| de::Error::missing_field("seq_number"))?
                .clone(),
        )
        .map_err(|e| de::Error::custom(e))?;

        let symbol: Symbol = serde_json::from_value(
            json.get("symbol")
                .ok_or_else(|| de::Error::missing_field("symbol"))?
                .clone(),
        )
        .map_err(|e| de::Error::custom(e))?;

        let level: OrderBookLevel = serde_json::from_value(
            json.get("level")
                .ok_or_else(|| de::Error::missing_field("level"))?
                .clone(),
        )
        .map_err(|e| de::Error::custom(e))?;

        let book_value = json
            .get("book")
            .ok_or_else(|| de::Error::missing_field("book"))?
            .clone();
        let book: OrderBook = match level {
            OrderBookLevel::Level2 => OrderBook::Level2(
                serde_json::from_value(book_value).map_err(|e| de::Error::custom(e))?,
            ),
            OrderBookLevel::Level3 => OrderBook::Level3(
                serde_json::from_value(book_value).map_err(|e| de::Error::custom(e))?,
            ),
        };

        Ok(OrderBookResp {
            level,
            seq_number,
            symbol,
            book,
        })
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderBookLevel {
    Level2,
    Level3,
}

impl OrderBookLevel {
    pub fn from_int(i: u64) -> Option<OrderBookLevel> {
        match i {
            2 => Some(OrderBookLevel::Level2),
            3 => Some(OrderBookLevel::Level3),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u64 {
        match self {
            OrderBookLevel::Level2 => 2,
            OrderBookLevel::Level3 => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrderBook {
    Level2(OrderBookLevel2),
    Level3(OrderBookLevel3),
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(try_from = "String")]
pub struct KeyPrice(u64);

impl std::convert::TryFrom<String> for KeyPrice {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(KeyPrice(u64::from_str(&value)?))
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OrderBookLevel2 {
    pub asks: HashMap<KeyPrice, u64>,
    pub bids: HashMap<KeyPrice, u64>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum MarginType {
    Isolated,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderType {
    Limit,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum SettlementType {
    Instant,
    Delayed,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum OrderSide {
    Ask,
    Bid,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct OrderBookLevel3 {
    pub asks: Vec<(u64, Vec<OrderDetails>)>,
    pub bids: Vec<(u64, Vec<OrderDetails>)>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_level_2() {
        let data = r#"
        {
            "book": {
                "asks": {
                    "486950": 1016,
                    "487195": 1071,
                    "487440": 1126,
                    "487680": 1181,
                    "487925": 1236,
                    "488170": 1291,
                    "488415": 1346,
                    "488725": 1161,
                    "499360": 120,
                    "515030": 9,
                    "517200": 207,
                    "517210": 371
                },
                "bids": {
                    "400000": 3,
                    "485140": 1401,
                    "485380": 1346,
                    "485625": 1291,
                    "485870": 1236,
                    "486110": 1181,
                    "486355": 1126,
                    "486595": 1071,
                    "486840": 1016
                }
            },
            "level": "Level2",
            "seq_number": 8411464,
            "symbol": "BTCUSD.PERP"
        }"#;

        let v: OrderBookResp = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            OrderBookResp {
                level: OrderBookLevel::Level2,
                seq_number: 8411464,
                symbol: "BTCUSD.PERP".to_owned(),
                book: OrderBook::Level2(OrderBookLevel2 {
                    asks: hashmap! {
                        KeyPrice(486950) => 1016,
                        KeyPrice(487195) => 1071,
                        KeyPrice(487440) => 1126,
                        KeyPrice(487680) => 1181,
                        KeyPrice(487925) => 1236,
                        KeyPrice(488170) => 1291,
                        KeyPrice(488415) => 1346,
                        KeyPrice(488725) => 1161,
                        KeyPrice(499360) => 120,
                        KeyPrice(515030) => 9,
                        KeyPrice(517200) => 207,
                        KeyPrice(517210) => 371
                    },
                    bids: hashmap! {
                        KeyPrice(400000) => 3,
                        KeyPrice(485140) => 1401,
                        KeyPrice(485380) => 1346,
                        KeyPrice(485625) => 1291,
                        KeyPrice(485870) => 1236,
                        KeyPrice(486110) => 1181,
                        KeyPrice(486355) => 1126,
                        KeyPrice(486595) => 1071,
                        KeyPrice(486840) => 1016
                    },
                }),
            }
        );
    }

    #[test]
    fn test_orderbook_level_3() {
        let data = r#"
        {
            "book": {
                "asks": [
                    [
                        486950,
                        [
                            {
                                "advanced_order_type": null,
                                "ext_order_id": "07e10e56-bd45-4e3c-9981-688e6af7fc69",
                                "filled": 0,
                                "leverage": 100,
                                "margin_type": "Isolated",
                                "order_id": 9317213,
                                "order_type": "Limit",
                                "price": 486950,
                                "quantity": 1016,
                                "settlement_type": "Delayed",
                                "side": "Ask",
                                "symbol": "BTCUSD.PERP",
                                "timestamp": 0,
                                "trigger_price_type": null,
                                "uid": 1
                            }
                        ]
                    ],
                    [
                        487195,
                        [
                            {
                                "advanced_order_type": null,
                                "ext_order_id": "07e10e56-bd45-4e3c-9981-688e6af7fc69",
                                "filled": 0,
                                "leverage": 100,
                                "margin_type": "Isolated",
                                "order_id": 9317212,
                                "order_type": "Limit",
                                "price": 487195,
                                "quantity": 1071,
                                "settlement_type": "Delayed",
                                "side": "Ask",
                                "symbol": "BTCUSD.PERP",
                                "timestamp": 0,
                                "trigger_price_type": null,
                                "uid": 1
                            }
                        ]
                    ]
                ],
                "bids": [
                    [
                        486840,
                        [
                            {
                                "advanced_order_type": null,
                                "ext_order_id": "07e10e56-bd45-4e3c-9981-688e6af7fc69",
                                "filled": 0,
                                "leverage": 100,
                                "margin_type": "Isolated",
                                "order_id": 9317209,
                                "order_type": "Limit",
                                "price": 486840,
                                "quantity": 1016,
                                "settlement_type": "Delayed",
                                "side": "Bid",
                                "symbol": "BTCUSD.PERP",
                                "timestamp": 0,
                                "trigger_price_type": null,
                                "uid": 1
                            }
                        ]
                    ],
                    [
                        486595,
                        [
                            {
                                "advanced_order_type": null,
                                "ext_order_id": "07e10e56-bd45-4e3c-9981-688e6af7fc69",
                                "filled": 0,
                                "leverage": 100,
                                "margin_type": "Isolated",
                                "order_id": 9317215,
                                "order_type": "Limit",
                                "price": 486595,
                                "quantity": 1071,
                                "settlement_type": "Delayed",
                                "side": "Bid",
                                "symbol": "BTCUSD.PERP",
                                "timestamp": 0,
                                "trigger_price_type": null,
                                "uid": 1
                            }
                        ]
                    ]
                ]
            },
            "level": "Level3",
            "seq_number": 8411404,
            "symbol": "BTCUSD.PERP"
        }"#;

        let v: OrderBookResp = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            OrderBookResp {
                level: OrderBookLevel::Level3,
                seq_number: 8411404,
                symbol: "BTCUSD.PERP".to_owned(),
                book: OrderBook::Level3(OrderBookLevel3 {
                    asks: vec![
                        (
                            486950,
                            vec![OrderDetails {
                                ext_order_id: "07e10e56-bd45-4e3c-9981-688e6af7fc69".to_owned(),
                                filled: 0.0,
                                leverage: 100,
                                margin_type: MarginType::Isolated,
                                order_id: 9317213,
                                order_type: OrderType::Limit,
                                price: 486950,
                                quantity: 1016,
                                settlement_type: SettlementType::Delayed,
                                side: OrderSide::Ask,
                                symbol: "BTCUSD.PERP".to_owned(),
                                timestamp: 0,
                                uid: 1
                            }]
                        ),
                        (
                            487195,
                            vec![OrderDetails {
                                ext_order_id: "07e10e56-bd45-4e3c-9981-688e6af7fc69".to_owned(),
                                filled: 0.0,
                                leverage: 100,
                                margin_type: MarginType::Isolated,
                                order_id: 9317212,
                                order_type: OrderType::Limit,
                                price: 487195,
                                quantity: 1071,
                                settlement_type: SettlementType::Delayed,
                                side: OrderSide::Ask,
                                symbol: "BTCUSD.PERP".to_owned(),
                                timestamp: 0,
                                uid: 1
                            }]
                        )
                    ],
                    bids: vec![
                        (
                            486840,
                            vec![OrderDetails {
                                ext_order_id: "07e10e56-bd45-4e3c-9981-688e6af7fc69".to_owned(),
                                filled: 0.0,
                                leverage: 100,
                                margin_type: MarginType::Isolated,
                                order_id: 9317209,
                                order_type: OrderType::Limit,
                                price: 486840,
                                quantity: 1016,
                                settlement_type: SettlementType::Delayed,
                                side: OrderSide::Bid,
                                symbol: "BTCUSD.PERP".to_owned(),
                                timestamp: 0,
                                uid: 1
                            }]
                        ),
                        (
                            486595,
                            vec![OrderDetails {
                                ext_order_id: "07e10e56-bd45-4e3c-9981-688e6af7fc69".to_owned(),
                                filled: 0.0,
                                leverage: 100,
                                margin_type: MarginType::Isolated,
                                order_id: 9317215,
                                order_type: OrderType::Limit,
                                price: 486595,
                                quantity: 1071,
                                settlement_type: SettlementType::Delayed,
                                side: OrderSide::Bid,
                                symbol: "BTCUSD.PERP".to_owned(),
                                timestamp: 0,
                                uid: 1
                            }]
                        )
                    ],
                }),
            }
        );
    }
}
