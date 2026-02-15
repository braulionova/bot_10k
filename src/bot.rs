use anyhow::Result;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn, error};
use rust_decimal::Decimal;

use crate::config::Config;
use crate::exchange::{ExchangeConnector, AccountBalance};
use crate::intelligence::{ConfluenceScorer, AssetRanker};
use crate::risk_v2::AdaptiveRiskManager;
use crate::execution_v2::{SmartEntryManager, DynamicTPManager, NewsCalendar};
use crate::monitoring::{PerformanceMetrics, MetricsCalculator, TelegramAlerter};
use crate::types::Timeframe;
use chrono::Datelike;

pub struct TradingBot {
    config: Config,
    exchange: Arc<dyn ExchangeConnector>,
    confluence_scorer: ConfluenceScorer,
    asset_ranker: AssetRanker,
    risk_manager: AdaptiveRiskManager,
    entry_manager: SmartEntryManager,
    tp_manager: DynamicTPManager,
    news_calendar: NewsCalendar,
    alerter: Option<TelegramAlerter>,
    metrics_calculator: MetricsCalculator,

    // State
    initial_balance: Decimal,
    current_balance: Decimal,
    valid_trading_days: u32,
    is_running: bool,
}

impl TradingBot {
    pub fn new(config: Config, exchange: Arc<dyn ExchangeConnector>) -> Self {
        let initial_balance = Decimal::try_from(config.initial_capital).unwrap();

        let alerter = if config.enable_alerts {
            config.telegram_bot_token.as_ref().and_then(|token| {
                config.telegram_chat_id.as_ref().map(|chat_id| {
                    TelegramAlerter::new(
                        token.clone(),
                        chat_id.clone(),
                        config.enable_alerts
                    )
                })
            })
        } else {
            None
        };

        Self {
            confluence_scorer: ConfluenceScorer::new(config.min_confluence_score),
            asset_ranker: AssetRanker::new(),
            risk_manager: AdaptiveRiskManager::new(
                config.risk_per_trade_base,
                config.risk_per_trade_min,
                config.risk_per_trade_max,
            ),
            entry_manager: SmartEntryManager::new(config.min_confluence_score),
            tp_manager: DynamicTPManager::new(),
            news_calendar: NewsCalendar::new(),
            alerter,
            metrics_calculator: MetricsCalculator::new(1000),
            initial_balance,
            current_balance: initial_balance,
            valid_trading_days: 0,
            is_running: false,
            config,
            exchange,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting HyroTrader Bot v2.0");

        // Send detailed startup notification
        if let Some(alerter) = &self.alerter {
            let startup_config = crate::monitoring::StartupConfig {
                initial_balance: self.config.initial_capital,
                target_profit_pct: self.config.target_profit_percent,
                min_days: self.config.min_trading_days,
                exchange_name: "Bybit".to_string(),
                testnet: self.config.exchange_testnet,
                risk_base: self.config.risk_per_trade_base,
                risk_min: self.config.risk_per_trade_min,
                risk_max: self.config.risk_per_trade_max,
                min_confluence: self.config.min_confluence_score,
                max_drawdown: self.config.max_total_dd_percent,
            };

            alerter.send_startup(
                self.config.initial_capital,
                &startup_config
            ).await.ok();
        }

        // Fetch news calendar
        info!("📅 Fetching economic calendar...");
        self.news_calendar.fetch_events().await?;

        self.is_running = true;

        // Main trading loop - runs every minute
        let mut tick_interval = interval(Duration::from_secs(60));

        while self.is_running {
            tick_interval.tick().await;

            if let Err(e) = self.trading_cycle().await {
                error!("Error in trading cycle: {}", e);

                if let Some(alerter) = &self.alerter {
                    alerter.send_alert(
                        &format!("Error en ciclo de trading: {}", e),
                        crate::monitoring::AlertLevel::Warning
                    ).await.ok();
                }
            }

            // Check if we've reached the target
            if self.has_reached_target() {
                info!("🎯 TARGET REACHED! Challenge completed!");
                if let Some(alerter) = &self.alerter {
                    alerter.send_alert(
                        "🎯 OBJETIVO ALCANZADO! Challenge completado!",
                        crate::monitoring::AlertLevel::Info
                    ).await.ok();
                }
                break;
            }

            // Check kill-switch conditions
            if self.should_stop_trading() {
                warn!("🛑 Kill-switch activated - stopping trading");
                if let Some(alerter) = &self.alerter {
                    alerter.send_alert(
                        "🛑 Kill-switch activado - Trading detenido",
                        crate::monitoring::AlertLevel::Critical
                    ).await.ok();
                }
                break;
            }
        }

        info!("Bot stopped");
        Ok(())
    }

    async fn trading_cycle(&mut self) -> Result<()> {
        // 1. Update account balance
        let account = self.exchange.get_account_balance().await?;
        self.update_balance(&account);

        // 2. Check if it's safe to trade (news, market hours, etc.)
        if !self.is_safe_to_trade() {
            return Ok(());
        }

        // 3. Rank assets and select best candidates
        let assets = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
        let mut market_data_map = std::collections::HashMap::new();

        for symbol in &assets {
            let data = self.exchange.get_market_data(symbol, Timeframe::H1, 200).await?;
            market_data_map.insert(symbol.clone(), data);
        }

        let ranked_assets = self.asset_ranker.rank_assets(assets, &market_data_map).await;

        info!("Asset rankings:");
        for asset in &ranked_assets {
            info!("  {} - Score: {:.1}", asset.symbol, asset.total_score);
        }

        // 4. Check for trade opportunities on top-ranked assets
        for asset in ranked_assets.iter().take(2) {
            if asset.total_score < 75.0 {
                continue;
            }

            if let Some(data) = market_data_map.get(&asset.symbol) {
                self.evaluate_trade_opportunity(&asset.symbol, data).await?;
            }
        }

        // 5. Manage open positions
        self.manage_open_positions().await?;

        Ok(())
    }

    async fn evaluate_trade_opportunity(&mut self, symbol: &str, data: &crate::types::MarketData) -> Result<()> {
        // Calculate confluence score
        let confluence = self.confluence_scorer.calculate_score(data, Timeframe::M15).await;

        if !confluence.is_valid {
            return Ok(());
        }

        info!("🎯 Potential setup on {} - Confluence: {}/100", symbol, confluence.total_score);

        // TODO: Implement full trade entry logic
        // - Identify swing high/low
        // - Calculate Fibonacci zones
        // - Validate entry with SmartEntryManager
        // - Calculate position size with AdaptiveRiskManager
        // - Place orders via exchange
        // - Send alerts

        Ok(())
    }

    async fn manage_open_positions(&mut self) -> Result<()> {
        // TODO: Implement position management
        // - Update trailing stops
        // - Check TP levels
        // - Monitor for exit conditions
        Ok(())
    }

    fn update_balance(&mut self, account: &AccountBalance) {
        self.current_balance = Decimal::try_from(account.total_balance_usdt).unwrap();

        let profit = self.current_balance - self.initial_balance;
        let profit_pct = (profit / self.initial_balance) * Decimal::from(100);

        info!("💰 Balance: ${:.2} ({:+.2}%)",
            account.total_balance_usdt,
            profit_pct.to_string().parse::<f64>().unwrap_or(0.0)
        );
    }

    async fn send_balance_notification(&self) {
        if let Some(ref alerter) = self.alerter {
            let metrics = self.metrics_calculator.calculate();
            let balance_info = crate::monitoring::BalanceInfo {
                initial: self.initial_balance.to_string().parse().unwrap_or(0.0),
                current: self.current_balance.to_string().parse().unwrap_or(0.0),
                target: self.initial_balance.to_string().parse::<f64>().unwrap_or(0.0) * 1.10,
                pnl: (self.current_balance - self.initial_balance).to_string().parse().unwrap_or(0.0),
                pnl_pct: ((self.current_balance - self.initial_balance) / self.initial_balance * Decimal::from(100)).to_string().parse().unwrap_or(0.0),
                drawdown: metrics.current_drawdown,
                total_trades: metrics.total_trades,
                win_rate: metrics.win_rate * 100.0,
                valid_days: self.valid_trading_days,
                min_days: self.config.min_trading_days,
            };

            if let Err(e) = alerter.send_balance_update(&balance_info).await {
                tracing::warn!("Failed to send balance update: {}", e);
            }
        }
    }

    fn is_safe_to_trade(&self) -> bool {
        // Check news calendar
        let now = chrono::Utc::now();
        if !self.news_calendar.is_safe_to_trade(now) {
            warn!("⛔ Trading blocked due to upcoming high-impact news");
            return false;
        }

        // Check if it's weekend (if weekend trading is disabled)
        if !self.config.weekend_trading_enabled {
            let weekday = now.weekday();
            if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
                return false;
            }
        }

        true
    }

    fn has_reached_target(&self) -> bool {
        let profit = self.current_balance - self.initial_balance;
        let target = self.initial_balance * Decimal::try_from(self.config.target_profit_percent / 100.0).unwrap();

        profit >= target && self.valid_trading_days >= self.config.min_trading_days
    }

    fn should_stop_trading(&self) -> bool {
        let metrics = self.metrics_calculator.calculate();

        // Kill-switch condition 1: Daily loss exceeds 1.0%
        // TODO: Track daily P&L

        // Kill-switch condition 2: Total drawdown exceeds 8.0%
        if metrics.max_drawdown > 8.0 {
            return true;
        }

        // Kill-switch condition 3: Consecutive losses reach 4
        if metrics.consecutive_losses >= 4 {
            return true;
        }

        false
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics_calculator.calculate()
    }
}
