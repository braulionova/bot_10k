# 🤖 HyroTrader Bot - Setup Completado

## ✅ Estado Actual

### 1. **Bot en Rust - CONFIGURADO**

**Credenciales Configuradas:**
- ✅ Bybit Testnet API Key: `b11AtdufRHH8zIyHD5`
- ✅ Telegram Bot: `77004865221:AAFuu2ygokFNesm1uB6_JM96KxQwcc4q-dk`
- ✅ Telegram Chat ID: `483428397`
- ✅ Modo: TESTNET (sin riesgo real)

**Archivo `.env` creado y configurado** ✅

---

## 🚀 Cómo Ejecutar el Bot

### Opción 1: Ejecución Manual

```bash
cd /home/nova/bot_10k

# Compilar (si no está compilado)
cargo build --release

# Ejecutar
./target/release/hyrotrader-bot
```

### Opción 2: Como Servicio Systemd (RECOMENDADO)

El bot se ejecutará continuamente en segundo plano:

```bash
# 1. Instalar el servicio
./install_service.sh

# 2. Iniciar el bot
sudo systemctl start hyrotrader

# 3. Ver el estado
sudo systemctl status hyrotrader

# 4. Ver logs en tiempo real
journalctl -u hyrotrader -f
```

**Comandos del servicio:**
```bash
sudo systemctl start hyrotrader    # Iniciar
sudo systemctl stop hyrotrader     # Detener
sudo systemctl restart hyrotrader  # Reiniciar
sudo systemctl status hyrotrader   # Ver estado
```

---

## 📊 Funcionalidades Implementadas

### ✅ Core del Bot
- [x] Conexión a Bybit Testnet
- [x] Obtención de datos de mercado (klines/velas)
- [x] Indicadores técnicos (RSI, ATR, ADX, EMA)
- [x] Sistema de confluencias (0-100 puntos)
- [x] Gestión de riesgo adaptativa (0.3% - 1.0%)
- [x] Alertas por Telegram

### 🚧 En Desarrollo
- [ ] Ejecución real de órdenes (firmado HMAC)
- [ ] Gestión de posiciones abiertas
- [ ] Trailing stops dinámicos
- [ ] Backtesting completo

### 📝 Por Implementar
- [ ] Dashboard web
- [ ] Filtro de noticias económicas (API externa)
- [ ] Selección dinámica de activos
- [ ] Histórico de trades

---

## 🎯 Reglas del Challenge

El bot está configurado para el **HyroTrader Challenge**:

- **Capital inicial:** $10,000 (virtual en testnet)
- **Objetivo:** +$1,000 (10% profit)
- **Mínimo días:** 10 días de trading
- **Max drawdown diario:** < 5%
- **Max drawdown total:** < 10%
- **Max posiciones:** 1 a la vez
- **Kill-switch:** 4 pérdidas consecutivas

---

## 📱 Alertas de Telegram

El bot enviará notificaciones a tu Telegram cuando:

- ✅ Bot inicia/detiene
- ✅ Trade abierto (con score, precio, R:R)
- ✅ Trade cerrado (con P&L)
- ⚠️ Alertas importantes (drawdown, kill-switch)
- 📊 Resumen diario

**Configuración actual:**
- Token: Configurado ✅
- Chat ID: Configurado ✅
- Alertas: **HABILITADAS** ✅

---

## 🔍 Monitoreo

### Ver Logs del Bot

**Si ejecutas manualmente:**
```bash
# Los logs se muestran en la terminal
./target/release/hyrotrader-bot
```

**Si ejecutas como servicio:**
```bash
# Ver logs en tiempo real
journalctl -u hyrotrader -f

# Ver últimos 100 logs
journalctl -u hyrotrader -n 100

# Ver logs del día
journalctl -u hyrotrader --since today
```

### Verificar Estado del Servicio

```bash
sudo systemctl status hyrotrader
```

Verás:
- ● **active (running)** = Bot funcionando ✅
- ● **inactive (dead)** = Bot detenido ⏸️
- ● **failed** = Error, revisar logs ❌

---

## 🧪 Testing con Bybit Testnet

### API Testnet de Bybit

**URL:** https://testnet.bybit.com/

**Features:**
- ✅ $10,000 USDT virtuales gratis
- ✅ Datos de mercado en tiempo real
- ✅ Sin riesgo (dinero virtual)
- ✅ Misma API que producción

### Obtener más fondos virtuales

Si gastas los $10,000 testnet:
1. Ve a https://testnet.bybit.com/
2. Login
3. Ve a "Assets" → "Reset Balance"

---

## 📂 Estructura de Archivos

