use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub to: String,
    pub from: String,
    pub subject: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteConfig {
    pub ip: String,
    pub port: u16,
    pub socket_addr: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub email: EmailConfig,
    pub smtp: SmtpConfig,
    pub remote: RemoteConfig,
}
