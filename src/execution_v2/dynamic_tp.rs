use rust_decimal::Decimal;
use crate::types::TrendDirection;

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

        let trailing_distance = atr * Decimal::try_from(1.5).unwrap();

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
        let target_distance = sl_distance * Decimal::try_from(multiplier).unwrap();

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
            tracing::info!("Trailing stop updated: {}", new_level);
        }
    }
}

impl Default for DynamicTPManager {
    fn default() -> Self {
        Self::new()
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
