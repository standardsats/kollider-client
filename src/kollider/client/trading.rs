use super::env::KolliderClient;
use super::error::{Error, Result};
use crate::kollider::api::{
    FillDetails, OrderBody, OrderCreated, OrderDetails, OrderPrediction, PositionDetails, Symbol,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct CancelOrder {
    order_id: u64,
    symbol: String,
}

#[derive(Deserialize)]
struct CancelResult {
    reason: String,
}

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

    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<()> {
        let inner_result: CancelResult = self
            .delete_request_auth(
                "/orders",
                &CancelOrder {
                    order_id: order_id,
                    symbol: symbol.to_owned(),
                },
            )
            .await?;
        if inner_result.reason == "Cancel" {
            Ok(())
        } else {
            Err(Error::CancelOrder(
                order_id,
                symbol.to_owned(),
                inner_result.reason,
            ))
        }
    }
}
