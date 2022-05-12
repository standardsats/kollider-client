use super::env::KolliderClient;
use super::error::Result;
use crate::kollider::api::account::{
    AccountInfo, DepositBody, DepositBodyInner, DepositResp, WithdrawalBody, WithdrawalResp,
};

impl KolliderClient {
    /// GET endpoint `/user/account`
    pub async fn user_account(&self) -> Result<AccountInfo> {
        self.get_request_auth_noargs("/user/account").await
    }

    /// POST endpoint /wallet/deposit
    pub async fn wallet_deposit(&self, body: &DepositBody) -> Result<DepositResp> {
        let inner: DepositBodyInner = body.clone().into();
        self.post_request_auth("/wallet/deposit", Some(&inner)).await
    }

    /// POST endpoint /wallet/withdrawal
    pub async fn wallet_withdrawal(&self, body: &WithdrawalBody) -> Result<WithdrawalResp> {
        self.post_request_auth("/wallet/withdrawal", Some(body))
            .await
    }
}
