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
    max_history: usize,
}

impl MetricsCalculator {
    pub fn new(max_history: usize) -> Self {
        Self {
            trades: VecDeque::new(),
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
            current_drawdown: 0.0,
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
            valid_trading_days: 0,
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
            mean / std_dev * (252.0_f64).sqrt()
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
