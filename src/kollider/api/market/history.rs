use super::super::products::Symbol;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "openapi")]
use rweb::Schema;

/// Time interval between points in time. 5m, 15m, 1h, 1d
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub enum IntervalSize {
    #[serde(rename = "5m")]
    FiveMin,
    #[serde(rename = "15m")]
    FifteenMin,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "1d")]
    OneDay,
}

impl fmt::Display for IntervalSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntervalSize::FiveMin => write!(f, "5m"),
            IntervalSize::FifteenMin => write!(f, "15m"),
            IntervalSize::OneHour => write!(f, "1h"),
            IntervalSize::OneDay => write!(f, "1d"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct UnknownInterval(String);

impl std::error::Error for UnknownInterval {}

impl fmt::Display for UnknownInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Given interval '{}' is unknown, valid are: 5m, 15m, 1h, 1d",
            self.0
        )
    }
}

impl FromStr for IntervalSize {
    type Err = UnknownInterval;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "5m" => Ok(IntervalSize::FiveMin),
            "15m" => Ok(IntervalSize::FifteenMin),
            "1h" => Ok(IntervalSize::OneHour),
            "1d" => Ok(IntervalSize::OneDay),
            _ => Err(UnknownInterval(s.to_owned())),
        }
    }
}

/// Response body of the /market/historic_index_prices
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct HistoryResp {
    data: Vec<HistoryItem>,
    symbol: Symbol,
}

/// Response item of the /market/historic_index_prices
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "openapi", derive(Schema))]
pub struct HistoryItem {
    max: Option<f64>,
    min: Option<f64>,
    mean: Option<f64>,
    time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_deserialize() {
        let data = r#"
        {"data":[{"max":null,"mean":null,"min":null,"time":1639603374}],"symbol":"BTCUSD.PERP"}
        "#;

        let v: HistoryResp = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            HistoryResp {
                data: vec![HistoryItem {
                    max: None,
                    min: None,
                    mean: None,
                    time: 1639603374,
                }],
                symbol: "BTCUSD.PERP".to_owned(),
            }
        );
    }
}
