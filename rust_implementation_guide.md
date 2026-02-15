# Implementación Técnica - Mejoras Críticas en Rust

## 🔧 Módulos Nuevos a Implementar

---

## 1. Sistema de Confluencias (confluence_scorer.rs)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfluenceSignal {
    pub name: String,
    pub score: u8,
    pub weight: u8,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct ConfluenceScorer {
    min_score: u8,
}

impl ConfluenceScorer {
    pub fn new(min_score: u8) -> Self {
        Self { min_score }
    }

    pub async fn calculate_score(
        &self,
        market_data: &MarketData,
        timeframe: Timeframe,
    ) -> ConfluenceResult {
        let mut signals = Vec::new();

        // Señales Primarias (40 pts)
        signals.push(self.check_breakout(market_data, 20));
        signals.push(self.check_retest(market_data, 20));

        // Señales Secundarias (35 pts)
        signals.push(self.check_volume_spike(market_data, 10));
        signals.push(self.check_rsi_zone(market_data, 10));
        signals.push(self.check_macd_divergence(market_data, 15));

        // Confluencias de Tiempo (25 pts)
        signals.push(self.check_multi_timeframe_alignment(market_data, 15));
        signals.push(self.check_trading_session(10));

        let total_score: u8 = signals
            .iter()
            .filter(|s| s.is_active)
            .map(|s| s.score)
            .sum();

        ConfluenceResult {
            total_score,
            signals,
            is_valid: total_score >= self.min_score,
        }
    }

    fn check_breakout(&self, data: &MarketData, weight: u8) -> ConfluenceSignal {
        // Implementación de detección de breakout
        let is_active = data.has_structure_break();
        
        ConfluenceSignal {
            name: "Breakout Confirmado".to_string(),
            score: if is_active { weight } else { 0 },
            weight,
            is_active,
        }
    }

    fn check_volume_spike(&self, data: &MarketData, weight: u8) -> ConfluenceSignal {
        let avg_volume = data.volume_sma(20);
        let current_volume = data.current_volume();
        let is_active = current_volume > avg_volume * 1.5;

        ConfluenceSignal {
            name: "Volume Spike".to_string(),
            score: if is_active { weight } else { 0 },
            weight,
            is_active,
        }
    }

    fn check_rsi_zone(&self, data: &MarketData, weight: u8) -> ConfluenceSignal {
        let rsi = data.rsi(14);
        let direction = data.trend_direction();
        
        let is_active = match direction {
            TrendDirection::Long => rsi > 40.0 && rsi < 70.0,
            TrendDirection::Short => rsi > 30.0 && rsi < 60.0,
            _ => false,
        };

        ConfluenceSignal {
            name: "RSI Zona Favorable".to_string(),
            score: if is_active { weight } else { 0 },
            weight,
            is_active,
        }
    }

    // ... más implementaciones
}

#[derive(Debug)]
pub struct ConfluenceResult {
    pub total_score: u8,
    pub signals: Vec<ConfluenceSignal>,
    pub is_valid: bool,
}
```

---

## 2. Riesgo Dinámico (adaptive_sizing.rs)

```rust
use rust_decimal::Decimal;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AdaptiveRiskManager {
    base_risk: Decimal,
    min_risk: Decimal,
    max_risk: Decimal,
    recent_trades: VecDeque<TradeResult>,
    max_history: usize,
}

impl AdaptiveRiskManager {
    pub fn new(base: f64, min: f64, max: f64) -> Self {
        Self {
            base_risk: Decimal::from_f64_retain(base).unwrap(),
            min_risk: Decimal::from_f64_retain(min).unwrap(),
            max_risk: Decimal::from_f64_retain(max).unwrap(),
            recent_trades: VecDeque::new(),
            max_history: 10,
        }
    }

    pub fn calculate_risk_percent(
        &mut self,
        account_balance: Decimal,
        initial_balance: Decimal,
        current_dd: Decimal,
    ) -> Decimal {
        let profit_percent = ((account_balance - initial_balance) / initial_balance) * Decimal::from(100);
        
        // Fase 1: Inicio conservador
        if profit_percent < Decimal::from(3) {
            return self.base_risk;
        }

        // Fase 2: Confianza (3-6% ganancia)
        if profit_percent < Decimal::from(6) {
            return Decimal::from_f64_retain(0.7).unwrap();
        }

        // Fase 3: Objetivo final (6-10% ganancia)
        if profit_percent < Decimal::from(10) {
            return self.max_risk;
        }

        // Protección por drawdown
        if current_dd > Decimal::from(3) {
            return self.min_risk;
        }

        // Análisis de racha
        self.adjust_for_streak()
    }

