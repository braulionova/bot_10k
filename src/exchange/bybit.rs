use super::*;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct BybitConnector {
    client: Client,
    api_key: String,
    api_secret: String,
    testnet: bool,
}

impl BybitConnector {
    pub fn new(api_key: String, api_secret: String, testnet: bool) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
            testnet,
        }
    }

    fn base_url(&self) -> &str {
        if self.testnet {
            "https://api-testnet.bybit.com"
        } else {
            "https://api.bybit.com"
        }
    }

    fn timeframe_to_interval(&self, timeframe: Timeframe) -> &str {
        match timeframe {
            Timeframe::M1 => "1",
            Timeframe::M5 => "5",
            Timeframe::M15 => "15",
            Timeframe::M30 => "30",
            Timeframe::H1 => "60",
            Timeframe::H4 => "240",
            Timeframe::D1 => "D",
        }
    }
}

#[async_trait]
impl ExchangeConnector for BybitConnector {
    async fn get_market_data(&self, symbol: &str, timeframe: Timeframe, limit: usize) -> Result<MarketData> {
        let interval = self.timeframe_to_interval(timeframe);
        let url = format!("{}/v5/market/kline", self.base_url());

        tracing::info!("Fetching {} {} candles from Bybit...", symbol, interval);

        let response = self.client
            .get(&url)
            .query(&[
                ("category", "linear"), // USDT perpetuals
                ("symbol", symbol),
                ("interval", interval),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(anyhow!("Bybit API error: {} - {}", status, text));
        }

        let api_response: BybitKlineResponse = serde_json::from_str(&text)
            .map_err(|e| anyhow!("Failed to parse Bybit response: {} - Response: {}", e, text))?;

        if api_response.ret_code != 0 {
            return Err(anyhow!("Bybit API returned error: {}", api_response.ret_msg));
        }

        let candles: Vec<Candle> = api_response.result.list
            .into_iter()
            .map(|k| Candle {
                timestamp: k.0.parse::<i64>().unwrap_or(0),
                open: Decimal::from_str(&k.1).unwrap_or(Decimal::ZERO),
                high: Decimal::from_str(&k.2).unwrap_or(Decimal::ZERO),
                low: Decimal::from_str(&k.3).unwrap_or(Decimal::ZERO),
                close: Decimal::from_str(&k.4).unwrap_or(Decimal::ZERO),
                volume: Decimal::from_str(&k.5).unwrap_or(Decimal::ZERO),
            })
            .collect();

        tracing::info!("Fetched {} candles for {}", candles.len(), symbol);

        Ok(MarketData {
            symbol: symbol.to_string(),
            candles,
            timeframe,
        })
    }

    async fn place_order(&self, symbol: &str, side: OrderSide, quantity: f64, price: Option<f64>) -> Result<Order> {
        // TODO: Implement Bybit order placement with proper signing
        // Bybit requires HMAC-SHA256 signature
        tracing::warn!("⚠️  Order placement not yet implemented for Bybit - returning mock order");
        tracing::info!("Mock order: {} {} {} @ {:?}",
            if matches!(side, OrderSide::Buy) { "BUY" } else { "SELL" },
            quantity,
            symbol,
            price
        );

        Ok(Order {
            id: format!("bybit_mock_{}", chrono::Utc::now().timestamp_millis()),
            symbol: symbol.to_string(),
            side,
            order_type: if price.is_some() { OrderType::Limit } else { OrderType::Market },
            quantity,
            price,
            status: OrderStatus::New,
            filled_quantity: 0.0,
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()> {
        tracing::info!("Canceling order {} for {}", order_id, symbol);
        // TODO: Implement order cancellation
        Ok(())
    }

    async fn get_account_balance(&self) -> Result<AccountBalance> {
        let url = format!("{}/v5/account/wallet-balance", self.base_url());

        // TODO: Implement proper authentication
        // For now, return demo balance
        tracing::debug!("Getting account balance (mock data for now)");

        Ok(AccountBalance {
            total_balance_usdt: 10000.0,
            available_balance_usdt: 10000.0,
            positions: Vec::new(),
        })
    }

    async fn get_open_orders(&self, _symbol: &str) -> Result<Vec<Order>> {
        // TODO: Implement open orders retrieval
        Ok(Vec::new())
    }
}

// Bybit API response types (v5 API)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitKlineResponse {
    ret_code: i32,
    ret_msg: String,
    result: BybitKlineResult,
}

#[derive(Debug, Deserialize)]
struct BybitKlineResult {
    category: String,
    symbol: String,
    list: Vec<BybitKline>,
}

#[derive(Debug, Deserialize)]
struct BybitKline(
    String, // Start time
    String, // Open
    String, // High
    String, // Low
    String, // Close
    String, // Volume
    String, // Turnover
);
