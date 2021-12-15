pub const KOLLIDER_MAINNET: &str = "https://api.kollider.xyz/v1/";
pub const KOLLIDER_TESTNET: &str = "https://test.api.kollider.xyz/v1/";

pub struct KolliderClient {
    pub client: reqwest::Client,
    pub server: String,
}

impl KolliderClient {
    pub fn new() -> Self {
        KolliderClient::mainnet()
    }

    pub fn testnet() -> Self {
        KolliderClient {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            server: KOLLIDER_TESTNET.to_owned(),
        }
    }

    pub fn mainnet() -> Self {
        KolliderClient {
            client: reqwest::ClientBuilder::new().build().unwrap(),
            server: KOLLIDER_MAINNET.to_owned(),
        }
    }
}

impl Default for KolliderClient {
    fn default() -> Self {
        KolliderClient::new()
    }
}