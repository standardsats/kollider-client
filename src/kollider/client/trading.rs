use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::trading::{OrderBody};

impl KolliderClient {
    pub async fn create_order(&self, body: &OrderBody) -> Result<()> {
        self.post_request_auth("/orders", Some(body)).await
    }
}