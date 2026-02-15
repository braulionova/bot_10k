pub mod adaptive_sizing;
pub mod streak_detector;
pub mod correlation_matrix;

pub use adaptive_sizing::{AdaptiveRiskManager, TradeResult};
pub use streak_detector::StreakDetector;
pub use correlation_matrix::CorrelationMatrix;
