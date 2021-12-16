use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::market::{OrderBookResp, OrderBookLevel, Ticker, HistoryResp, IntervalSize};
use chrono::prelude::*;

impl KolliderClient {
    /// GET endpoint `/market/orderbook`
    pub async fn market_orderbook(&self, level: OrderBookLevel, symbol: &str) -> Result<OrderBookResp> {
        let endpoint = format!("{}/{}", self.server, "market/orderbook");
        let build_request = || self
            .client
            .get(endpoint)
            .query(&[
                ("level", format!("{:?}", level)),
                ("symbol", format!("{}", symbol)),
            ]);

        // println!("{}", build_request.clone()().send().await?.text().await?);
        let raw_res = build_request()
            .send()
            .await?;
        Ok(raw_res.json().await?)
    }

    /// GET endpoint `/market/ticker`
    pub async fn market_ticker(&self, symbol: &str) -> Result<Ticker> {
        let endpoint = format!("{}/{}", self.server, "market/ticker");
        let build_request = || self
            .client
            .get(endpoint)
            .query(&[
                ("symbol", format!("{}", symbol)),
            ]);

        // println!("{}", build_request.clone()().send().await?.text().await?);
        let raw_res = build_request()
            .send()
            .await?;
        Ok(raw_res.json().await?)
    }

    /// GET endpoint `/market/historic_index_prices`
    pub async fn market_historic_index_prices(&self, limit: usize, symbol: &str, start: DateTime<Local>, end: DateTime<Local>, interval_size: IntervalSize) -> Result<HistoryResp> {
        let endpoint = format!("{}/{}", self.server, "market/historic_index_prices");
        let build_request = || self
            .client
            .get(endpoint)
            .query(&[
                ("limit", format!("{}", limit)),
                ("symbol", format!("{}", symbol)),
                ("start", format!("{}", start.timestamp())),
                ("end", format!("{}", end.timestamp())),
                ("interval_size", format!("{}", interval_size)),
            ]);
        println!("URL: {}", build_request.clone()().build().unwrap().url());
        println!("{}", build_request.clone()().send().await?.text().await?);
        let raw_res = build_request()
            .send()
            .await?;
        Ok(raw_res.json().await?)
    }
}
