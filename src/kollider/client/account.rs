use super::env::KolliderClient;
use super::error::{Error, Result};
use crate::kollider::api::account::{AccountInfo, DepositBody, DepositResp, WithdrawalBody};
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
        let body: Option<()> = None;
        let build_request = || auth.inject_auth("GET", path, body, self.client.get(endpoint));

        // println!("{}", build_request.clone()()?.send().await?.text().await?);
        let raw_res: KolliderResult<AccountInfo> = build_request()?.send().await?.json().await?;
        let res: std::result::Result<AccountInfo, KolliderError> = raw_res.into();
        Ok(res?)
    }

    pub async fn wallet_deposit(&self, body: &DepositBody) -> Result<DepositResp> {
        let path = "/wallet/deposit";
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| Error::AuthRequired(path.to_owned()))?;
        let endpoint = format!("{}{}", self.server, path);
        let build_request = || {
            auth.inject_auth(
                "POST",
                path,
                Some(body),
                self.client.post(endpoint).json(body),
            )
        };

        // println!("URL: {}", build_request.clone()()?.build().unwrap().url());
        // println!(
        //     "Body: {}",
        //     std::str::from_utf8(
        //         build_request.clone()()?
        //             .build()
        //             .unwrap()
        //             .body()
        //             .unwrap()
        //             .as_bytes()
        //             .unwrap()
        //     )
        //     .unwrap()
        // );
        // println!("{}", build_request.clone()()?.send().await?.text().await?);
        let raw_res: KolliderResult<DepositResp> = build_request()?.send().await?.json().await?;
        let res: std::result::Result<DepositResp, KolliderError> = raw_res.into();
        Ok(res?)
    }

    pub async fn wallet_withdrawal(&self, body: &WithdrawalBody) -> Result<()> {
        let path = "/wallet/withdrawal";
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| Error::AuthRequired(path.to_owned()))?;
        let endpoint = format!("{}{}", self.server, path);
        let build_request = || {
            auth.inject_auth(
                "POST",
                path,
                Some(body),
                self.client.post(endpoint).json(body),
            )
        };

        // println!("URL: {}", build_request.clone()()?.build().unwrap().url());
        println!(
            "Body: {}",
            std::str::from_utf8(
                build_request.clone()()?
                    .build()
                    .unwrap()
                    .body()
                    .unwrap()
                    .as_bytes()
                    .unwrap()
            )
            .unwrap()
        );
        println!("{}", build_request.clone()()?.send().await?.text().await?);
        let raw_res: KolliderResult<()> = build_request()?.send().await?.json().await?;
        let res: std::result::Result<(), KolliderError> = raw_res.into();
        Ok(res?)
    }
}
