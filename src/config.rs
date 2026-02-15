use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Challenge
    pub challenge_mode: String,
    pub initial_capital: f64,
    pub target_profit_percent: f64,
    pub min_trading_days: u32,

    // Risk Management
    pub risk_per_trade_base: f64,
    pub risk_per_trade_max: f64,
    pub risk_per_trade_min: f64,
    pub max_daily_loss_percent: f64,
    pub max_total_dd_percent: f64,

    // Strategy
    pub min_confluence_score: u8,
    pub enable_dynamic_asset_selection: bool,
    pub enable_news_filter: bool,
    pub enable_atr_tp: bool,

    // Exchange
    pub exchange_api_key: String,
    pub exchange_api_secret: String,
    pub exchange_testnet: bool,

    // Telegram
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub enable_alerts: bool,

    // Monitoring
    pub dashboard_port: u16,
    pub log_level: String,

    // Trading Session
    pub weekend_trading_enabled: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let config = envy::from_env::<Config>()?;

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.min_confluence_score > 100 {
            anyhow::bail!("min_confluence_score must be <= 100");
        }

        if self.risk_per_trade_min > self.risk_per_trade_max {
            anyhow::bail!("risk_per_trade_min must be <= risk_per_trade_max");
        }

        if self.initial_capital <= 0.0 {
            anyhow::bail!("initial_capital must be > 0");
        }

        Ok(())
    }
}
