use serde::Deserialize;
use std::fmt;

/// Response item of the /market/products
#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct KolliderError {
    error: ErrorType,
    msg: String,
}

impl std::error::Error for KolliderError {}

impl fmt::Display for KolliderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            ErrorType::Simple(err) => write!(
                f,
                "Kollider error {}: {}",
                err, self.msg
            ),
            ErrorType::Detailed{general_error} => write!(
                f,
                "Kollider general error {}: {}",
                general_error, self.msg
            ),
        }

    }
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum ErrorType {
    Simple(String),
    Detailed {
        #[serde(rename = "GeneralError")]
        general_error: String,
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
    fn test_simple_error_deserialize() {
        let data = r#"
        {"error":"InvalidKey","msg":"Your API key is invalid."}
        "#;

        let v: KolliderError = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderError {
                error: ErrorType::Simple("InvalidKey".to_owned()),
                msg: "Your API key is invalid.".to_owned()
            }
        );
    }

    #[test]
    fn test_general_error_deserialize() {
        let data = r#"
        {"error": { "GeneralError": "Unauthorized" },"msg":"A general error has occured."}
        "#;

        let v: KolliderError = serde_json::from_str(data).unwrap();

        assert_eq!(
            v,
            KolliderError {
                error: ErrorType::Detailed { general_error: "Unauthorized".to_owned() },
                msg: "A general error has occured.".to_owned()
            }
        );
    }
}
