use chrono::prelude::*;
use crate::kollider::api::{OrderBody, OrderDetails, Symbol, FillDetails};
use std::collections::HashMap;
use super::env::KolliderClient;
use super::error::Result;

impl KolliderClient {
    pub async fn create_order(&self, body: &OrderBody) -> Result<()> {
        self.post_request_auth("/orders", Some(body)).await
    }

    pub async fn order_prediction(&self, body: &OrderBody) -> Result<()> {
        self.post_request_auth("/orders/prediction", Some(body)).await
    }

    pub async fn orders(&self, symbol: &str, start: DateTime<Local>, end: DateTime<Local>, limit: usize) -> Result<Vec<OrderDetails>> {
        self.get_request_auth("/orders", &[
            ("symbol", format!("{}", symbol)),
            ("start", format!("{}", start.timestamp())),
            ("end", format!("{}", end.timestamp())),
            ("limit", format!("{}", limit)),
        ]).await
    }

    pub async fn open_orders(&self) -> Result<HashMap<Symbol, Vec<OrderDetails>>> {
        self.get_request_auth_noargs("/orders/open").await
    }

    pub async fn fills(&self, symbol: &str, start: DateTime<Local>, end: DateTime<Local>, limit: usize) -> Result<Vec<FillDetails>> {
        self.get_request_auth("/user/fills", &[
            ("symbol", format!("{}", symbol)),
            ("start", format!("{}", start.timestamp())),
            ("end", format!("{}", end.timestamp())),
            ("limit", format!("{}", limit)),
        ]).await
    }
}