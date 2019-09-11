use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use super::core::HIBPError;
use super::core::HIBP;
use super::core::HIBP_BASE_URL;
use super::http::{fetch, percent_encode};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PasteAccount {
    pub source: String,
    pub id: String,
    pub title: String,
    pub date: NaiveDate,
    pub email_count: DateTime<Utc>,
}

impl HIBP {
    pub fn paste_account(self, account: &str) -> Result<Vec<PasteAccount>, HIBPError> {
        let encoded_account = percent_encode(account);
        let url = format!("{}/pasteaccount/{}", HIBP_BASE_URL, encoded_account);
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<PasteAccount> = res.json()?;
        Ok(breaches)
    }
}
