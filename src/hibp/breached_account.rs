use serde::Deserialize;

use super::breaches::Breach;
use super::core::HIBPError;
use super::core::HIBP;
use super::http::{fetch, percent_encode};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BreachedAccount {
    pub name: String,
}

impl HIBP {
    pub fn breached_account(self, account: &str) -> Result<Vec<BreachedAccount>, HIBPError> {
        let encoded_account = percent_encode(account);
        let url = format!(
            "https://haveibeenpwned.com/api/v3/breachedaccount/{}",
            encoded_account
        );
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<BreachedAccount> = res.json()?;
        Ok(breaches)
    }

    pub fn breached_account_full(self, account: &str) -> Result<Vec<Breach>, HIBPError> {
        let encoded_account = percent_encode(account);
        let url = format!(
            "https://haveibeenpwned.com/api/v3/breachedaccount/{}?truncateResponse=false",
            encoded_account
        );
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<Breach> = res.json()?;
        Ok(breaches)
    }

    pub fn breached_account_by_domain(
        self,
        account: &str,
        domain: &str,
    ) -> Result<Vec<BreachedAccount>, HIBPError> {
        let encoded_account = percent_encode(account);
        let encoded_domain = percent_encode(domain);
        let url = format!(
            "https://haveibeenpwned.com/api/v3/breachedaccount/{}?domain={}",
            encoded_account, encoded_domain
        );
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<BreachedAccount> = res.json()?;
        Ok(breaches)
    }

    pub fn breached_account_by_domain_full(
        self,
        account: &str,
        domain: &str,
    ) -> Result<Vec<Breach>, HIBPError> {
        let encoded_account = percent_encode(account);
        let encoded_domain = percent_encode(domain);
        let url = format!(
            "https://haveibeenpwned.com/api/v3/breachedaccount/{}?domain={}&truncateResponse=false",
            encoded_account, encoded_domain
        );
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<Breach> = res.json()?;
        Ok(breaches)
    }
}
