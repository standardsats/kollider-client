use super::env::KolliderClient;
use super::error::{Error, Result};
use crate::kollider::api::account::AccountInfo;
use crate::kollider::api::error::{KolliderError, KolliderResult};

impl KolliderClient {
    /// GET endpoint `/user/account`
    pub async fn user_account(&self) -> Result<AccountInfo> {
        let path = "/user/account";
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| Error::AuthRequired(path.to_owned()))?;
        let endpoint = format!("{}{}", self.server, path);
        let body : Option<()> = None;
        let build_request = || auth.inject_auth("GET", path, body, self.client.get(endpoint));

        // println!("{}", build_request.clone()()?.send().await?.text().await?);
        let raw_res: KolliderResult<AccountInfo> = build_request()?.send().await?.json().await?;
        let res: std::result::Result<AccountInfo, KolliderError> = raw_res.into();
        Ok(res?)
    }
}
