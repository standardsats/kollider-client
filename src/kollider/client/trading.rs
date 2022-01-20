use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::{
    FillDetails, OrderBody, OrderCreated, OrderDetails, OrderPrediction, PositionDetails, Symbol,
};
use chrono::prelude::*;
use std::collections::HashMap;

impl KolliderClient {
    pub async fn create_order(&self, body: &OrderBody) -> Result<OrderCreated> {
        self.post_request_auth("/orders", Some(body)).await
    }

    pub async fn order_prediction(&self, body: &OrderBody) -> Result<OrderPrediction> {
        self.post_request_auth("/orders/prediction", Some(body))
            .await
    }

    pub async fn orders(
        &self,
        symbol: &str,
        start: DateTime<Local>,
        end: DateTime<Local>,
        limit: usize,
    ) -> Result<Vec<OrderDetails>> {
        self.get_request_auth(
            "/orders",
            &[
                ("symbol", symbol.to_owned()),
                ("start", format!("{}", start.timestamp())),
                ("end", format!("{}", end.timestamp())),
                ("limit", format!("{}", limit)),
            ],
        )
        .await
    }

    pub async fn open_orders(&self) -> Result<HashMap<Symbol, Vec<OrderDetails>>> {
        self.get_request_auth_noargs("/orders/open").await
    }

    pub async fn fills(
        &self,
        symbol: &str,
        start: DateTime<Local>,
        end: DateTime<Local>,
        limit: usize,
    ) -> Result<Vec<FillDetails>> {
        self.get_request_auth(
            "/user/fills",
            &[
                ("symbol", symbol.to_owned()),
                ("start", format!("{}", start.timestamp())),
                ("end", format!("{}", end.timestamp())),
                ("limit", format!("{}", limit)),
            ],
        )
        .await
    }

    pub async fn positions(&self) -> Result<HashMap<Symbol, PositionDetails>> {
        self.get_request_auth_noargs("/positions").await
    }

    pub async fn cancel_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<HashMap<Symbol, PositionDetails>> {
        self.delete_request_auth(
            "/orders",
            &[
                ("symbol", symbol.to_owned()),
                ("order_id", order_id.to_owned()),
            ],
        )
        .await
    }
}
