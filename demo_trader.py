#!/usr/bin/env python3
"""
HyroTrader Bot - Demo Mode
Simulador de trading paper con datos reales de Bybit
"""

import requests
import time
import json
from datetime import datetime
from typing import Dict, List, Optional

class BybitDemo:
    def __init__(self):
        self.base_url = "https://api-testnet.bybit.com"
        self.balance = 10000.0  # $10,000 inicial
        self.initial_balance = 10000.0
        self.positions = []
        self.trade_history = []

    def get_klines(self, symbol: str = "BTCUSDT", interval: str = "5", limit: int = 100):
        """Obtener velas del mercado"""
        url = f"{self.base_url}/v5/market/kline"
        params = {
            "category": "linear",
            "symbol": symbol,
            "interval": interval,
            "limit": limit
        }

        try:
            response = requests.get(url, params=params)
            data = response.json()

            if data["retCode"] == 0:
                return data["result"]["list"]
            else:
                print(f"❌ Error: {data['retMsg']}")
                return []
        except Exception as e:
            print(f"❌ Error fetching klines: {e}")
            return []

    def get_ticker(self, symbol: str = "BTCUSDT"):
        """Obtener precio actual"""
        url = f"{self.base_url}/v5/market/tickers"
        params = {
            "category": "linear",
            "symbol": symbol
        }

        try:
            response = requests.get(url, params=params)
            data = response.json()

            if data["retCode"] == 0 and len(data["result"]["list"]) > 0:
                ticker = data["result"]["list"][0]
                return {
                    "price": float(ticker["lastPrice"]),
                    "high24h": float(ticker["highPrice24h"]),
                    "low24h": float(ticker["lowPrice24h"]),
                    "volume24h": float(ticker["volume24h"]),
                }
            return None
        except Exception as e:
            print(f"❌ Error fetching ticker: {e}")
            return None

    def calculate_rsi(self, klines: List, period: int = 14) -> float:
        """Calcular RSI simplificado"""
        if len(klines) < period + 1:
            return 50.0

        closes = [float(k[4]) for k in klines[:period+1]]

        gains = 0.0
        losses = 0.0

        for i in range(1, len(closes)):
            change = closes[i] - closes[i-1]
            if change > 0:
                gains += change
            else:
                losses += abs(change)

        if losses == 0:
            return 100.0

        avg_gain = gains / period
        avg_loss = losses / period
        rs = avg_gain / avg_loss
        rsi = 100 - (100 / (1 + rs))

        return rsi

    def analyze_market(self, symbol: str = "BTCUSDT"):
        """Analizar el mercado y buscar oportunidades"""
        print(f"\n📊 Analizando {symbol}...")

        # Obtener datos
        ticker = self.get_ticker(symbol)
        if not ticker:
            return None

        klines = self.get_klines(symbol, "5", 100)
        if not klines:
            return None

        # Calcular indicadores
        current_price = ticker["price"]
        rsi = self.calculate_rsi(klines)

        # Calcular ATR simple
        atr_periods = []
        for i in range(min(14, len(klines))):
            high = float(klines[i][2])
            low = float(klines[i][3])
            atr_periods.append(high - low)
        atr = sum(atr_periods) / len(atr_periods) if atr_periods else 0

        print(f"  💰 Precio: ${current_price:,.2f}")
        print(f"  📈 High 24h: ${ticker['high24h']:,.2f}")
        print(f"  📉 Low 24h: ${ticker['low24h']:,.2f}")
        print(f"  📊 Volumen 24h: {ticker['volume24h']:,.2f} BTC")
        print(f"  🎯 RSI(14): {rsi:.1f}")
        print(f"  📏 ATR: ${atr:.2f}")

        # Señales de trading (simplificado)
        confluence_score = 0
        signals = []

        # RSI en zona favorable
        if 40 < rsi < 70:
            confluence_score += 10
            signals.append("✅ RSI en zona alcista")
        elif 30 < rsi < 60:
            confluence_score += 10
            signals.append("✅ RSI en zona bajista")

        # Volumen significativo
        if ticker['volume24h'] > 10000:
            confluence_score += 10
            signals.append("✅ Volumen alto")

        # Volatilidad moderada
        if 100 < atr < 2000:
            confluence_score += 10
            signals.append("✅ Volatilidad moderada")

        print(f"\n  🎲 Score de Confluencia: {confluence_score}/100")
        for signal in signals:
            print(f"     {signal}")

        if confluence_score < 70:
            print(f"  ⚠️  Score insuficiente (mínimo 70)")

        return {
            "price": current_price,
            "rsi": rsi,
            "atr": atr,
            "score": confluence_score,
            "signals": signals
        }

    def show_balance(self):
        """Mostrar balance actual"""
        profit = self.balance - self.initial_balance
        profit_pct = (profit / self.initial_balance) * 100

        print("\n" + "="*60)
        print("💼 ESTADO DE LA CUENTA")
        print("="*60)
        print(f"  Balance inicial:  ${self.initial_balance:,.2f}")
        print(f"  Balance actual:   ${self.balance:,.2f}")
        print(f"  P&L:              ${profit:+,.2f} ({profit_pct:+.2f}%)")
        print(f"  Objetivo:         ${self.initial_balance * 1.10:,.2f} (+10%)")
        print(f"  Trades:           {len(self.trade_history)}")
        print("="*60)

    def run_demo(self, duration_minutes: int = 5):
        """Ejecutar demo trading"""
        print("\n" + "🚀"*30)
        print("  HYROTRADER BOT - DEMO MODE")
        print("  Paper Trading con datos reales de Bybit Testnet")
        print("🚀"*30)

        self.show_balance()

        symbols = ["BTCUSDT", "ETHUSDT"]
        interval = 60  # segundos entre análisis

        print(f"\n⏰ Demo correrá por {duration_minutes} minutos")
        print(f"📊 Analizando: {', '.join(symbols)}")
        print(f"🔄 Actualización cada {interval} segundos")
        print("\n💡 Presiona Ctrl+C para detener\n")

        start_time = time.time()
        end_time = start_time + (duration_minutes * 60)

        try:
            cycle = 0
            while time.time() < end_time:
                cycle += 1
                print(f"\n{'='*60}")
                print(f"🔄 Ciclo #{cycle} - {datetime.now().strftime('%H:%M:%S')}")
                print(f"{'='*60}")

                for symbol in symbols:
                    analysis = self.analyze_market(symbol)
                    time.sleep(2)  # Evitar rate limiting

                self.show_balance()

                remaining = int(end_time - time.time())
                if remaining > 0:
                    print(f"\n⏳ Próxima actualización en {interval}s (quedan {remaining//60}m {remaining%60}s)")
                    time.sleep(interval)

        except KeyboardInterrupt:
            print("\n\n⚠️  Demo detenido por el usuario")

        print("\n" + "="*60)
        print("📊 RESUMEN FINAL")
        print("="*60)
        self.show_balance()
        print("\n✅ Demo completado\n")

if __name__ == "__main__":
    demo = BybitDemo()
    demo.run_demo(duration_minutes=10)  # 10 minutos de demo
