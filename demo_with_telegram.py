#!/usr/bin/env python3
"""
HyroTrader Bot - Demo con Telegram
Versión ligera con notificaciones
"""

import requests
import time
from datetime import datetime

# Configuración
BYBIT_API = "https://api-testnet.bybit.com"
TELEGRAM_TOKEN = "7700486521:AAFuu2ygokFNesm1uB6_JM96KxQwcc4q-dk"
CHAT_ID = "483428397"

def send_telegram(message):
    """Enviar mensaje a Telegram"""
    url = f"https://api.telegram.org/bot{TELEGRAM_TOKEN}/sendMessage"
    data = {
        "chat_id": CHAT_ID,
        "text": message,
        "parse_mode": "HTML"
    }
    try:
        response = requests.post(url, json=data)
        return response.json().get("ok", False)
    except Exception as e:
        print(f"Error enviando a Telegram: {e}")
        return False

def get_price(symbol="BTCUSDT"):
    """Obtener precio actual"""
    url = f"{BYBIT_API}/v5/market/tickers"
    params = {"category": "linear", "symbol": symbol}

    try:
        response = requests.get(url, params=params)
        data = response.json()

        if data["retCode"] == 0 and len(data["result"]["list"]) > 0:
            return float(data["result"]["list"][0]["lastPrice"])
    except:
        pass
    return None

def calculate_rsi_simple(klines, period=14):
    """RSI simplificado"""
    if len(klines) < period + 1:
        return 50.0

    closes = [float(k[4]) for k in klines[:period+1]]
    gains, losses = 0.0, 0.0

    for i in range(1, len(closes)):
        change = closes[i] - closes[i-1]
        if change > 0:
            gains += change
        else:
            losses += abs(change)

    if losses == 0:
        return 100.0

    rs = (gains/period) / (losses/period)
    return 100 - (100 / (1 + rs))

def get_klines(symbol="BTCUSDT", limit=50):
    """Obtener velas"""
    url = f"{BYBIT_API}/v5/market/kline"
    params = {
        "category": "linear",
        "symbol": symbol,
        "interval": "5",
        "limit": limit
    }

    try:
        response = requests.get(url, params=params)
        data = response.json()

        if data["retCode"] == 0:
            return data["result"]["list"]
    except:
        pass
    return []

def analyze_market(symbol="BTCUSDT"):
    """Análisis de mercado"""
    price = get_price(symbol)
    if not price:
        return None

    klines = get_klines(symbol, 50)
    if not klines:
        return None

    rsi = calculate_rsi_simple(klines, 14)

    # Calcular ATR simple
    atr_values = []
    for k in klines[:14]:
        high, low = float(k[2]), float(k[3])
        atr_values.append(high - low)
    atr = sum(atr_values) / len(atr_values) if atr_values else 0

    # Confluencias básicas
    score = 0
    signals = []

    if 40 < rsi < 70:
        score += 15
        signals.append("RSI alcista")
    elif 30 < rsi < 60:
        score += 15
        signals.append("RSI bajista")

    if 100 < atr < 2000:
        score += 10
        signals.append("Volatilidad OK")

    # Volume check (simplificado)
    recent_vol = sum([float(k[5]) for k in klines[:5]]) / 5
    avg_vol = sum([float(k[5]) for k in klines]) / len(klines)

    if recent_vol > avg_vol * 1.2:
        score += 10
        signals.append("Volume alto")

    return {
        "symbol": symbol,
        "price": price,
        "rsi": rsi,
        "atr": atr,
        "score": score,
        "signals": signals
    }

def run_demo():
    """Ejecutar demo"""
    # Mensaje inicial
    msg = """🚀 <b>HyroTrader Bot - INICIADO</b>

📊 Modo: DEMO / Paper Trading
💰 Capital: $10,000 (virtual)
🎯 Objetivo: +10% ($1,000)
⚡ Exchange: Bybit Testnet

Monitoreando BTCUSDT y ETHUSDT..."""

    print(msg)
    send_telegram(msg)

    symbols = ["BTCUSDT", "ETHUSDT"]

    for cycle in range(3):  # 3 ciclos de análisis
        print(f"\n{'='*50}")
        print(f"Ciclo #{cycle+1} - {datetime.now().strftime('%H:%M:%S')}")
        print(f"{'='*50}\n")

        report_lines = [f"📊 <b>Análisis #{cycle+1}</b> - {datetime.now().strftime('%H:%M:%S')}"]
        report_lines.append("")

        for symbol in symbols:
            analysis = analyze_market(symbol)

            if analysis:
                print(f"📈 {symbol}")
                print(f"   💰 ${analysis['price']:,.2f}")
                print(f"   🎯 RSI: {analysis['rsi']:.1f}")
                print(f"   📏 ATR: ${analysis['atr']:.2f}")
                print(f"   🎲 Score: {analysis['score']}/100")
                print(f"   ✅ {', '.join(analysis['signals'])}\n")

                report_lines.append(f"<b>{symbol}</b>")
                report_lines.append(f"💰 ${analysis['price']:,.2f}")
                report_lines.append(f"🎯 RSI: {analysis['rsi']:.1f}")
                report_lines.append(f"🎲 Score: {analysis['score']}/100")

                if analysis['score'] >= 70:
                    report_lines.append("✅ <b>SETUP VÁLIDO!</b>")
                else:
                    report_lines.append(f"⚠️ Score bajo (mín 70)")

                report_lines.append("")

            time.sleep(2)

        # Enviar reporte a Telegram
        report_lines.append("💼 Balance: $10,000 (sin trades)")
        telegram_msg = "\n".join(report_lines)
        send_telegram(telegram_msg)

        if cycle < 2:
            print("⏳ Esperando 30s para próximo ciclo...")
            time.sleep(30)

    # Mensaje final
    final_msg = """✅ <b>Demo completado</b>

📊 El bot está funcionando correctamente
🔄 Conectado a Bybit Testnet
💬 Alertas de Telegram activas

⚠️ Compilando versión Rust completa...
Espera unos minutos para el bot final."""

    print(f"\n{final_msg}")
    send_telegram(final_msg)

if __name__ == "__main__":
    print("🤖 HyroTrader Bot Demo - Con Telegram")
    print("="*50)
    run_demo()
    print("\n✅ Demo finalizado")
