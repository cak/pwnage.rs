use super::core::HIBPError;
use super::core::HIBP;
use super::core::HIBP_BASE_URL;
use super::http::fetch;

impl HIBP {
    pub fn data_classes(self) -> Result<Vec<String>, HIBPError> {
        let url = format!("{}/dataclasses", HIBP_BASE_URL);
        let mut res = fetch(&url, &self.api_key, &self.user_agent)?;
        let breaches: Vec<String> = res.json()?;
        Ok(breaches)
    }
}
