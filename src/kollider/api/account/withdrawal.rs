use serde::{Serialize};

/// Single value tag for tagging only one possible type of "Ln"
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum LnTag {
    Ln
}

/// Single value tag for tagging only one possible type of "BTC"
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum BtcTag {
    BTC
}

/// Request body for the /wallet/withdrawal
#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum WithdrawalBody {
    Lighting {
        #[serde(rename = "type")]
        _type: LnTag,
        payment_request: String,
        amount: u64,
    },
    Bitcoin {
        #[serde(rename = "type")]
        _type: BtcTag,
        receive_address: String,
        amount: u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_body_ln() {
        let data = WithdrawalBody::Lighting {
            _type: LnTag::Ln,
            payment_request: "lntb1u1pwz5w78pp5e8w8cr5c30xzws92v36sk45znhjn098rtc4pea6ertnmvu25ng3sdpywd6hyetyvf5hgueqv3jk6meqd9h8vmmfvdjsxqrrssy29mzkzjfq27u67evzu893heqex737dhcapvcuantkztg6pnk77nrm72y7z0rs47wzc09vcnugk2ve6sr2ewvcrtqnh3yttv847qqvqpvv398".to_owned(),
            amount: 100,
        };

        let v: String = serde_json::to_string(&data).unwrap();

        assert_eq!(
            v,
            r#"{"type":"Ln","payment_request":"lntb1u1pwz5w78pp5e8w8cr5c30xzws92v36sk45znhjn098rtc4pea6ertnmvu25ng3sdpywd6hyetyvf5hgueqv3jk6meqd9h8vmmfvdjsxqrrssy29mzkzjfq27u67evzu893heqex737dhcapvcuantkztg6pnk77nrm72y7z0rs47wzc09vcnugk2ve6sr2ewvcrtqnh3yttv847qqvqpvv398","amount":100}"#
        );
    }

    #[test]
    fn test_withdrawal_body_btc() {
        let data = WithdrawalBody::Bitcoin {
            _type: BtcTag::BTC,
            receive_address: "bc1qhwqkf2emlvng5p2c5pvm8py0lfjjkk7atmhfk0".to_owned(),
            amount: 100,
        };

        let v: String = serde_json::to_string(&data).unwrap();

        assert_eq!(
            v,
            r#"{"type":"BTC","receive_address":"bc1qhwqkf2emlvng5p2c5pvm8py0lfjjkk7atmhfk0","amount":100}"#
        );
    }
}