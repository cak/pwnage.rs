use reqwest::Response;
use sha1::{Digest, Sha1};

use super::core::HIBPError;
use super::core::HIBP;
use super::http::{fetch, fetch_unauth};
use std::str::FromStr;

const PWNED_PASSWORD_URL: &str = "https://api.pwnedpasswords.com/range";

pub struct PwnedPassword {
    pub pwned: bool,
    pub times: u32,
}

struct HashedPassword {
    prefix: String,
    suffix: String,
}

pub fn pwned_passwords(password: &str, user_agent: &str) -> Result<PwnedPassword, HIBPError> {
    let hashed_password = hash(password);
    let url = format!("{}/{}", PWNED_PASSWORD_URL, hashed_password.prefix);
    let res = fetch_unauth(&url, user_agent)?;
    check_password_hash(res, hashed_password)
}

fn hash(raw: &str) -> HashedPassword {
    let mut hasher = Sha1::new();
    hasher.input(raw);
    let result = hasher.result();
    let hash = format!("{:x}", result);
    let prefix = hash[0..5].to_owned();
    let suffix = hash[5..].to_owned();
    HashedPassword { prefix, suffix }
}

fn check_password_hash(
    mut res: Response,
    hashed_password: HashedPassword,
) -> Result<PwnedPassword, HIBPError> {
    let res_text = res.text()?;
    for hash in res_text.lines() {
        let hash_vec = hash.split(':').collect::<Vec<&str>>();
        if hash_vec[0] == hashed_password.suffix.to_uppercase() {
            let times = u32::from_str(hash_vec[1]).unwrap_or(0);
            return Ok(PwnedPassword { pwned: true, times });
        }
    }

    Ok(PwnedPassword {
        pwned: false,
        times: 0,
    })
}

impl HIBP {
    pub fn pwned_passwords(self, password: &str) -> Result<PwnedPassword, HIBPError> {
        let hashed_password = hash(password);
        let url = format!("{}/{}", PWNED_PASSWORD_URL, hashed_password.prefix);
        let res = fetch(&url, &self.api_key, &self.user_agent)?;
        check_password_hash(res, hashed_password)
    }
}
