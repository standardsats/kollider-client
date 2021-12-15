use crate::kollider::api::products::{Product, Symbol};
use std::collections::HashMap;
use super::env::KolliderClient;
use super::error::Result;

impl KolliderClient {
    /// GET endpoint `/market/api`
    pub async fn market_products(&self) -> Result<HashMap<Symbol, Product>> {
        let endpoint = format!("{}/{}", self.server, "market/products");
        let raw_res = self.client.get(endpoint).send().await?;
        // println!("{}", self.client.get(endpoint).send().await?.text().await?);
        Ok(raw_res.json().await?)
    }
}
