use serde::{Deserialize, Serialize};
use super::super::{products::Symbol, order::{OrderSide, MarginType, OrderType, SettlementType}};
use serde_aux::field_attributes::deserialize_number_from_string;

/// Body of post /orders
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct OrderBody {
    pub symbol: Symbol,
    pub quantity: u64,
    pub leverage: u64,
    pub side: OrderSide,
    pub margin_type: MarginType,
    pub order_type: OrderType,
    pub settlement_type: SettlementType,
    pub price: u64,
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct PositionDetails {
    pub uid: u64,
    pub timestamp: u64,
    pub symbol: Symbol,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub upnl: i64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub entry_price: f64,
    pub side: OrderSide,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub quantity: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub liq_price: f64,
    pub open_order_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_order_body() {
        let data = OrderBody {
            symbol: "BTCUSD.PERP".to_owned(),
            quantity: 10,
            leverage: 100,
            side: OrderSide::Bid,
            margin_type: MarginType::Isolated,
            order_type: OrderType::Limit,
            settlement_type: SettlementType::Delayed,
            price: 100,
        };

        let v: String = serde_json::to_string(&data).unwrap();

        assert_eq!(
            v,
            r#"{"symbol":"BTCUSD.PERP","quantity":10,"leverage":100,"side":"Bid","margin_type":"Isolated","order_type":"Limit","settlement_type":"Delayed","price":100}"#
        );
    }

    #[test]
    fn test_position_details() {
        let data = r#"
        {
            "BTCUSD.PERP": {
              "uid": 11,
              "timestamp": 1604332066202,
              "symbol": "BTCUSD.PERP",
              "upnl": "-6",
              "leverage": "1.00",
              "entry_price": "13534.0",
              "side": "Bid",
              "quantity": "1",
              "liq_price": "6788.3",
              "open_order_ids": []
            }
        }"#;

        let v: HashMap<Symbol, PositionDetails> = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            hashmap! {
                "BTCUSD.PERP".to_owned() => PositionDetails {
                    uid: 11,
                    timestamp: 1604332066202,
                    symbol: "BTCUSD.PERP".to_owned(),
                    upnl: -6,
                    leverage: 1.0,
                    entry_price: 13534.0,
                    side: OrderSide::Bid,
                    quantity: 1.0,
                    liq_price: 6788.3,
                    open_order_ids: vec![],
                }
            }
        );
    }
}

