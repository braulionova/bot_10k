use std::collections::HashMap;
use crate::types::MarketData;

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

    fn calculate_asset_score(&self, symbol: &str, _data: &MarketData) -> AssetScore {
        // TODO: Implement actual scoring logic
        let volatility_score = 0.5;
        let trend_strength = 0.5;
        let liquidity_score = 0.5;
        let spread_cost = 0.5;

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
            total_score: total_score * 100.0,
        }
    }
}

impl Default for AssetRanker {
    fn default() -> Self {
        Self::new()
    }
}
