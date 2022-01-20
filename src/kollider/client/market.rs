use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::market::{
    HistoryResp, IntervalSize, OrderBookLevel, OrderBookResp, Ticker,
};
use chrono::prelude::*;

impl KolliderClient {
    /// GET endpoint `/market/orderbook`
    pub async fn market_orderbook(
        &self,
        level: OrderBookLevel,
        symbol: &str,
    ) -> Result<OrderBookResp> {
        self.get_request(
            "/market/orderbook",
            &[
                ("level", format!("{:?}", level)),
                ("symbol", symbol.to_owned()),
            ],
        )
        .await
    }

    /// GET endpoint `/market/ticker`
    pub async fn market_ticker(&self, symbol: &str) -> Result<Ticker> {
        self.get_request("/market/ticker", &[("symbol", symbol)])
            .await
    }

    /// GET endpoint `/market/historic_index_prices`
    pub async fn market_historic_index_prices(
        &self,
        limit: usize,
        symbol: &str,
        start: DateTime<Local>,
        end: DateTime<Local>,
        interval_size: IntervalSize,
    ) -> Result<HistoryResp> {
        self.get_request(
            "/market/historic_index_prices",
            &[
                ("limit", format!("{}", limit)),
                ("symbol", symbol.to_owned()),
                ("start", format!("{}", start.timestamp())),
                ("end", format!("{}", end.timestamp())),
                ("interval_size", format!("{}", interval_size)),
            ],
        )
        .await
    }
}
