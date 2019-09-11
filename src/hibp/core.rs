use reqwest;

pub const HIBP_BASE_URL: &str = "https://haveibeenpwned.com/api/v3";

pub struct HIBP {
    pub api_key: String,
    pub user_agent: String,
}

impl HIBP {
    pub fn new(api_key: String, user_agent: String) -> HIBP {
        HIBP {
            api_key,
            user_agent,
        }
    }
}

#[derive(Debug)]
pub struct HIBPError {
    pub kind: String,
    pub message: String,
}

impl From<reqwest::Error> for HIBPError {
    fn from(error: reqwest::Error) -> Self {
        HIBPError {
            kind: String::from("reqwest"),
            message: error.to_string(),
        }
    }
}
