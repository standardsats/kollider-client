use super::super::products::Symbol;
use super::orderbook::OrderSide;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// Response item of the /market/ticker
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Ticker {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    best_ask: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    best_bid: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    last_price: f64,
    last_quantity: u64,
    last_side: OrderSide,
    symbol: Symbol,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_deserialize() {
        let data = r#"
        {
            "best_ask": "47949.5",
            "best_bid": "47938.5",
            "last_price": "46490.0",
            "last_quantity": 100,
            "last_side": "Bid",
            "symbol": "BTCUSD.PERP"
        }"#;

        let v: Ticker = serde_json::from_str(data).unwrap();

        assert_eq!(v, Ticker {
            best_ask: 47949.5,
            best_bid: 47938.5,
            last_price: 46490.0,
            last_quantity: 100,
            last_side: OrderSide::Bid,
            symbol: "BTCUSD.PERP".to_owned(),
        });
    }
}
