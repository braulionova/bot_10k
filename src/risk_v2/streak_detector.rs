use crate::risk_v2::TradeResult;

pub struct StreakDetector {
    max_consecutive_losses: usize,
}

impl StreakDetector {
    pub fn new(max_consecutive_losses: usize) -> Self {
        Self {
            max_consecutive_losses,
        }
    }

    pub fn should_reduce_risk(&self, recent_trades: &[TradeResult]) -> bool {
        if recent_trades.len() < 2 {
            return false;
        }

        let consecutive_losses = self.count_consecutive_losses(recent_trades);
        consecutive_losses >= 2
    }

    pub fn should_activate_kill_switch(&self, recent_trades: &[TradeResult]) -> bool {
        let consecutive_losses = self.count_consecutive_losses(recent_trades);
        consecutive_losses >= self.max_consecutive_losses
    }

    fn count_consecutive_losses(&self, trades: &[TradeResult]) -> usize {
        let mut count = 0;
        for trade in trades.iter().rev() {
            if !trade.is_win {
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
