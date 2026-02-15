# 📋 Bots Detectados en el Sistema

## 🤖 Bots Actualmente Corriendo:

### Python Bots (root)
1. **bot_nova_v1.py** - PID 1083481
   - Ubicación: `/root/bot_hyrotrader_v7/`
   - Script: `start_bot.sh`
   - Corriendo desde: Feb 09

2. **bot_challenge_4h.py** - PID 1137027
   - Ubicación: `/root/bot_hyrotrader_v7/`
   - Script: `start_challenge_bot.sh`
   - Corriendo desde: Feb 09

### Rust Bots (root)
3. **bot_order_book** - PID 1836463, 1894165 (2 instancias)
   - Ubicación: `/root/bot_order_book/`
   - Binario: `target/release/bot_order_book`
   - Corriendo desde: Feb 13

4. **bot_margin_v2** - PID 1942530
   - Ubicación: `/root/bot_margin_v2/`
   - Binario: `target/release/bot`
   - Corriendo desde: hoy 06:36

### Demo (nova)
5. **demo_trader.py** - PID 1967252
   - Ubicación: `/home/nova/bot_10k/`
   - Usuario: nova
   - Corriendo desde: 19:23

---

## 🛑 Cómo Detener TODOS los Bots

### Opción 1: Script Automático
```bash
cd /home/nova/bot_10k
./stop_all_bots.sh
```

### Opción 2: Manual (uno por uno)

**Detener Python bots:**
```bash
sudo pkill -9 -f "bot_nova_v1.py"
sudo pkill -9 -f "bot_challenge_4h.py"
```

**Detener Rust bots:**
```bash
sudo pkill -9 bot_order_book
sudo pkill -9 -f "bot_margin_v2"
```

**Detener scripts de inicio:**
```bash
sudo pkill -9 -f "start_bot.sh"
sudo pkill -9 -f "start_challenge_bot.sh"
```

---

## ✅ Verificar que Estén Detenidos

```bash
ps aux | grep -E "(bot_|hyrotrader)" | grep -v grep
```

Si no aparece nada, todos los bots están detenidos ✅

---

## 🚀 Solo Iniciar el Nuevo Bot (HyroTrader Rust)

Después de detener los demás:

```bash
cd /home/nova/bot_10k
./target/release/hyrotrader-bot
```

O como servicio:
```bash
sudo systemctl start hyrotrader
journalctl -u hyrotrader -f
```

---

## 📊 Comparación de Bots

| Bot | Lenguaje | Ubicación | Estado |
|-----|----------|-----------|--------|
| bot_nova_v1 | Python | /root/bot_hyrotrader_v7/ | ❌ Detener |
| bot_challenge_4h | Python | /root/bot_hyrotrader_v7/ | ❌ Detener |
| bot_order_book | Rust | /root/bot_order_book/ | ❌ Detener |
| bot_margin_v2 | Rust | /root/bot_margin_v2/ | ❌ Detener |
| **hyrotrader-bot** | **Rust** | **/home/nova/bot_10k/** | **✅ NUEVO** |

---

## ⚠️ Importante

Los bots viejos están corriendo como **root**. Para detenerlos necesitas usar `sudo`.

El nuevo bot (hyrotrader-bot) es el único que:
- ✅ Usa Bybit v5 API (la más actual)
- ✅ Tiene sistema de confluencias mejorado
- ✅ Gestión de riesgo adaptativa
- ✅ Alertas de Telegram
- ✅ Documentación completa
- ✅ Corre como servicio systemd

**Recomendación:** Detén todos los bots viejos y usa solo el nuevo.
