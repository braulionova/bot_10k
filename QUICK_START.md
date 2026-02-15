# 🚀 HyroTrader Bot - Guía de Inicio Rápido

## Bot de Trading Cripto para Bybit Demo/Testnet

Este bot está diseñado específicamente para el desafío HyroTrader con soporte para **Bybit en modo demo/testnet**.

---

## 📋 Requisitos Previos

1. **Rust 1.83 o superior**
   ```bash
   rustc --version
   # Si necesitas actualizar: rustup update
   ```

2. **Cuenta Bybit Testnet**
   - Regístrate en: https://testnet.bybit.com/
   - Obtén $10,000 USDT virtuales automáticamente

3. **API Keys de Bybit Testnet**
   - Ve a: https://testnet.bybit.com/app/user/api-management
   - Crea nuevas API keys con permisos de trading

---

## ⚡ Instalación y Configuración

### 1. Clonar el Repositorio (si no lo has hecho)

```bash
cd /home/nova/bot_10k
```

### 2. Configurar Variables de Entorno

Copia el archivo de ejemplo:
```bash
cp .env.example .env
```

Edita `.env` con tus credenciales de Bybit Testnet:

```bash
nano .env
```

Configuración mínima necesaria:

```env
# Exchange
EXCHANGE_TYPE=bybit
EXCHANGE_API_KEY=tu_api_key_de_bybit_testnet
EXCHANGE_API_SECRET=tu_api_secret_de_bybit_testnet
EXCHANGE_TESTNET=true

# Challenge (valores por defecto ya configurados)
INITIAL_CAPITAL=10000
TARGET_PROFIT_PERCENT=10
MIN_TRADING_DAYS=10

# Risk
RISK_PER_TRADE_BASE=0.5
RISK_PER_TRADE_MIN=0.3
RISK_PER_TRADE_MAX=1.0

# Telegram (opcional - dejar en false si no lo usas)
ENABLE_ALERTS=false
```

### 3. Compilar el Proyecto

```bash
cargo build --release
```

Esto tardará unos minutos la primera vez mientras descarga dependencias.

---

## 🎮 Uso del Bot

### Ejecutar en Modo Demo (Sin Trading Real)

```bash
cargo run --release
```

El bot se conectará a Bybit Testnet y:
- ✅ Obtendrá datos de mercado en tiempo real
- ✅ Calculará scores de confluencia
- ✅ Identificará oportunidades de trading
- ⚠️ NO ejecutará órdenes reales aún (en desarrollo)

### Ver Logs Detallados

```bash
RUST_LOG=info cargo run --release
```

Para debug más detallado:
```bash
RUST_LOG=debug cargo run --release
```

---

## 📊 Características Implementadas

### ✅ Completado

- **Conectores de Exchange**
  - ✅ Bybit Testnet/Demo
  - ✅ Binance Testnet (alternativo)

- **Indicadores Técnicos**
  - ✅ RSI (Relative Strength Index)
  - ✅ ATR (Average True Range)
  - ✅ ADX (Average Directional Index)
  - ✅ EMA (Exponential Moving Average)
  - ✅ Volume Analysis

- **Sistema de Confluencias**
  - ✅ Estructura del sistema (0-100 puntos)
  - ⚠️ Lógica de detección (en desarrollo)

- **Gestión de Riesgo Adaptativa**
  - ✅ Escalado dinámico 0.3% - 1.0%
  - ✅ Protección por rachas
  - ✅ Protección por drawdown

### 🚧 En Desarrollo

- **Entry System**
  - 🚧 Detección de zonas Fibonacci
  - 🚧 Validación multi-timeframe
  - 🚧 Confirmación M5

- **Position Management**
  - 🚧 Take Profits dinámicos
  - 🚧 Trailing stops
  - 🚧 Partial closes

- **Trade Execution**
  - 🚧 Colocación de órdenes real
  - 🚧 Gestión de posiciones abiertas
  - 🚧 Cancelación de órdenes

- **Monitoring**
  - 🚧 Alertas Telegram
  - 🚧 Dashboard web
  - 🚧 Métricas de performance

---

## 🧪 Testing

### Test de Conexión a Bybit

El bot automáticamente probará la conexión al iniciar:

```
✅ Exchange connection successful
   Fetched 10 candles for BTCUSDT
   Last close price: $50,234.50
```

Si ves esto, ¡todo está funcionando!

