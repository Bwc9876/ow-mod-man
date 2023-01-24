use serde::Deserialize;

use crate::config::Config;

#[derive(Deserialize)]
pub struct Alert {
    pub enabled: bool,
    pub severity: String,
    pub message: String,
}

pub async fn fetch_alert(config: &Config) -> Result<Alert, anyhow::Error> {
    let resp = reqwest::get(&config.alert_url).await?;
    let alert: Alert = serde_json::from_str(&resp.text().await?)?;
    Ok(alert)
}