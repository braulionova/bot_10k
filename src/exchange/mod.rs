pub mod binance;
pub mod bybit;

pub use binance::BinanceConnector;
pub use bybit::BybitConnector;

use anyhow::Result;
use crate::types::{MarketData, Candle, Timeframe};
use async_trait::async_trait;

#[async_trait]
pub trait ExchangeConnector: Send + Sync {
    async fn get_market_data(&self, symbol: &str, timeframe: Timeframe, limit: usize) -> Result<MarketData>;
    async fn place_order(&self, symbol: &str, side: OrderSide, quantity: f64, price: Option<f64>) -> Result<Order>;
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()>;
    async fn get_account_balance(&self) -> Result<AccountBalance>;
    async fn get_open_orders(&self, symbol: &str) -> Result<Vec<Order>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: OrderStatus,
    pub filled_quantity: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct AccountBalance {
    pub total_balance_usdt: f64,
    pub available_balance_usdt: f64,
    pub positions: Vec<Position>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub side: OrderSide,
}
