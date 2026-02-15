use crate::types::MarketData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketRegime {
    Trending,
    Ranging,
    Volatile,
    LowLiquidity,
}

pub struct MarketRegimeDetector;

impl MarketRegimeDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_regime(&self, _data: &MarketData) -> MarketRegime {
        // TODO: Implement regime detection using ADX, ATR, etc.
        MarketRegime::Ranging
    }

    pub fn is_safe_to_trade(&self, regime: MarketRegime) -> bool {
        matches!(regime, MarketRegime::Trending)
    }
}

impl Default for MarketRegimeDetector {
    fn default() -> Self {
        Self::new()
    }
}
