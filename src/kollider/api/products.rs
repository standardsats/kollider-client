use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[cfg(feature = "openapi")]
use rweb::Schema;

/// Symbol type of the tickers and products. Ex: "BTCUSD.PERP" or ".XTBUSD"
pub type Symbol = String;

/// Response item of the /market/products
#[derive(Deserialize, Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct Product {
    pub symbol: Symbol,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub contract_size: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_leverage: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub base_margin: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub maintenance_margin: f64,
    pub is_inverse_priced: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub price_dp: f64,
    pub underlying_symbol: Symbol,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub last_price: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub tick_size: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub risk_limit: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_product_deserialize() {
        let data = r#"
        {
            "BTCUSD.PERP": {
              "symbol": "BTCUSD.PERP",
              "contract_size": "1",
              "max_leverage": "100.00",
              "base_margin": "0.00500",
              "maintenance_margin": "0.00400",
              "is_inverse_priced": true,
              "price_dp": "1",
              "underlying_symbol": ".XTBUSD",
              "last_price": "130713",
              "tick_size": "0.5",
              "risk_limit": "100000000"
            },
            "LTCUSD.PERP": {
              "symbol": "LTCUSD.PERP",
              "contract_size": "1",
              "max_leverage": "25.00",
              "base_margin": "0.00500",
              "maintenance_margin": "0.00400",
              "is_inverse_priced": false,
              "price_dp": "1",
              "underlying_symbol": ".LTCUSD",
              "last_price": "546",
              "tick_size": "0.5",
              "risk_limit": "100000000"
            }
        }"#;

        let v: HashMap<Symbol, Product> = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            hashmap! {
                "BTCUSD.PERP".to_owned() => Product {
                    symbol: "BTCUSD.PERP".to_owned(),
                    contract_size: 1.0,
                    max_leverage: 100.0,
                    base_margin: 0.005,
                    maintenance_margin: 0.004,
                    is_inverse_priced: true,
                    price_dp: 1.0,
                    underlying_symbol: ".XTBUSD".to_owned(),
                    last_price: 130713.0,
                    tick_size: 0.5,
                    risk_limit: 100000000.0,
                },
                "LTCUSD.PERP".to_owned() => Product {
                    symbol: "LTCUSD.PERP".to_owned(),
                    contract_size: 1.0,
                    max_leverage: 25.0,
                    base_margin: 0.005,
                    maintenance_margin: 0.004,
                    is_inverse_priced: false,
                    price_dp: 1.0,
                    underlying_symbol: ".LTCUSD".to_owned(),
                    last_price: 546.0,
                    tick_size: 0.5,
                    risk_limit: 100000000.0,
                },
            }
        );
    }
}
