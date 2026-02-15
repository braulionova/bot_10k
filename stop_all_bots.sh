#!/bin/bash

echo "🛑 Deteniendo todos los bots de trading..."
echo ""

# Detener scripts de bash
echo "📍 Deteniendo scripts de bash..."
sudo pkill -9 -f "start_bot.sh"
sudo pkill -9 -f "start_challenge_bot.sh"

# Detener bots de Python
echo "📍 Deteniendo bots de Python..."
sudo pkill -9 -f "bot_nova_v1.py"
sudo pkill -9 -f "bot_challenge_4h.py"
sudo pkill -9 -f "demo_trader.py"

# Detener bots de Rust
echo "📍 Deteniendo bots de Rust..."
sudo pkill -9 bot_order_book
sudo pkill -9 -f "bot_margin_v2"
sudo pkill -9 -f "hyrotrader-bot"

# Detener cualquier cargo run
echo "📍 Deteniendo cargo run..."
sudo pkill -9 -f "cargo run"

sleep 2

echo ""
echo "✅ Bots detenidos. Verificando..."
echo ""

# Verificar
REMAINING=$(ps aux | grep -E "(bot_|hyrotrader|cargo run)" | grep -v grep | grep -v "stop_all_bots" | wc -l)

if [ $REMAINING -eq 0 ]; then
    echo "✅ Todos los bots han sido detenidos correctamente"
else
    echo "⚠️  Aún hay $REMAINING procesos corriendo:"
    ps aux | grep -E "(bot_|hyrotrader|cargo run)" | grep -v grep | grep -v "stop_all_bots"
fi

echo ""
echo "Para ver procesos activos:"
echo "  ps aux | grep bot"
