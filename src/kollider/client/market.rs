use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::market::{OrderBookResp, OrderBookLevel, Ticker};

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
}
