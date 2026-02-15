#!/bin/bash
echo "🔄 Reiniciando HyroTrader Bot..."
sudo systemctl restart hyrotrader
sleep 2
echo ""
echo "✅ Estado del servicio:"
sudo systemctl status hyrotrader --no-pager -l
echo ""
echo "📜 Últimos logs:"
journalctl -u hyrotrader -n 20 --no-pager
