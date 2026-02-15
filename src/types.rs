use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Long,
    Short,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Timeframe {
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
}

impl Timeframe {
    pub fn to_minutes(&self) -> u32 {
        match self {
            Timeframe::M1 => 1,
            Timeframe::M5 => 5,
            Timeframe::M15 => 15,
            Timeframe::M30 => 30,
            Timeframe::H1 => 60,
            Timeframe::H4 => 240,
            Timeframe::D1 => 1440,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: i64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

impl Candle {
    pub fn range(&self) -> Decimal {
        self.high - self.low
    }

    pub fn body(&self) -> Decimal {
        (self.close - self.open).abs()
    }

    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
}

#[derive(Debug, Clone)]
pub struct MarketData {
    pub symbol: String,
    pub candles: Vec<Candle>,
    pub timeframe: Timeframe,
}

impl MarketData {
    pub fn new(symbol: String, timeframe: Timeframe) -> Self {
        Self {
            symbol,
            candles: Vec::new(),
            timeframe,
        }
    }

    pub fn last_candle(&self) -> Option<&Candle> {
        self.candles.last()
    }

    pub fn close(&self) -> Decimal {
        self.last_candle()
            .map(|c| c.close)
            .unwrap_or(Decimal::ZERO)
    }

    pub fn current_volume(&self) -> Decimal {
        self.last_candle()
            .map(|c| c.volume)
            .unwrap_or(Decimal::ZERO)
    }

    // RSI (Relative Strength Index)
    pub fn rsi(&self, period: usize) -> f64 {
        if self.candles.len() < period + 1 {
            return 50.0;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in (self.candles.len() - period)..self.candles.len() {
            let change = (self.candles[i].close - self.candles[i - 1].close)
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);

            if change > 0.0 {
                gains += change;
            } else {
                losses += change.abs();
            }
        }

        let avg_gain = gains / period as f64;
        let avg_loss = losses / period as f64;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }

    // ATR (Average True Range)
    pub fn atr(&self, period: usize) -> Decimal {
        if self.candles.len() < period + 1 {
            return Decimal::ZERO;
        }

        let mut tr_sum = Decimal::ZERO;

        for i in (self.candles.len() - period)..self.candles.len() {
            let high_low = self.candles[i].high - self.candles[i].low;
            let high_close = (self.candles[i].high - self.candles[i - 1].close).abs();
            let low_close = (self.candles[i].low - self.candles[i - 1].close).abs();

            let tr = high_low.max(high_close).max(low_close);
            tr_sum += tr;
        }

        tr_sum / Decimal::from(period)
    }

    // ADX (Average Directional Index)
    pub fn adx(&self, period: usize) -> f64 {
        if self.candles.len() < period + 1 {
            return 0.0;
        }

        let mut plus_dm_sum = 0.0;
        let mut minus_dm_sum = 0.0;
        let mut tr_sum = 0.0;

        for i in (self.candles.len() - period)..self.candles.len() {
            let high_diff = (self.candles[i].high - self.candles[i - 1].high)
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);
            let low_diff = (self.candles[i - 1].low - self.candles[i].low)
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0);

            let plus_dm = if high_diff > low_diff && high_diff > 0.0 { high_diff } else { 0.0 };
            let minus_dm = if low_diff > high_diff && low_diff > 0.0 { low_diff } else { 0.0 };

            let tr = self.candles[i].range().to_string().parse::<f64>().unwrap_or(0.0);

            plus_dm_sum += plus_dm;
            minus_dm_sum += minus_dm;
            tr_sum += tr;
        }

        if tr_sum == 0.0 {
            return 0.0;
        }

        let plus_di = (plus_dm_sum / tr_sum) * 100.0;
        let minus_di = (minus_dm_sum / tr_sum) * 100.0;

        let dx = ((plus_di - minus_di).abs() / (plus_di + minus_di)) * 100.0;
        dx
    }

    // EMA (Exponential Moving Average)
    pub fn ema(&self, period: usize) -> Decimal {
        if self.candles.len() < period {
            return self.close();
        }

        let multiplier = Decimal::from(2) / Decimal::from(period + 1);
        let mut ema = self.candles.iter()
            .take(period)
            .map(|c| c.close)
            .sum::<Decimal>() / Decimal::from(period);

        for candle in self.candles.iter().skip(period) {
            ema = (candle.close - ema) * multiplier + ema;
        }

        ema
    }

    // SMA for volume
    pub fn volume_sma(&self, period: usize) -> Decimal {
        if self.candles.len() < period {
            return self.current_volume();
        }

        self.candles.iter()
            .rev()
            .take(period)
            .map(|c| c.volume)
            .sum::<Decimal>() / Decimal::from(period)
    }

    // Average range for candles
    pub fn average_range(&self, period: usize) -> Decimal {
        if self.candles.len() < period {
            return Decimal::ZERO;
        }

        self.candles.iter()
            .rev()
            .take(period)
            .map(|c| c.range())
            .sum::<Decimal>() / Decimal::from(period)
    }

    // Trend direction based on EMAs
    pub fn trend_direction(&self) -> TrendDirection {
        if self.candles.len() < 50 {
            return TrendDirection::Neutral;
        }

        let ema_20 = self.ema(20);
        let ema_50 = self.ema(50);
        let current_price = self.close();

        if current_price > ema_20 && ema_20 > ema_50 {
            TrendDirection::Long
        } else if current_price < ema_20 && ema_20 < ema_50 {
            TrendDirection::Short
        } else {
            TrendDirection::Neutral
        }
    }

    // Check if structure break exists (simplified)
    pub fn has_structure_break(&self) -> bool {
        if self.candles.len() < 20 {
            return false;
        }

        let recent_high = self.candles.iter()
            .rev()
            .take(20)
            .map(|c| c.high)
            .max()
            .unwrap_or(Decimal::ZERO);

        let recent_low = self.candles.iter()
            .rev()
            .take(20)
            .map(|c| c.low)
            .min()
            .unwrap_or(Decimal::ZERO);

        let current = self.close();
        let previous = self.candles.get(self.candles.len() - 2)
            .map(|c| c.close)
            .unwrap_or(Decimal::ZERO);

        // Bullish break
        if current > recent_high && previous <= recent_high {
            return true;
        }

        // Bearish break
        if current < recent_low && previous >= recent_low {
            return true;
        }

        false
    }

    // 24h volume (sum of last ~288 M5 candles or equivalent)
    pub fn volume_24h(&self) -> f64 {
        let candles_in_24h = match self.timeframe {
            Timeframe::M5 => 288,
            Timeframe::M15 => 96,
            Timeframe::H1 => 24,
            _ => 24,
        };

        self.candles.iter()
            .rev()
            .take(candles_in_24h)
            .map(|c| c.volume.to_string().parse::<f64>().unwrap_or(0.0))
            .sum()
    }

    // Order book depth (placeholder - needs real exchange integration)
    pub fn orderbook_depth(&self, _levels: usize) -> f64 {
        // This would need real exchange data
        // For now return a placeholder
        1_000_000.0
    }

    // Spread in basis points (placeholder - needs real exchange integration)
    pub fn spread_bps(&self) -> f64 {
        // This would need real exchange data
        // For now return typical crypto spread
        5.0 // 0.05%
    }
}
