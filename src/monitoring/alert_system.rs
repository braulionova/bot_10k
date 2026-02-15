use reqwest::Client;
use serde_json::json;

pub struct TelegramAlerter {
    bot_token: String,
    chat_id: String,
    client: Client,
    enabled: bool,
}

impl TelegramAlerter {
    pub fn new(bot_token: String, chat_id: String, enabled: bool) -> Self {
        Self {
            bot_token,
            chat_id,
            client: Client::new(),
            enabled,
        }
    }

    pub async fn send_trade_opened(&self, trade: &TradeInfo) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let message = format!(
            "🟢 <b>TRADE OPENED</b>\n\
            Symbol: {}\n\
            Direction: {}\n\
            Entry: ${:.2}\n\
            Stop Loss: ${:.2}\n\
            Take Profits: ${:.2} / ${:.2} / ${:.2}\n\
            Risk: {:.2}%\n\
            Confluence Score: {}/100\n\
            Size: {} contracts",
            trade.symbol,
            trade.direction,
            trade.entry_price,
            trade.stop_loss,
            trade.tp1,
            trade.tp2,
            trade.tp3,
            trade.risk_percent,
            trade.confluence_score,
            trade.size,
        );

        self.send_message(&message).await
    }

    pub async fn send_trade_closed(
        &self,
        trade: &TradeInfo,
        exit_price: f64,
        pnl: f64,
        reason: &str,
    ) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let emoji = if pnl > 0.0 { "💰" } else { "🔴" };
        let message = format!(
            "{} <b>TRADE CLOSED</b>\n\
            Symbol: {}\n\
            Entry: ${:.2}\n\
            Exit: ${:.2}\n\
            P&L: ${:.2} ({:.2}%)\n\
            Reason: {}",
            emoji,
            trade.symbol,
            trade.entry_price,
            exit_price,
            pnl,
            (pnl / (trade.entry_price * trade.size as f64)) * 100.0,
            reason,
        );

        self.send_message(&message).await
    }

    pub async fn send_alert(&self, message: &str, level: AlertLevel) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let emoji = match level {
            AlertLevel::Info => "ℹ️",
            AlertLevel::Warning => "⚠️",
            AlertLevel::Critical => "🛑",
        };

        let formatted = format!("{} {}", emoji, message);
        self.send_message(&formatted).await
    }

    pub async fn send_startup(&self, initial_balance: f64, config: &StartupConfig) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let message = format!(
            "🚀 <b>HYROTRADER BOT INICIADO</b>\n\n\
            💼 <b>Configuración:</b>\n\
            • Capital inicial: ${:.2}\n\
            • Objetivo: ${:.2} (+{}%)\n\
            • Días mínimos: {}\n\
            • Exchange: {} {}\n\n\
            ⚙️ <b>Parámetros:</b>\n\
            • Riesgo base: {}%\n\
            • Riesgo min/max: {}% - {}%\n\
            • Score mínimo: {}/100\n\
            • Max drawdown: {}%\n\n\
            📊 <b>Assets monitoreados:</b>\n\
            • BTCUSDT, ETHUSDT\n\n\
            ✅ Bot activo - Buscando oportunidades...",
            initial_balance,
            initial_balance * (config.target_profit_pct / 100.0),
            config.target_profit_pct,
            config.min_days,
            config.exchange_name,
            if config.testnet { "(TESTNET)" } else { "" },
            config.risk_base,
            config.risk_min,
            config.risk_max,
            config.min_confluence,
            config.max_drawdown,
        );

        self.send_message(&message).await
    }

    pub async fn send_balance_update(&self, balance: &BalanceInfo) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let emoji = if balance.pnl > 0.0 { "📈" } else if balance.pnl < 0.0 { "📉" } else { "📊" };
        let pnl_emoji = if balance.pnl > 0.0 { "💰" } else { "🔴" };

        let message = format!(
            "{} <b>ACTUALIZACIÓN DE BALANCE</b>\n\n\
            💼 <b>Balance actual:</b> ${:.2}\n\
            {} <b>P&L:</b> ${:+.2} ({:+.2}%)\n\
            🎯 <b>Objetivo:</b> ${:.2} / ${:.2}\n\
            📊 <b>Progreso:</b> {:.1}%\n\
            📉 <b>Drawdown:</b> {:.2}%\n\n\
            📈 <b>Estadísticas:</b>\n\
            • Trades: {}\n\
            • Win rate: {:.1}%\n\
            • Días válidos: {}/{}",
            emoji,
            balance.current,
            pnl_emoji,
            balance.pnl,
            balance.pnl_pct,
            balance.current,
            balance.target,
            (balance.current - balance.initial) / (balance.target - balance.initial) * 100.0,
            balance.drawdown,
            balance.total_trades,
            balance.win_rate,
            balance.valid_days,
            balance.min_days,
        );

        self.send_message(&message).await
    }

    pub async fn send_market_analysis(&self, analysis: &MarketAnalysis) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let score_emoji = if analysis.score >= 70 { "✅" } else { "⚠️" };

        let message = format!(
            "📊 <b>ANÁLISIS DE MERCADO</b>\n\n\
            🪙 <b>{}</b>\n\
            💰 Precio: ${:.2}\n\
            📈 Cambio 24h: {:+.2}%\n\n\
            📊 <b>Indicadores:</b>\n\
            • RSI(14): {:.1}\n\
            • ADX: {:.1}\n\
            • ATR: ${:.2}\n\n\
            🎯 <b>Confluencia:</b> {}/{} {}\n\
            {}\n\n\
            {}",
            analysis.symbol,
            analysis.price,
            analysis.change_24h,
            analysis.rsi,
            analysis.adx,
            analysis.atr,
            analysis.score,
            100,
            score_emoji,
            analysis.signals.join("\n"),
            if analysis.score >= 70 {
                "✅ <b>SETUP VÁLIDO - Preparado para entrar</b>"
            } else {
                "⏳ Esperando mejores condiciones..."
            }
        );

        self.send_message(&message).await
    }

    pub async fn send_daily_summary(&self, summary: &DailySummary) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let message = format!(
            "📅 <b>RESUMEN DIARIO</b>\n\n\
            📊 <b>Performance:</b>\n\
            • Trades: {} ({}W / {}L)\n\
            • Win Rate: {:.1}%\n\
            • P&L del día: ${:+.2}\n\
            • P&L total: ${:+.2} ({:+.2}%)\n\n\
            💼 <b>Balance:</b>\n\
            • Inicial: ${:.2}\n\
            • Actual: ${:.2}\n\
            • Objetivo: ${:.2}\n\n\
            📈 <b>Progreso Challenge:</b>\n\
            • Días válidos: {}/{}\n\
            • Progreso: {:.1}%\n\
            • Drawdown máx: {:.2}%\n\n\
            {}",
            summary.trades_today,
            summary.wins,
            summary.losses,
            summary.win_rate,
            summary.daily_pnl,
            summary.total_pnl,
            summary.total_pnl_pct,
            summary.initial_balance,
            summary.current_balance,
            summary.target_balance,
            summary.valid_days,
            summary.min_days,
            (summary.current_balance - summary.initial_balance) / (summary.target_balance - summary.initial_balance) * 100.0,
            summary.max_drawdown,
            if summary.valid_days >= summary.min_days && summary.total_pnl_pct >= 10.0 {
                "🎉 <b>¡CHALLENGE COMPLETADO!</b>"
            } else if summary.total_pnl_pct >= 5.0 {
                "🔥 ¡Buen progreso! A mitad de camino..."
            } else {
                "💪 Seguimos adelante..."
            }
        );

        self.send_message(&message).await
    }

    async fn send_message(&self, text: &str) -> Result<(), String> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let payload = json!({
            "chat_id": self.chat_id,
            "text": text,
            "parse_mode": "HTML",
        });

        self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct TradeInfo {
    pub symbol: String,
    pub direction: String,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub tp1: f64,
    pub tp2: f64,
    pub tp3: f64,
    pub risk_percent: f64,
    pub confluence_score: u8,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct StartupConfig {
    pub initial_balance: f64,
    pub target_profit_pct: f64,
    pub min_days: u32,
    pub exchange_name: String,
    pub testnet: bool,
    pub risk_base: f64,
    pub risk_min: f64,
    pub risk_max: f64,
    pub min_confluence: u8,
    pub max_drawdown: f64,
}

#[derive(Debug, Clone)]
pub struct BalanceInfo {
    pub initial: f64,
    pub current: f64,
    pub target: f64,
    pub pnl: f64,
    pub pnl_pct: f64,
    pub drawdown: f64,
    pub total_trades: usize,
    pub win_rate: f64,
    pub valid_days: u32,
    pub min_days: u32,
}

#[derive(Debug, Clone)]
pub struct MarketAnalysis {
    pub symbol: String,
    pub price: f64,
    pub change_24h: f64,
    pub rsi: f64,
    pub adx: f64,
    pub atr: f64,
    pub score: u8,
    pub signals: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DailySummary {
    pub trades_today: usize,
    pub wins: usize,
    pub losses: usize,
    pub win_rate: f64,
    pub daily_pnl: f64,
    pub total_pnl: f64,
    pub total_pnl_pct: f64,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub target_balance: f64,
    pub valid_days: u32,
    pub min_days: u32,
    pub max_drawdown: f64,
}
