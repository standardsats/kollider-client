use serde::{Deserialize, Serialize};

/// Request body for the /wallet/deposit
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(tag = "type", content = "amount")]
pub enum DepositBody {
    #[serde(rename = "Ln")]
    Lighting(u64),
    #[serde(rename = "BTC")]
    Bitcoin,
}

/// Response item of the /wallet/deposit
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum DepositResp {
    Lightning {
        payment_request: String,
    },
    Bitcoin {
        receive_address: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_body_ln() {
        let data = DepositBody::Lighting(142);

        let v: String = serde_json::to_string(&data).unwrap();

        assert_eq!(
            v,
            r#"{"type":"Ln","amount":142}"#
        );
    }

    #[test]
    fn test_deposit_body_btc() {
        let data = DepositBody::Bitcoin;

        let v: String = serde_json::to_string(&data).unwrap();

        assert_eq!(
            v,
            r#"{"type":"BTC"}"#
        );
    }

    #[test]
    fn test_deposit_resp_ln() {
        let data = r#"
        {
            "payment_request": "lntb1u1pwz5w78pp5e8w8cr5c30xzws92v36sk45znhjn098rtc4pea6ertnmvu25ng3sdpywd6hyetyvf5hgueqv3jk6meqd9h8vmmfvdjsxqrrssy29mzkzjfq27u67evzu893heqex737dhcapvcuantkztg6pnk77nrm72y7z0rs47wzc09vcnugk2ve6sr2ewvcrtqnh3yttv847qqvqpvv398"
        }
        "#;

        let v: DepositResp = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            DepositResp::Lightning {
                payment_request: "lntb1u1pwz5w78pp5e8w8cr5c30xzws92v36sk45znhjn098rtc4pea6ertnmvu25ng3sdpywd6hyetyvf5hgueqv3jk6meqd9h8vmmfvdjsxqrrssy29mzkzjfq27u67evzu893heqex737dhcapvcuantkztg6pnk77nrm72y7z0rs47wzc09vcnugk2ve6sr2ewvcrtqnh3yttv847qqvqpvv398".to_owned(),
             }
        );
    }

    #[test]
    fn test_deposit_resp_btc() {
        let data = r#"
        {
            "receive_address": "bc1qhwqkf2emlvng5p2c5pvm8py0lfjjkk7atmhfk0"
        }
        "#;

        let v: DepositResp = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            DepositResp::Bitcoin {
                receive_address: "bc1qhwqkf2emlvng5p2c5pvm8py0lfjjkk7atmhfk0".to_owned(),
             }
        );
    }
}
