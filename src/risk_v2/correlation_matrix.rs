use std::collections::HashMap;

pub struct CorrelationMatrix {
    correlations: HashMap<(String, String), f64>,
}

impl CorrelationMatrix {
    pub fn new() -> Self {
        Self {
            correlations: HashMap::new(),
        }
    }

    pub fn calculate_correlation(&mut self, _symbol1: &str, _symbol2: &str) -> f64 {
        // TODO: Implement correlation calculation
        0.0
    }

    pub fn are_highly_correlated(&self, symbol1: &str, symbol2: &str, threshold: f64) -> bool {
        let key = (symbol1.to_string(), symbol2.to_string());
        self.correlations
            .get(&key)
            .map(|&corr| corr.abs() > threshold)
            .unwrap_or(false)
    }
}

impl Default for CorrelationMatrix {
    fn default() -> Self {
        Self::new()
    }
}
