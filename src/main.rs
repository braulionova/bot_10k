use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use std::sync::Arc;

mod config;
mod types;
mod intelligence;
mod risk_v2;
mod execution_v2;
mod monitoring;
mod exchange;
mod bot;

use config::Config;
use exchange::{BybitConnector, BinanceConnector};
use bot::TradingBot;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .json()
        .init();

    info!("🚀 HyroTrader Bot v2.0 Starting...");

    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Configuration loaded");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("  Challenge mode: {}", config.challenge_mode);
    info!("  Initial capital: ${}", config.initial_capital);
    info!("  Target profit: {}%", config.target_profit_percent);
    info!("  Min trading days: {}", config.min_trading_days);
    info!("  Min confluence score: {}", config.min_confluence_score);
    info!("  Risk range: {}% - {}%", config.risk_per_trade_min, config.risk_per_trade_max);
    info!("  Testnet mode: {}", config.exchange_testnet);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Determine which exchange to use based on environment
    let exchange_type = std::env::var("EXCHANGE_TYPE").unwrap_or_else(|_| "bybit".to_string());

    info!("Initializing {} connector...", exchange_type.to_uppercase());

    let exchange: Arc<dyn exchange::ExchangeConnector> = match exchange_type.as_str() {
        "bybit" => {
            Arc::new(BybitConnector::new(
                config.exchange_api_key.clone(),
                config.exchange_api_secret.clone(),
                config.exchange_testnet,
            ))
        }
        "binance" => {
            Arc::new(BinanceConnector::new(
                config.exchange_api_key.clone(),
                config.exchange_api_secret.clone(),
                config.exchange_testnet,
            ))
        }
        _ => {
            error!("Unknown exchange type: {}. Using Bybit as default.", exchange_type);
            Arc::new(BybitConnector::new(
                config.exchange_api_key.clone(),
                config.exchange_api_secret.clone(),
                config.exchange_testnet,
            ))
        }
    };

    // Test exchange connection
    info!("Testing exchange connection...");
    match exchange.get_market_data("BTCUSDT", types::Timeframe::M5, 10).await {
        Ok(data) => {
            info!("✅ Exchange connection successful");
            info!("   Fetched {} candles for BTCUSDT", data.candles.len());
            if let Some(last_candle) = data.last_candle() {
                info!("   Last close price: ${}", last_candle.close);
            }
        }
        Err(e) => {
            error!("❌ Failed to connect to exchange: {}", e);
            return Err(e);
        }
    }

    // Initialize trading bot
    info!("Initializing trading bot...");
    let mut bot = TradingBot::new(config, exchange);

    info!("✅ Bot initialized successfully");
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("  HYROTRADER CHALLENGE - TRADING RULES");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("  ✓ Max 1 position at a time");
    info!("  ✓ Confluence score ≥ 70 required");
    info!("  ✓ Risk per trade: 0.3% - 1.0% adaptive");
    info!("  ✓ Max daily drawdown: < 5%");
    info!("  ✓ Max total drawdown: < 10%");
    info!("  ✓ Kill-switch at 4 consecutive losses");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Start the bot
    info!("🚀 Starting trading loop...");

    // Setup Ctrl+C handler
    let bot_handle = tokio::spawn(async move {
        bot.start().await
    });

    tokio::select! {
        result = bot_handle => {
            match result {
                Ok(Ok(())) => info!("Bot stopped gracefully"),
                Ok(Err(e)) => error!("Bot error: {}", e),
                Err(e) => error!("Bot task panic: {}", e),
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
        }
    }

    info!("Shutdown complete");
    Ok(())
}
