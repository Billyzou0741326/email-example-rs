pub use crate::config::AppConfig;

mod config;

pub struct App {
    pub config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}
