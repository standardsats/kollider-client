use serde::{Deserialize, Serialize};
use super::super::{products::Symbol, order::{OrderSide, MarginType, OrderType, SettlementType}};

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

#[cfg(test)]
mod tests {
    use super::*;

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

}