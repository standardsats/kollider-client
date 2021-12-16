use serde::Deserialize;
use std::fmt;

/// Response item of the /market/products
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct KolliderError {
    error: String,
    msg: String,
}

impl std::error::Error for KolliderError {}

impl fmt::Display for KolliderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Kollider error {}: {}",
            self.error, self.msg
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum KolliderResult<T> {
    Err(KolliderError),
    Ok(T),
}

impl<T> Into<Result<T, KolliderError>> for KolliderResult<T> {
    fn into(self) -> Result<T, KolliderError> {
        match self {
            KolliderResult::Err(e) => Err(e),
            KolliderResult::Ok(v) => Ok(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_deserialize() {
        let data = r#"
        {"error":"InvalidKey","msg":"Your API key is invalid."}
        "#;

        let v: KolliderError = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderError {
                error: "InvalidKey".to_owned(),
                msg: "Your API key is invalid.".to_owned()
            }
        );
    }
}
