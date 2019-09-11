use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use super::core::HIBPError;
use super::core::HIBP;
use super::core::HIBP_BASE_URL;
use super::http::{fetch, percent_encode};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Breach {
    pub name: String,
    pub title: String,
    pub domain: String,
    pub breach_date: NaiveDate,
    pub added_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
    pub pwn_count: u32,
    pub description: String,
    pub data_classes: Vec<String>,
    pub is_verified: bool,
    pub is_fabricated: bool,
    pub is_sensitive: bool,
    pub is_retired: bool,
    pub is_spam_list: bool,
    pub logo_path: String,
}

impl HIBP {
    pub fn breaches(self) -> Result<Vec<Breach>, HIBPError> {
        let url = format!("{}/breaches", HIBP_BASE_URL);
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<Breach> = res.json()?;
        Ok(breaches)
    }

    pub fn breach(self, name: &str) -> Result<Vec<Breach>, HIBPError> {
        let encoded_name = percent_encode(name);
        let url = format!("{}/breach/{}", HIBP_BASE_URL, encoded_name);
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<Breach> = res.json()?;
        Ok(breaches)
    }

    pub fn breaches_by_domain(self, domain: &str) -> Result<Vec<Breach>, HIBPError> {
        let encoded_domain = percent_encode(domain);
        let url = format!("{}/breaches?domain={}", HIBP_BASE_URL, encoded_domain);
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<Breach> = res.json()?;
        Ok(breaches)
    }
}
