use super::error::{Error, Result};
use crate::kollider::api::error::{KolliderError, KolliderResult};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use log::*;
use sha2::Sha256;
use thiserror::Error;

pub const KOLLIDER_MAINNET: &str = "https://api.kollider.xyz/v1/";
pub const KOLLIDER_TESTNET: &str = "https://test.api.kollider.xyz/v1/";

pub struct KolliderClient {
    pub client: reqwest::Client,
    pub server: String,
    pub auth: Option<KolliderAuth>,
}

impl KolliderClient {
    pub fn new() -> Self {
        KolliderClient::mainnet()
    }

    pub fn testnet() -> Self {
        KolliderClient {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            server: KOLLIDER_TESTNET.to_owned(),
            auth: None,
        }
    }

    pub fn mainnet() -> Self {
        KolliderClient {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            server: KOLLIDER_MAINNET.to_owned(),
            auth: None,
        }
    }

    /// Helper to query GET request with authentification headers
    pub async fn get_request_auth<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| Error::AuthRequired(path.to_owned()))?;
        let endpoint = format!("{}{}", self.server, path);
        let body: Option<()> = None;

        let request = auth
            .inject_auth("GET", path, body, self.client.get(endpoint))?
            .build()?;
        debug!("Requesting GET URL {}", request.url());
        let txt = self.client.execute(request).await?.text().await?;
        debug!("Got response body {}", txt);
        let raw_res: KolliderResult<T> = serde_json::from_str(&txt)?;
        let res: std::result::Result<T, KolliderError> = raw_res.into();
        Ok(res?)
    }

    /// Helper to query GET request without auth
    pub async fn get_request<T, Q>(&self, path: &str, query_args: &Q) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize + ?Sized,
    {
        let endpoint = format!("{}{}", self.server, path);

        let request = self.client.get(endpoint).query(query_args).build()?;

        debug!("Requesting GET URL {}", request.url());
        let txt = self.client.execute(request).await?.text().await?;
        debug!("Got response body {}", txt);
        let raw_res: KolliderResult<T> = serde_json::from_str(&txt)?;
        let res: std::result::Result<T, KolliderError> = raw_res.into();
        Ok(res?)
    }

    /// Helper to query GET request without auth
    pub async fn get_request_noargs<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let endpoint = format!("{}{}", self.server, path);

        let request = self.client.get(endpoint).build()?;

        debug!("Requesting GET URL {}", request.url());
        let txt = self.client.execute(request).await?.text().await?;
        debug!("Got response body {}", txt);
        let raw_res: KolliderResult<T> = serde_json::from_str(&txt)?;
        let res: std::result::Result<T, KolliderError> = raw_res.into();
        Ok(res?)
    }

    /// Helper to query GET request with authentification headers
    pub async fn post_request_auth<T, R>(&self, path: &str, body: Option<&T>) -> Result<R>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| Error::AuthRequired(path.to_owned()))?;
        let endpoint = format!("{}{}", self.server, path);

        let raw_request = if let Some(b) = body {
            self.client.post(endpoint).json(b)
        } else {
            self.client.post(endpoint)
        };
        let request = auth.inject_auth("POST", path, body, raw_request)?.build()?;
        if log_enabled!(Level::Debug) {
            let body = std::str::from_utf8(request.body().unwrap().as_bytes().unwrap()).unwrap();
            debug!("Requesting POST URL {} with body {}", request.url(), body);
        }
        let txt = self.client.execute(request).await?.text().await?;
        debug!("Got response body {}", txt);
        let raw_res: KolliderResult<R> = serde_json::from_str(&txt)?;
        let res: std::result::Result<R, KolliderError> = raw_res.into();
        Ok(res?)
    }
}

impl Default for KolliderClient {
    fn default() -> Self {
        KolliderClient::new()
    }
}

pub struct KolliderAuth {
    pub api_key: String,
    pub api_secret: Vec<u8>,
    pub password: String,
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("API secret invalid: {0}")]
    ApiSecretInvalid(#[from] crypto_common::InvalidLength),
    #[error("Cannot serialize body to sign: {0}")]
    BodySerialization(#[from] serde_json::Error),
}

impl KolliderAuth {
    pub fn new(
        api_key: &str,
        api_secret: &str,
        password: &str,
    ) -> std::result::Result<Self, base64::DecodeError> {
        Ok(KolliderAuth {
            api_key: api_key.to_owned(),
            api_secret: base64::decode(api_secret)?,
            password: password.to_owned(),
        })
    }

    pub fn inject_auth<T>(
        &self,
        method: &str,
        route: &str,
        mbody: Option<T>,
        request: reqwest::RequestBuilder,
    ) -> std::result::Result<reqwest::RequestBuilder, AuthError>
    where
        T: serde::Serialize,
    {
        let mut mac = HmacSha256::new_from_slice(&self.api_secret)?;
        let mut payload = vec![];
        let timestamp = format!("{}", Utc::now().timestamp());
        payload.extend(timestamp.as_bytes());
        payload.extend(method.bytes());
        payload.extend(route.bytes());
        if let Some(body) = mbody {
            let mut body_str = serde_json::to_string(&body)?;
            body_str.retain(|c| !c.is_whitespace());
            payload.extend(body_str.bytes());
        }
        mac.update(&payload);
        let signature = base64::encode(mac.finalize().into_bytes());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("K-API-KEY", self.api_key.parse().unwrap());
        headers.insert("K-SIGNATURE", signature.parse().unwrap());
        headers.insert("K-TIMESTAMP", timestamp.parse().unwrap());
        headers.insert("K-PASSPHRASE", self.password.parse().unwrap());

        Ok(request.headers(headers))
    }
}
