use serde::{Deserialize, Serialize};
use super::super::{products::Symbol, order::{OrderSide, MarginType, OrderType, SettlementType}};
use serde_aux::field_attributes::deserialize_number_from_string;

/// Body of post /orders
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct OrderBody {
    pub price: u64,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub quantity: u64,
    pub symbol: Symbol,
    pub leverage: u64,
    pub margin_type: MarginType,
    pub settlement_type: SettlementType,
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct OrderPrediction {
    pub uid: u64,
    pub ext_id: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub margin_required: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub exchange_fee: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub estimated_liquidation_price: f64,
    pub rejection_reason: Option<String>,
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

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct OrderCreated {
    pub timestamp: u64,
    pub order_id: u64,
    pub ext_order_id: String,
    pub uid: u64,
    pub symbol: String,
    pub quantity: u64,
    pub order_type: OrderType,
    pub price: u64,
    pub leverage: u64,
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
            r#"{"price":100,"order_type":"Limit","side":"Bid","quantity":10,"symbol":"BTCUSD.PERP","leverage":100,"margin_type":"Isolated","settlement_type":"Delayed"}"#
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

    #[test]
    fn test_order_prediction() {
        let data = r#"
        {
            "uid": 7051,
            "ext_id": "8c8b8062-d14b-4b14-a05b-84c92a32f4d3",
            "margin_required": "2083.3333333333333333333300000",
            "value": "2083.3333333333333333333300000",
            "exchange_fee": "-0.5208333333333333333333325000",
            "estimated_liquidation_price": "240782.54326561324303988018849",
            "rejection_reason": "InstantLiquidation"
        }"#;

        let v: OrderPrediction = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            OrderPrediction {
                uid: 7051,
                ext_id: "8c8b8062-d14b-4b14-a05b-84c92a32f4d3".to_owned(),
                margin_required: 2083.3333333333335,
                value: 2083.3333333333335,
                exchange_fee: -0.5208333333333334,
                estimated_liquidation_price: 240782.54326561323,
                rejection_reason: Some("InstantLiquidation".to_owned()),
            }
        );
    }

    #[test]
    fn test_order_created() {
        let data = r#"
        {
            "timestamp": 1640100776001,
            "order_id": 9640692,
            "ext_order_id": "9eccb2e0-ff3d-4d72-a52d-00f662e325a9",
            "uid": 7051,
            "symbol": "BTCUSD.PERP",
            "quantity": 1,
            "order_type": "Limit",
            "price": 485155,
            "leverage": 1
        }"#;

        let v: OrderCreated = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            OrderCreated {
                timestamp: 1640100776001,
                order_id: 9640692,
                ext_order_id: "9eccb2e0-ff3d-4d72-a52d-00f662e325a9".to_owned(),
                uid: 7051,
                symbol: "BTCUSD.PERP".to_owned(),
                quantity: 1,
                order_type: OrderType::Limit,
                price: 485155,
                leverage: 1,
            }
        );
    }
}

