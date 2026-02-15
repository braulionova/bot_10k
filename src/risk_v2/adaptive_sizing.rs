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
            base_risk: Decimal::try_from(base).unwrap(),
            min_risk: Decimal::try_from(min).unwrap(),
            max_risk: Decimal::try_from(max).unwrap(),
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

        // Phase 1: Conservative start
        if profit_percent < Decimal::from(3) {
            return self.base_risk;
        }

        // Phase 2: Confidence (3-6% profit)
        if profit_percent < Decimal::from(6) {
            return Decimal::try_from(0.7).unwrap();
        }

        // Phase 3: Final target (6-10% profit)
        if profit_percent < Decimal::from(10) {
            return self.max_risk;
        }

        // Drawdown protection
        if current_dd > Decimal::from(3) {
            return self.min_risk;
        }

        // Streak analysis
        self.adjust_for_streak()
    }

    fn adjust_for_streak(&self) -> Decimal {
        if self.recent_trades.len() < 2 {
            return self.base_risk;
        }

        let last_two: Vec<_> = self.recent_trades.iter().rev().take(2).collect();

        // 2 consecutive losses
        if last_two.iter().all(|t| !t.is_win) {
            return self.min_risk;
        }

        // 2 consecutive wins
        if last_two.iter().all(|t| t.is_win) {
            return self.base_risk; // Maintain, don't increase
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
