# HyroTrader Bot v2.0

A sophisticated crypto trading bot designed for the HyroTrader challenge.

## Goal

Achieve 10% profit ($1,000) from $10,000 initial capital in minimum 10 trading days while respecting strict risk constraints.

## Project Status

🚧 **In Development** - Core structure implemented, modules need completion.

## Features

- **Multi-Confirmation System**: Confluence scoring (0-100) with minimum threshold of 70
- **Adaptive Risk Management**: Dynamic position sizing (0.3-1.0%) based on performance
- **Smart Entry System**: Fibonacci retracement zones with validation
- **Dynamic Take Profits**: ATR-based targets with trailing stops
- **Asset Ranking**: Daily scoring of crypto assets for optimal selection
- **News Filter**: Economic calendar integration to avoid high-impact events
- **Performance Tracking**: Real-time metrics (win rate, Sharpe ratio, drawdowns)
- **Telegram Alerts**: Notifications for trades and critical events

## Requirements

- **Rust 1.83 or newer** - See `RUST_VERSION.md` if you need to update Rust
- Exchange API credentials (Binance, etc.)
- Optional: Telegram bot token for alerts

## Setup

1. **Update Rust** (if needed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. Copy `.env.example` to `.env` and configure:
   ```bash
   cp .env.example .env
   ```

3. Edit `.env` with your API keys and preferences

4. Build the project:
   ```bash
   cargo build --release
   ```

5. Run (when implementation is complete):
   ```bash
   cargo run --release
   ```

## Testing

```bash
cargo test
```

## Project Structure

```
src/
├── main.rs                    # Entry point
├── lib.rs                     # Library root
├── config.rs                  # Configuration management
├── types.rs                   # Common types
├── intelligence/              # Analysis modules
│   ├── confluence_scorer.rs   # Multi-signal scoring
│   ├── asset_ranker.rs        # Asset selection
│   └── market_regime.rs       # Market condition detection
├── risk_v2/                   # Risk management
│   ├── adaptive_sizing.rs     # Dynamic position sizing
│   ├── streak_detector.rs     # Win/loss streak tracking
│   └── correlation_matrix.rs  # Asset correlation
├── execution_v2/              # Trade execution
│   ├── smart_entry.rs         # Entry logic with Fib zones
│   ├── dynamic_tp.rs          # Take profit management
│   └── news_calendar.rs       # Economic events filter
└── monitoring/                # Tracking & alerts
    ├── performance_metrics.rs # Performance calculation
    ├── alert_system.rs        # Telegram notifications
    └── health_checker.rs      # System health monitoring
```

## Safety Features

- **Kill-Switch**: Auto-stop at 4 consecutive losses or 8% total drawdown
- **Drawdown Protection**: Reduces risk when drawdown exceeds 3%
- **Streak Protection**: Automatically reduces position size after 2 consecutive losses
- **Market Filters**: No trading during extreme volatility, low liquidity, or high-impact news

## Documentation

- `CLAUDE.md` - Development guide for Claude Code
- `hyrotrader_strategy_improved.md` - Complete strategy documentation
- `rust_implementation_guide.md` - Technical implementation details

## License

Private - Not for distribution

## Warning

⚠️ **This is experimental software. Do not use with real funds until thoroughly backtested and validated.**