```
/home/nova/bot_10k/
├── .env                     # Configuración (API keys, etc.)
├── .env.example             # Template de configuración
├── Cargo.toml               # Dependencias Rust
├── QUICK_START.md           # Guía rápida
├── SETUP_COMPLETO.md        # Este archivo
├── hyrotrader.service       # Archivo de servicio systemd
├── install_service.sh       # Script de instalación
├── src/
│   ├── main.rs             # Punto de entrada
│   ├── bot.rs              # Lógica principal del bot
│   ├── config.rs           # Gestión de configuración
│   ├── types.rs            # Tipos + indicadores técnicos
│   ├── exchange/
│   │   ├── mod.rs
│   │   ├── bybit.rs        # Connector Bybit
│   │   └── binance.rs      # Connector Binance
│   ├── intelligence/
│   │   ├── confluence_scorer.rs
│   │   ├── asset_ranker.rs
│   │   └── market_regime.rs
│   ├── risk_v2/
│   │   ├── adaptive_sizing.rs
│   │   ├── streak_detector.rs
│   │   └── correlation_matrix.rs
│   ├── execution_v2/
│   │   ├── smart_entry.rs
│   │   ├── dynamic_tp.rs
│   │   └── news_calendar.rs
│   └── monitoring/
│       ├── performance_metrics.rs
│       ├── alert_system.rs
│       └── health_checker.rs
└── target/
    └── release/
        └── hyrotrader-bot   # Binario ejecutable
```

---

## ⚙️ Configuración Avanzada

### Cambiar Parámetros de Trading

Edita el archivo `.env`:

```bash
nano .env
```

**Parámetros clave:**
```env
# Riesgo por trade
RISK_PER_TRADE_BASE=0.5      # 0.5% (conservador)
RISK_PER_TRADE_MIN=0.3       # Mínimo tras pérdidas
RISK_PER_TRADE_MAX=1.0       # Máximo en buenas rachas

# Confluencias
MIN_CONFLUENCE_SCORE=70      # Mínimo para tradear

# Exchange
EXCHANGE_TESTNET=true        # SIEMPRE true para demo
```

Reinicia el bot después de cambios:
```bash
sudo systemctl restart hyrotrader
```

---

## 🐛 Troubleshooting

### El bot no inicia

1. **Verificar compilación:**
   ```bash
   cargo build --release
   ```

2. **Verificar .env:**
   ```bash
   cat .env
   ```
   Asegúrate de que las API keys estén correctas.

3. **Ver logs de error:**
   ```bash
   journalctl -u hyrotrader -n 50
   ```

### No llegan alertas de Telegram

1. **Verificar bot token:**
   - Habla con @BotFather en Telegram
   - Verifica que el token esté correcto

2. **Verificar chat ID:**
   - Habla con @userinfobot para obtener tu ID
   - Compara con el valor en `.env`

3. **Test manual:**
   ```bash
   curl -X POST "https://api.telegram.org/bot7700486521:AAFuu2ygokFNesm1uB6_JM96KxQwcc4q-dk/sendMessage" \
     -d "chat_id=483428397" \
     -d "text=Test message"
   ```

### El bot tradea muy poco

Esto es normal - el bot es conservador:
- Requiere score ≥70 de confluencia
- Solo 1 posición a la vez
- Evita mercados volátiles
- Respeta horarios de trading

Para ver más actividad (en desarrollo):
- Reducir `MIN_CONFLUENCE_SCORE` (no recomendado)
- Habilitar más assets (SOLUSDT, BNBUSDT)

---

## 🎓 Próximos Pasos

### 1. **Verificar que el bot compile**
```bash
cd /home/nova/bot_10k
cargo build --release
```

### 2. **Probar ejecución manual primero**
```bash
./target/release/hyrotrader-bot
```
Deja correr 5-10 minutos, verifica que:
- ✅ Se conecta a Bybit
- ✅ Obtiene precios
- ✅ Calcula indicadores
- ✅ Envía mensaje de inicio a Telegram

### 3. **Instalar como servicio**
```bash
./install_service.sh
sudo systemctl start hyrotrader
```

### 4. **Monitorear**
```bash
journalctl -u hyrotrader -f
```

---

## 📞 Soporte

- **Logs:** `journalctl -u hyrotrader -f`
- **Documentación:** Ver `QUICK_START.md`
- **Estrategia:** Ver `hyrotrader_strategy_improved.md`

---

## ⚠️ Advertencias Importantes

1. **SOLO USAR BYBIT TESTNET**
   - Nunca uses API keys de producción
   - Verifica siempre `EXCHANGE_TESTNET=true`

2. **NO TOCAR API SECRETS**
   - No compartas tus API keys
   - No las subas a Git

3. **TRADING ES RIESGOSO**
   - Incluso en testnet, el bot está en desarrollo
   - No todas las funciones están implementadas
   - Supervisa el bot regularmente

---

## 🎉 ¡Listo!

Tu bot HyroTrader está configurado y listo para correr en Bybit Testnet con alertas de Telegram.

**Para iniciar:**
```bash
cd /home/nova/bot_10k
cargo build --release
sudo systemctl start hyrotrader
journalctl -u hyrotrader -f
```

¡Buena suerte con el challenge! 🚀
