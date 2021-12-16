use serde::{Deserialize};

/// Response item of the /user/account
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct AccountInfo {
    created_at: AccountCreated,
    email: String,
    lnauth_enabled: bool,
    user_type: String,
    username: String,
    validated_email: bool,
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct AccountCreated {
    nanos_since_epoch: u64,
    secs_since_epoch: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lnuser_deserialize() {
        let data = r#"
        {
            "created_at": {
                "nanos_since_epoch": 513796000,
                "secs_since_epoch": 1639663512
            },
            "email": "28c27121-36d7-3ad5-87d6-8352b28e90a6-lnurl-auth@kollider.xyz",
            "lnauth_enabled": true,
            "user_type": "lnuser",
            "username": "28c27121-36d7-3ad5-87d6-8352b28e90a6",
            "validated_email": false
        }
        "#;

        let v: AccountInfo = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            AccountInfo {
                created_at: AccountCreated {
                    nanos_since_epoch: 513796000,
                    secs_since_epoch: 1639663512,
                },
                email: "28c27121-36d7-3ad5-87d6-8352b28e90a6-lnurl-auth@kollider.xyz".to_owned(),
                lnauth_enabled: true,
                user_type: "lnuser".to_owned(),
                username: "28c27121-36d7-3ad5-87d6-8352b28e90a6".to_owned(),
                validated_email: false,
             }
        );
    }
}
