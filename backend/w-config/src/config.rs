use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub gpt_api_key: String,
    pub amazon_affiliate: AmazonAffiliateConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AmazonAffiliateConfig {
    pub access_key: String,
    pub secret: String,
}

pub fn load_config() -> Result<Config, config_file::ConfigFileError> {
    Config::from_config_file("../config.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() -> Result<(), config_file::ConfigFileError> {
        let config = load_config();
        assert!(config.is_ok());
        Ok(())
    }
}
