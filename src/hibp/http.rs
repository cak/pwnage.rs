use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest;
use reqwest::Response;

use super::core::HIBPError;

pub fn fetch(url: &str, hibp_api_key: &str, user_agent: &str) -> Result<Response, HIBPError> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("hibp-api-key", hibp_api_key)
        .header("user-agent", user_agent)
        .send()?;

    let status_code = res.status().as_u16();

    match status_code {
        200 => return Ok(res),
        400 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Bad request".to_string(),
            })
        }
        401 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Unauthorised".to_string(),
            })
        }
        403 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Forbidden".to_string(),
            })
        }
        404 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Not found".to_string(),
            })
        }
        429 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Too many requests".to_string(),
            })
        }
        503 => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Service unavailable".to_string(),
            })
        }
        _ => {
            return Err(HIBPError {
                kind: "HIBP".to_string(),
                message: "Unknown error".to_string(),
            })
        }
    }
}

pub fn percent_encode(raw: &str) -> String {
    utf8_percent_encode(raw, NON_ALPHANUMERIC).to_string()
}

pub fn fetch_unauth(url: &str, user_agent: &str) -> Result<Response, HIBPError> {
    let client = reqwest::Client::new();
    let res = client.get(url).header("user-agent", user_agent).send()?;
    Ok(res)
}
