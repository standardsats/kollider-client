use crate::kollider::api::products::{Product, Symbol};
use std::collections::HashMap;
use super::env::KolliderClient;
use super::error::Result;

impl KolliderClient {
    /// GET endpoint `/market/api`
    pub async fn market_products(&self) -> Result<HashMap<Symbol, Product>> {
        self.get_request_noargs("/market/products").await
    }
}
