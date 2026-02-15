# Project Structure

## Created Files

### Configuration
- `Cargo.toml` - Rust project manifest with dependencies
- `.env.example` - Environment variable template
- `.gitignore` - Git ignore patterns

### Documentation
- `README.md` - Project overview and setup instructions
- `CLAUDE.md` - Development guide for Claude Code
- `RUST_VERSION.md` - Rust version requirements and update instructions
- `PROJECT_STRUCTURE.md` - This file
- `hyrotrader_strategy_improved.md` - Trading strategy documentation (existing)
- `rust_implementation_guide.md` - Technical implementation guide (existing)

### Source Code Structure

```
src/
├── main.rs                          # Application entry point
├── lib.rs                           # Library root
├── config.rs                        # Configuration management (env variables)
├── types.rs                         # Common types (Candle, MarketData, TrendDirection, etc.)
│
├── intelligence/                    # Market analysis and decision making
│   ├── mod.rs
│   ├── confluence_scorer.rs         # Multi-signal scoring system (70/100 threshold)
│   ├── asset_ranker.rs              # Daily asset selection and ranking
│   └── market_regime.rs             # Market condition detection (trending/ranging/volatile)
│
├── risk_v2/                         # Risk management systems
│   ├── mod.rs
│   ├── adaptive_sizing.rs           # Dynamic position sizing (0.3-1.0%)
│   ├── streak_detector.rs           # Win/loss streak tracking
│   └── correlation_matrix.rs        # Asset correlation analysis
│
├── execution_v2/                    # Trade execution logic
│   ├── mod.rs
│   ├── smart_entry.rs               # Fibonacci zone entry validation
│   ├── dynamic_tp.rs                # ATR-based take profit levels
│   └── news_calendar.rs             # Economic news filter
│
└── monitoring/                      # Performance tracking and alerts
    ├── mod.rs
    ├── performance_metrics.rs       # Win rate, Sharpe ratio, drawdown tracking
    ├── alert_system.rs              # Telegram notifications
    └── health_checker.rs            # System health monitoring
```

## Module Descriptions

### Core Types (`types.rs`)
- `TrendDirection`: Long/Short/Neutral
- `Timeframe`: M1/M5/M15/H1/H4/D1
- `Candle`: OHLCV data structure
- `MarketData`: Container for candle data and indicators

### Intelligence Module
- **ConfluenceScorer**: Evaluates trade setups using multi-signal analysis
  - Primary signals (40 pts): Breakout + Retest
  - Secondary signals (35 pts): Volume, RSI, MACD
  - Time confluences (25 pts): Multi-timeframe alignment, session liquidity

- **AssetRanker**: Scores assets based on volatility, trend, liquidity, spread
- **MarketRegimeDetector**: Identifies market conditions for safe trading

### Risk Management Module
- **AdaptiveRiskManager**: Adjusts position size (0.3-1.0%) based on:
  - Account profit percentage
  - Current drawdown
  - Recent win/loss streaks

- **StreakDetector**: Tracks consecutive wins/losses for risk reduction
- **CorrelationMatrix**: Prevents correlated positions

### Execution Module
- **SmartEntryManager**: Validates entries using Fibonacci retracement zones
  - Premium zone: 61.8-78.6% (best R:R)
  - Standard zone: 50-61.8%
  - Marginal zone: 38.2-50% (high confluence required)

- **DynamicTPManager**: Sets take profits based on ATR
  - TP1: 1.5R (40% position)
  - TP2: 2.5R (40% position)
  - TP3: 4.0R (20% with trailing stop)

- **NewsCalendar**: Blocks trading around high-impact economic events

### Monitoring Module
- **MetricsCalculator**: Calculates performance statistics
- **TelegramAlerter**: Sends trade and alert notifications
- **HealthChecker**: System diagnostics and kill-switch conditions

## Implementation Status

🟡 **Structure Complete, Implementation Pending**

All module files are created with:
- ✅ Type definitions
- ✅ Struct declarations
- ✅ Method signatures
- ⚠️  TODO markers for actual implementation logic

## Next Steps

1. Update Rust to version 1.83+ (see `RUST_VERSION.md`)
2. Implement technical indicators in `types.rs` (RSI, ATR, ADX, EMA, MACD)
3. Complete confluence scoring logic
4. Implement Fibonacci zone calculations
5. Add exchange API integration
6. Build backtesting framework
7. Add unit tests for each module

## Testing

```bash
# Run tests (once implemented)
cargo test

# Run with logging
RUST_LOG=info cargo run

# Build release version
cargo build --release
```
