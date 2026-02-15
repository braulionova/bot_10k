# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a crypto trading bot designed for the HyroTrader challenge, aiming to achieve 10% profit ($1,000 from $10,000 capital) in minimum 10 trading days while respecting strict risk constraints.

**Current Status:** Planning/Design phase - no implementation code exists yet, only strategy documentation and Rust implementation guides.

## Challenge Constraints (IMMUTABLE)

- **Initial Capital:** $10,000
- **Target Profit:** 10% ($1,000)
- **Minimum Trading Days:** 10
- **Maximum Daily Drawdown:** <5%
- **Maximum Total Drawdown:** <10%
- **Maximum Open Positions:** 1 at a time
- **Risk Per Trade:** 0.3-1.0% (adaptive based on performance)

## Core Architecture

The system uses a multi-layer confirmation approach before entering trades:

### Confluence Scoring System (70/100 minimum to trade)
- Primary signals (40 pts): Breakout confirmation + retest validation
- Secondary signals (35 pts): Volume spike, RSI zones, MACD divergence
- Time confluences (25 pts): Multi-timeframe alignment, trading session liquidity

### Key Modules to Implement (Priority Order)

1. **confluence_scorer.rs** - Calculates 0-100 score for trade setups
2. **adaptive_sizing.rs** - Adjusts risk 0.3-1.0% based on account state and win/loss streaks
3. **smart_entry.rs** - Validates entries using Fibonacci retracement zones (Premium/Standard/Marginal)
4. **dynamic_tp.rs** - Sets take profits at 1.5R (40%), 2.5R (40%), 4.0R (20% with trailing)
5. **asset_ranker.rs** - Scores assets daily on volatility, trend strength, liquidity, spread
6. **news_calendar.rs** - Blocks trading 1h before and 2h after high-impact news
7. **performance_metrics.rs** - Tracks win rate, Sharpe ratio, profit factor, drawdowns
8. **alert_system.rs** - Sends Telegram notifications for trades and alerts

### Risk Management Rules

- **Streak Protection:** 2 consecutive losses → reduce to 0.3% risk for 3 trades
- **Drawdown Protection:** If drawdown >3%, reduce to 0.3% risk
- **Progressive Scaling:**
  - $10,000-$10,300: 0.5% risk
  - $10,300-$10,600: 0.7% risk
  - $10,600-$11,000: 1.0% risk

### Market Filters

**DO NOT TRADE when:**
- ATR(14) > 2.5 × ATR(50) (extreme volatility)
- Spread > 0.05%
- Order book depth < $500k in top 10 levels
- Weekends (Sat 22:00 - Mon 02:00 UTC)
- Within blackout window of high-impact news events

**ONLY TRADE when:**
- ADX > 25 (clear trend)
- Confluence score ≥ 70
- Multi-timeframe alignment (H1 + M15 + M5)

## Asset Selection

- **Always monitor:** BTCUSDT, ETHUSDT
- **Conditionally add:** SOLUSDT, BNBUSDT (if daily score >75)
- **Maximum simultaneous:** 2 active assets
- Assets ranked daily by: `0.4×volatility + 0.3×trend + 0.2×liquidity + 0.1×spread`

## Target Metrics

**Before Going Live (Backtesting Requirements):**
- Win Rate: ≥45%
- Profit Factor: ≥1.5
- Max Drawdown: ≤8%
- Sharpe Ratio: ≥1.2
- Backtesting Period: Minimum 6 months historical data
- Include 0.03% slippage + 0.055% fees in simulations

**Live Trading Objectives:**
- Win Rate: 50-60%
- Average Risk:Reward: 1:2.2
- Max Consecutive Losses: ≤3
- Sharpe Ratio: >1.5
- Profit Factor: >1.8

## Implementation Philosophy

**Conservatism First:** When in doubt, DO NOT trade. Missing a trade is better than a bad trade.

**No Over-Engineering:**
- Use proven indicators (EMA, ADX, RSI) over complex ML models
- Maximum 7 conditions per setup
- Round parameters (20, 50, 200... not 47.3)

**Kill-Switch Conditions:** Immediately stop trading if:
- Daily loss exceeds 1.0%
- Total drawdown exceeds 8.0%
- Consecutive losses reach 4

## References

See `hyrotrader_strategy_improved.md` for complete strategy details and `rust_implementation_guide.md` for technical implementation examples in Rust.