    fn adjust_for_streak(&self) -> Decimal {
        if self.recent_trades.len() < 2 {
            return self.base_risk;
        }

        let last_two: Vec<_> = self.recent_trades.iter().rev().take(2).collect();
        
        // 2 pérdidas consecutivas
        if last_two.iter().all(|t| !t.is_win) {
            return self.min_risk;
        }

        // 2 ganancias consecutivas
        if last_two.iter().all(|t| t.is_win) {
            return self.base_risk; // Mantener, no aumentar
        }

        self.base_risk
    }

    pub fn record_trade(&mut self, result: TradeResult) {
        self.recent_trades.push_back(result);
        if self.recent_trades.len() > self.max_history {
            self.recent_trades.pop_front();
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradeResult {
    pub is_win: bool,
    pub pnl: Decimal,
    pub timestamp: i64,
}
```

---

## 3. Selección Dinámica de Activos (asset_ranker.rs)

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetScore {
    pub symbol: String,
    pub volatility_score: f64,
    pub trend_strength: f64,
    pub liquidity_score: f64,
    pub spread_cost: f64,
    pub total_score: f64,
}

pub struct AssetRanker {
    weights: ScoreWeights,
}

#[derive(Debug, Clone)]
struct ScoreWeights {
    volatility: f64,
    trend: f64,
    liquidity: f64,
    spread: f64,
}

impl AssetRanker {
    pub fn new() -> Self {
        Self {
            weights: ScoreWeights {
                volatility: 0.4,
                trend: 0.3,
                liquidity: 0.2,
                spread: 0.1,
            },
        }
    }

    pub async fn rank_assets(
        &self,
        candidates: Vec<String>,
        market_data: &HashMap<String, MarketData>,
    ) -> Vec<AssetScore> {
        let mut scores = Vec::new();

        for symbol in candidates {
            if let Some(data) = market_data.get(&symbol) {
                let score = self.calculate_asset_score(&symbol, data);
                scores.push(score);
            }
        }

        scores.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
        scores
    }

    fn calculate_asset_score(&self, symbol: &str, data: &MarketData) -> AssetScore {
        let volatility_score = self.score_volatility(data);
        let trend_strength = self.score_trend(data);
        let liquidity_score = self.score_liquidity(data);
        let spread_cost = self.score_spread(data);

        let total_score = 
            (volatility_score * self.weights.volatility) +
            (trend_strength * self.weights.trend) +
            (liquidity_score * self.weights.liquidity) +
            (spread_cost * self.weights.spread);

        AssetScore {
            symbol: symbol.to_string(),
            volatility_score,
            trend_strength,
            liquidity_score,
            spread_cost,
            total_score: total_score * 100.0, // 0-100 scale
        }
    }

    fn score_volatility(&self, data: &MarketData) -> f64 {
        let atr = data.atr(14);
        let price = data.close();
        let atr_percent = (atr / price) * 100.0;

        // Sweet spot: 2-4% ATR diario
        match atr_percent {
            x if x < 1.0 => 0.3,
            x if x < 2.0 => 0.6,
            x if x >= 2.0 && x <= 4.0 => 1.0, // Óptimo
            x if x > 4.0 && x <= 6.0 => 0.7,
            _ => 0.2, // Demasiado volátil
        }
    }

    fn score_trend(&self, data: &MarketData) -> f64 {
        let adx = data.adx(14);
        
        match adx {
            x if x < 20.0 => 0.2,
            x if x >= 20.0 && x < 25.0 => 0.5,
            x if x >= 25.0 && x < 40.0 => 1.0, // Tendencia clara
            x if x >= 40.0 => 0.8, // Posible agotamiento
            _ => 0.0,
        }
    }

    fn score_liquidity(&self, data: &MarketData) -> f64 {
        let volume_24h = data.volume_24h();
        let book_depth = data.orderbook_depth(10);

        // BTCUSDT como referencia (100%)
        let btc_volume = 20_000_000_000.0; // ~$20B
        let relative_volume = (volume_24h / btc_volume).min(1.0);
        
        let depth_score = if book_depth > 500_000.0 { 1.0 } else { 0.5 };

        (relative_volume + depth_score) / 2.0
    }

    fn score_spread(&self, data: &MarketData) -> f64 {
        let spread_percent = data.spread_bps() / 10000.0 * 100.0;
        
        match spread_percent {
            x if x < 0.02 => 1.0,
            x if x < 0.05 => 0.8,
            x if x < 0.10 => 0.5,
            _ => 0.2,
        }
    }
}
```

---

## 4. Sistema de 3 Zonas Fibonacci (smart_entry.rs)

```rust
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub enum FibZone {
    Premium,   // 61.8-78.6%
    Standard,  // 50.0-61.8%
    Marginal,  // 38.2-50.0%
    Invalid,
}

pub struct SmartEntryManager {
    confluence_scorer: ConfluenceScorer,
}

impl SmartEntryManager {
    pub fn new(min_confluence: u8) -> Self {
        Self {
            confluence_scorer: ConfluenceScorer::new(min_confluence),
        }
    }

    pub async fn evaluate_entry(
        &self,
        market_data: &MarketData,
        swing_high: Decimal,
        swing_low: Decimal,
        direction: TrendDirection,
    ) -> Result<EntrySignal, String> {
        // 1. Calcular zona Fibonacci
        let current_price = market_data.close();
        let fib_zone = self.calculate_fib_zone(
            current_price,
            swing_high,
            swing_low,
            direction,
        );

        // 2. Verificar confluencias
        let confluence = self.confluence_scorer
            .calculate_score(market_data, Timeframe::M15)
            .await;

        // 3. Validar según zona
        let is_valid = match fib_zone {
            FibZone::Premium => confluence.total_score >= 65,
            FibZone::Standard => confluence.total_score >= 70,
            FibZone::Marginal => confluence.total_score >= 80,
            FibZone::Invalid => false,
        };

        if !is_valid {
            return Err("Entrada no válida: baja confluencia para la zona".to_string());
        }

        // 4. Confirmación en M5
        let m5_confirmation = self.check_m5_confirmation(market_data, direction);
        if !m5_confirmation {
            return Err("Sin confirmación en M5".to_string());
        }

        Ok(EntrySignal {
            price: current_price,
            fib_zone,
            confluence_score: confluence.total_score,
            direction,
        })
    }

    fn calculate_fib_zone(
        &self,
        price: Decimal,
        high: Decimal,
        low: Decimal,
        direction: TrendDirection,
    ) -> FibZone {
        let range = high - low;
        
        let fib_382 = low + (range * Decimal::from_f64_retain(0.382).unwrap());
        let fib_500 = low + (range * Decimal::from_f64_retain(0.500).unwrap());
        let fib_618 = low + (range * Decimal::from_f64_retain(0.618).unwrap());
        let fib_786 = low + (range * Decimal::from_f64_retain(0.786).unwrap());

        match direction {
            TrendDirection::Long => {
                if price >= fib_618 && price <= fib_786 {
                    FibZone::Premium
                } else if price >= fib_500 && price < fib_618 {
                    FibZone::Standard
                } else if price >= fib_382 && price < fib_500 {
                    FibZone::Marginal
                } else {
                    FibZone::Invalid
                }
            }
            TrendDirection::Short => {
                // Invertir lógica para shorts
                if price <= fib_382 && price >= fib_500 - fib_786 {
                    FibZone::Premium
                } else if price <= fib_500 && price > fib_382 {
                    FibZone::Standard
                } else if price <= fib_618 && price > fib_500 {
                    FibZone::Marginal
                } else {
                    FibZone::Invalid
                }
            }
            _ => FibZone::Invalid,
        }
    }

    fn check_m5_confirmation(&self, data: &MarketData, direction: TrendDirection) -> bool {
        let last_candle = data.last_candle(Timeframe::M5);
        let avg_range = data.average_range(Timeframe::M5, 20);

        // Vela de rechazo pequeña
        if last_candle.range() > avg_range * Decimal::from_f64_retain(0.3).unwrap() {
            return false;
        }

        // Cierre en favor
        match direction {
            TrendDirection::Long => last_candle.close > last_candle.open,
            TrendDirection::Short => last_candle.close < last_candle.open,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct EntrySignal {
    pub price: Decimal,
    pub fib_zone: FibZone,
    pub confluence_score: u8,
    pub direction: TrendDirection,
}
```

---

## 5. Take Profit Dinámico (dynamic_tp.rs)

```rust
use rust_decimal::Decimal;

pub struct DynamicTPManager {
    trailing_enabled: bool,
}

impl DynamicTPManager {
    pub fn new() -> Self {
        Self {
            trailing_enabled: true,
        }
    }

    pub fn calculate_targets(
        &self,
        entry_price: Decimal,
        stop_loss: Decimal,
        atr: Decimal,
        direction: TrendDirection,
    ) -> TakeProfitLevels {
        let sl_distance = match direction {
            TrendDirection::Long => entry_price - stop_loss,
            TrendDirection::Short => stop_loss - entry_price,
            _ => panic!("Invalid direction"),
        };

        let tp1 = self.calculate_tp(entry_price, sl_distance, 1.5, direction);
        let tp2 = self.calculate_tp(entry_price, sl_distance, 2.5, direction);
        let tp3 = self.calculate_tp(entry_price, sl_distance, 4.0, direction);

        let trailing_distance = atr * Decimal::from_f64_retain(1.5).unwrap();

        TakeProfitLevels {
            tp1: TPLevel {
                price: tp1,
                size_percent: Decimal::from(40),
                hit: false,
            },
            tp2: TPLevel {
                price: tp2,
                size_percent: Decimal::from(40),
                hit: false,
            },
            tp3: TPLevel {
                price: tp3,
                size_percent: Decimal::from(20),
                hit: false,
            },
            trailing_stop: TrailingStop {
                enabled: false,
                distance: trailing_distance,
                current_level: stop_loss,
            },
        }
    }

    fn calculate_tp(
        &self,
        entry: Decimal,
        sl_distance: Decimal,
        multiplier: f64,
        direction: TrendDirection,
    ) -> Decimal {
        let target_distance = sl_distance * Decimal::from_f64_retain(multiplier).unwrap();

        match direction {
            TrendDirection::Long => entry + target_distance,
            TrendDirection::Short => entry - target_distance,
            _ => entry,
        }
    }

    pub fn update_trailing_stop(
        &self,
        levels: &mut TakeProfitLevels,
        current_price: Decimal,
        direction: TrendDirection,
    ) {
        if !levels.trailing_stop.enabled || !levels.tp1.hit {
            return;
        }

        let new_stop = match direction {
            TrendDirection::Long => {
                let candidate = current_price - levels.trailing_stop.distance;
                if candidate > levels.trailing_stop.current_level {
                    Some(candidate)
                } else {
                    None
                }
            }
            TrendDirection::Short => {
                let candidate = current_price + levels.trailing_stop.distance;
                if candidate < levels.trailing_stop.current_level {
                    Some(candidate)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(new_level) = new_stop {
            levels.trailing_stop.current_level = new_level;
            tracing::info!("Trailing stop actualizado: {}", new_level);
        }
    }
}

#[derive(Debug, Clone)]
pub struct TakeProfitLevels {
    pub tp1: TPLevel,
    pub tp2: TPLevel,
    pub tp3: TPLevel,
    pub trailing_stop: TrailingStop,
}

#[derive(Debug, Clone)]
pub struct TPLevel {
    pub price: Decimal,
    pub size_percent: Decimal,
    pub hit: bool,
}

#[derive(Debug, Clone)]
pub struct TrailingStop {
    pub enabled: bool,
    pub distance: Decimal,
    pub current_level: Decimal,
}
```

---

## 6. Filtro de Noticias (news_calendar.rs)

```rust
use chrono::{DateTime, Utc, Duration};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct NewsEvent {
    pub title: String,
    pub impact: ImpactLevel,
    pub currency: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

pub struct NewsCalendar {
    events: Vec<NewsEvent>,
    blackout_before: Duration,
    blackout_after: Duration,
}

impl NewsCalendar {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            blackout_before: Duration::hours(1),
            blackout_after: Duration::hours(2),
        }
    }

    pub async fn fetch_events(&mut self) -> Result<(), String> {
        // Integración con API de calendario económico
        // Ejemplo: ForexFactory, Investing.com API, etc.
        
        // Eventos a monitorear (ejemplo hardcoded)
        let high_impact_keywords = vec![
            "FOMC", "NFP", "CPI", "Interest Rate",
            "GDP", "Unemployment", "Powell Speech",
        ];

        // TODO: Fetch real data from API
        Ok(())
    }

    pub fn is_safe_to_trade(&self, now: DateTime<Utc>) -> bool {
        for event in &self.events {
            if event.impact != ImpactLevel::High {
                continue;
            }

            let blackout_start = event.time - self.blackout_before;
            let blackout_end = event.time + self.blackout_after;

            if now >= blackout_start && now <= blackout_end {
                tracing::warn!(
                    "Trading bloqueado por evento: {} a las {}",
                    event.title,
                    event.time
                );
                return false;
            }
        }

        true
    }

    pub fn get_upcoming_events(&self, hours: i64) -> Vec<&NewsEvent> {
        let now = Utc::now();
        let cutoff = now + Duration::hours(hours);

        self.events
            .iter()
            .filter(|e| e.time >= now && e.time <= cutoff)
            .collect()
    }
}
```

---

## 7. Dashboard de Métricas (performance_metrics.rs)

```rust
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub current_drawdown: f64,
    pub total_pnl: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub consecutive_wins: usize,
    pub consecutive_losses: usize,
    pub max_consecutive_wins: usize,
    pub max_consecutive_losses: usize,
    pub valid_trading_days: usize,
}

pub struct MetricsCalculator {
    trades: VecDeque<CompletedTrade>,
    daily_pnl: Vec<f64>,
    max_history: usize,
}

impl MetricsCalculator {
    pub fn new(max_history: usize) -> Self {
        Self {
            trades: VecDeque::new(),
            daily_pnl: Vec::new(),
            max_history,
        }
    }

    pub fn add_trade(&mut self, trade: CompletedTrade) {
        self.trades.push_back(trade);
        if self.trades.len() > self.max_history {
            self.trades.pop_front();
        }
    }

    pub fn calculate(&self) -> PerformanceMetrics {
        let total_trades = self.trades.len();
        if total_trades == 0 {
            return PerformanceMetrics::default();
        }

        let winning_trades: Vec<_> = self.trades.iter()
            .filter(|t| t.pnl > 0.0)
            .collect();
        
        let losing_trades: Vec<_> = self.trades.iter()
            .filter(|t| t.pnl < 0.0)
            .collect();

        let win_rate = winning_trades.len() as f64 / total_trades as f64;

        let total_wins: f64 = winning_trades.iter().map(|t| t.pnl).sum();
        let total_losses: f64 = losing_trades.iter().map(|t| t.pnl.abs()).sum();
        
        let profit_factor = if total_losses > 0.0 {
            total_wins / total_losses
        } else {
            0.0
        };

        let avg_win = if !winning_trades.is_empty() {
            total_wins / winning_trades.len() as f64
        } else {
            0.0
        };

        let avg_loss = if !losing_trades.is_empty() {
            total_losses / losing_trades.len() as f64
        } else {
            0.0
        };

        let (max_cons_wins, max_cons_losses, current_cons) = 
            self.calculate_streaks();

        PerformanceMetrics {
            total_trades,
            winning_trades: winning_trades.len(),
            losing_trades: losing_trades.len(),
            win_rate,
            profit_factor,
            sharpe_ratio: self.calculate_sharpe(),
            max_drawdown: self.calculate_max_drawdown(),
            current_drawdown: 0.0, // Calcular desde balance actual
            total_pnl: self.trades.iter().map(|t| t.pnl).sum(),
            avg_win,
            avg_loss,
            largest_win: winning_trades.iter()
                .map(|t| t.pnl)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            largest_loss: losing_trades.iter()
                .map(|t| t.pnl.abs())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            consecutive_wins: current_cons.0,
            consecutive_losses: current_cons.1,
            max_consecutive_wins: max_cons_wins,
            max_consecutive_losses: max_cons_losses,
            valid_trading_days: 0, // Calcular desde log
        }
    }

    fn calculate_sharpe(&self) -> f64 {
        if self.trades.is_empty() {
            return 0.0;
        }

        let returns: Vec<f64> = self.trades.iter().map(|t| t.pnl).collect();
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        
        let variance: f64 = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        
        let std_dev = variance.sqrt();
        
        if std_dev == 0.0 {
            0.0
        } else {
            mean / std_dev * (252.0_f64).sqrt() // Anualizado
        }
    }

    fn calculate_max_drawdown(&self) -> f64 {
        let mut peak = 0.0;
        let mut max_dd = 0.0;
        let mut cumulative = 0.0;

        for trade in &self.trades {
            cumulative += trade.pnl;
            if cumulative > peak {
                peak = cumulative;
            }
            let dd = peak - cumulative;
            if dd > max_dd {
                max_dd = dd;
            }
        }

        max_dd
    }

    fn calculate_streaks(&self) -> (usize, usize, (usize, usize)) {
        let mut max_wins = 0;
        let mut max_losses = 0;
        let mut current_wins = 0;
        let mut current_losses = 0;

        for trade in &self.trades {
            if trade.pnl > 0.0 {
                current_wins += 1;
                current_losses = 0;
                max_wins = max_wins.max(current_wins);
            } else {
                current_losses += 1;
                current_wins = 0;
                max_losses = max_losses.max(current_losses);
            }
        }

        (max_wins, max_losses, (current_wins, current_losses))
    }
}

#[derive(Debug, Clone)]
pub struct CompletedTrade {
    pub symbol: String,
    pub pnl: f64,
    pub entry_time: i64,
    pub exit_time: i64,
    pub direction: String,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            win_rate: 0.0,
            profit_factor: 0.0,
            sharpe_ratio: 0.0,
            max_drawdown: 0.0,
            current_drawdown: 0.0,
            total_pnl: 0.0,
            avg_win: 0.0,
            avg_loss: 0.0,
            largest_win: 0.0,
            largest_loss: 0.0,
            consecutive_wins: 0,
            consecutive_losses: 0,
            max_consecutive_wins: 0,
            max_consecutive_losses: 0,
            valid_trading_days: 0,
        }
    }
}
```

---

## 8. Sistema de Alertas Telegram (alert_system.rs)

```rust
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
            "🟢 <b>TRADE ABIERTO</b>\n\
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
            "{} <b>TRADE CERRADO</b>\n\
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

    pub async fn send_daily_summary(&self, metrics: &PerformanceMetrics) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let message = format!(
            "📊 <b>RESUMEN DIARIO</b>\n\n\
            Total Trades: {}\n\
            Win Rate: {:.1}%\n\
            P&L: ${:.2}\n\
            Drawdown: {:.2}%\n\
            Sharpe: {:.2}\n\
            Días válidos: {}/10\n\n\
            Progreso: {:.1}% del objetivo",
            metrics.total_trades,
            metrics.win_rate * 100.0,
            metrics.total_pnl,
            metrics.current_drawdown,
            metrics.sharpe_ratio,
            metrics.valid_trading_days,
            (metrics.total_pnl / 1000.0) * 100.0,
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
```

---

## 📝 CHECKLIST DE IMPLEMENTACIÓN

### Prioridad ALTA (Semana 1)
- [ ] `confluence_scorer.rs` - Sistema de puntuación
- [ ] `adaptive_sizing.rs` - Riesgo dinámico
- [ ] `smart_entry.rs` - Zonas Fibonacci
- [ ] `dynamic_tp.rs` - Take profits mejorados
- [ ] `performance_metrics.rs` - Tracking de métricas

### Prioridad MEDIA (Semana 2)
- [ ] `asset_ranker.rs` - Selección dinámica
- [ ] `news_calendar.rs` - Filtro de noticias
- [ ] `alert_system.rs` - Notificaciones Telegram
- [ ] Tests unitarios para cada módulo

### Prioridad BAJA (Semana 3)
- [ ] Dashboard web (opcional)
- [ ] Backtesting avanzado
- [ ] Optimización de parámetros
- [ ] Documentación completa

---

## 🧪 TESTS UNITARIOS EJEMPLO

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confluence_scoring() {
        let scorer = ConfluenceScorer::new(70);
        // Test data...
        assert!(result.total_score >= 70);
    }

    #[test]
    fn test_adaptive_risk_scaling() {
        let mut manager = AdaptiveRiskManager::new(0.5, 0.3, 1.0);
        
        // Test fase inicial
        let risk = manager.calculate_risk_percent(
            Decimal::from(10100),
            Decimal::from(10000),
            Decimal::from(0),
        );
        assert_eq!(risk, Decimal::from_f64_retain(0.5).unwrap());
        
        // Test fase confianza
        let risk = manager.calculate_risk_percent(
            Decimal::from(10400),
            Decimal::from(10000),
            Decimal::from(0),
        );
        assert_eq!(risk, Decimal::from_f64_retain(0.7).unwrap());
    }

    #[test]
    fn test_fib_zone_calculation() {
        let manager = SmartEntryManager::new(70);
        let zone = manager.calculate_fib_zone(
            Decimal::from(50000),
            Decimal::from(52000),
            Decimal::from(48000),
            TrendDirection::Long,
        );
        
        // 50000 debería estar en zona standard (50-61.8%)
        assert_eq!(zone, FibZone::Standard);
    }
}
```

---

## 🚀 DEPLOYMENT

```toml
# Cargo.toml dependencies adicionales
[dependencies]
rust_decimal = "1.33"
chrono = "0.4"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Siguiente paso:** Implementar módulos en orden de prioridad y ejecutar backtesting.
