use rust_decimal::Decimal;
use crate::types::{MarketData, TrendDirection};
use crate::intelligence::ConfluenceScorer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn calculate_fib_zone(
        &self,
        price: Decimal,
        high: Decimal,
        low: Decimal,
        direction: TrendDirection,
    ) -> FibZone {
        let range = high - low;

        let fib_382 = low + (range * Decimal::try_from(0.382).unwrap());
        let fib_500 = low + (range * Decimal::try_from(0.500).unwrap());
        let fib_618 = low + (range * Decimal::try_from(0.618).unwrap());
        let fib_786 = low + (range * Decimal::try_from(0.786).unwrap());

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
                let inv_618 = high - (range * Decimal::try_from(0.618).unwrap());
                let inv_500 = high - (range * Decimal::try_from(0.500).unwrap());
                let inv_382 = high - (range * Decimal::try_from(0.382).unwrap());
                let inv_786 = high - (range * Decimal::try_from(0.786).unwrap());

                if price <= inv_618 && price >= inv_786 {
                    FibZone::Premium
                } else if price <= inv_500 && price > inv_618 {
                    FibZone::Standard
                } else if price <= inv_382 && price > inv_500 {
                    FibZone::Marginal
                } else {
                    FibZone::Invalid
                }
            }
            _ => FibZone::Invalid,
        }
    }

    fn check_m5_confirmation(&self, _data: &MarketData, _direction: TrendDirection) -> bool {
        // TODO: Implement M5 confirmation logic
        false
    }
}

#[derive(Debug)]
pub struct EntrySignal {
    pub price: Decimal,
    pub fib_zone: FibZone,
    pub confluence_score: u8,
    pub direction: TrendDirection,
}