### Errores Comunes

**Error: "Failed to connect to exchange"**
- Verifica tus API keys en `.env`
- Asegúrate de usar keys de TESTNET, no de producción
- Revisa que `EXCHANGE_TESTNET=true`

**Error: "Bybit API returned error"**
- Las keys pueden no tener permisos suficientes
- Crea nuevas keys con permisos de "Trade"

---

## 🎯 Objetivos del Challenge

El bot está configurado para:
- 💰 **Capital inicial:** $10,000 (virtual en testnet)
- 🎯 **Objetivo:** $1,000 profit (10%)
- 📅 **Mínimo:** 10 días de trading
- ⚠️ **Max drawdown diario:** <5%
- 🛑 **Max drawdown total:** <10%
- 📊 **Posiciones:** Máximo 1 a la vez

---

## 📖 Arquitectura del Sistema

```
src/
├── main.rs              # Punto de entrada
├── bot.rs               # Orquestador principal
├── config.rs            # Configuración
├── types.rs             # Tipos + indicadores técnicos
├── exchange/
│   ├── mod.rs          # Trait común
│   ├── bybit.rs        # Connector Bybit ✅
│   └── binance.rs      # Connector Binance ✅
├── intelligence/
│   ├── confluence_scorer.rs   # Sistema 0-100 pts
│   ├── asset_ranker.rs        # Ranking diario
│   └── market_regime.rs       # Detección condiciones
├── risk_v2/
│   ├── adaptive_sizing.rs     # Riesgo 0.3-1.0%
│   ├── streak_detector.rs     # Racha win/loss
│   └── correlation_matrix.rs  # Anti-correlación
├── execution_v2/
│   ├── smart_entry.rs         # Zonas Fibonacci
│   ├── dynamic_tp.rs          # TP basados en ATR
│   └── news_calendar.rs       # Filtro noticias
└── monitoring/
    ├── performance_metrics.rs # Win rate, Sharpe, etc.
    ├── alert_system.rs        # Telegram
    └── health_checker.rs      # Autodiagnóstico
```

---

## 🔧 Troubleshooting

### El bot no inicia

1. Verifica que `.env` existe:
   ```bash
   ls -la .env
   ```

2. Verifica la versión de Rust:
   ```bash
   rustc --version  # debe ser >= 1.83
   ```

3. Limpia y recompila:
   ```bash
   cargo clean
   cargo build --release
   ```

### El bot se conecta pero no tradea

Esto es normal - el sistema de ejecución de órdenes aún está en desarrollo. Por ahora el bot:
- ✅ Obtiene datos de mercado
- ✅ Calcula indicadores
- ✅ Evalúa oportunidades
- ⚠️ NO ejecuta trades reales

---

## 📝 Próximos Pasos

1. **Completar detección de setups**
   - Implementar lógica de breakout
   - Implementar validación de retest
   - Integrar MACD y divergencias

2. **Implementar ejecución de órdenes**
   - Firma HMAC para Bybit API
   - Gestión de posiciones
   - Stop loss y take profit

3. **Backtesting**
   - Probar con 6 meses de datos históricos
   - Validar win rate ≥45%
   - Confirmar profit factor ≥1.5

4. **Paper Trading**
   - 10 días simulados
   - Validar todas las reglas del challenge
   - Ajustar parámetros si necesario

5. **Live Trading**
   - Iniciar con 0.3% risk
   - Máximo 1 trade/día primera semana
   - Monitoreo continuo

---

## 📞 Soporte

- **Issues**: GitHub Issues (cuando esté público)
- **Documentación**: Ver `/docs` y `CLAUDE.md`
- **Logs**: Revisa la salida del bot para debugging

---

## ⚠️ Advertencias

- 🚫 **NO uses API keys de producción** - solo testnet
- 🚫 **NO operes con dinero real** hasta completar backtesting
- ✅ **Usa solo Bybit Testnet** para desarrollo
- ✅ **Mantén tus API keys privadas** (no las commits a Git)

---

## 🎓 Recursos

- **Bybit Testnet**: https://testnet.bybit.com/
- **Bybit API Docs**: https://bybit-exchange.github.io/docs/v5/intro
- **Estrategia completa**: Ver `hyrotrader_strategy_improved.md`
- **Guía de implementación**: Ver `rust_implementation_guide.md`

---

**¡Buena suerte con el challenge! 🚀**
