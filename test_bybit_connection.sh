#!/bin/bash

# Test Bybit API connection
API_KEY="b11AtdufRHH8zIyHD5"

echo "🔍 Testing Bybit Testnet API Connection..."
echo ""

# Test 1: Get server time
echo "1️⃣ Testing server time endpoint..."
response=$(curl -s "https://api-testnet.bybit.com/v5/market/time")
echo "Response: $response"
echo ""

# Test 2: Get BTCUSDT ticker
echo "2️⃣ Getting BTCUSDT ticker..."
response=$(curl -s "https://api-testnet.bybit.com/v5/market/tickers?category=linear&symbol=BTCUSDT")
echo "Response: $response"
echo ""

# Test 3: Get recent klines
echo "3️⃣ Getting BTCUSDT klines (last 5 candles, 5min)..."
response=$(curl -s "https://api-testnet.bybit.com/v5/market/kline?category=linear&symbol=BTCUSDT&interval=5&limit=5")
echo "Response: $response"
echo ""

echo "✅ API connection test complete!"
echo ""
echo "If you see JSON responses above, the Bybit testnet API is accessible."
echo "Now let's build and run the Rust bot..."
