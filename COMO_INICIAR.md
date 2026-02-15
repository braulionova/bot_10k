# 🚀 Cómo Iniciar el Bot - HyroTrader

## ✅ El bot está COMPILADO y LISTO

Ubicación: `/home/nova/bot_10k/target/release/hyrotrader-bot`

---

## 🎯 Opción 1: Ejecutar Manualmente (Rápido)

```bash
cd /home/nova/bot_10k
./target/release/hyrotrader-bot
```

**Ventajas:**
- ✅ Inmediato, no requiere sudo
- ✅ Ves los logs en tiempo real
- ✅ Fácil de detener (Ctrl+C)

**Desventajas:**
- ❌ Se detiene si cierras la terminal
- ❌ No se reinicia automáticamente

---

## 🔥 Opción 2: Como Servicio (RECOMENDADO)

### Paso 1: Instalar el Servicio

```bash
cd /home/nova/bot_10k
sudo cp hyrotrader.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable hyrotrader
```

### Paso 2: Iniciar el Bot

```bash
sudo systemctl start hyrotrader
```

### Paso 3: Verificar que Esté Corriendo

```bash
sudo systemctl status hyrotrader
```

Deberías ver: **● active (running)** ✅

### Paso 4: Ver Logs en Tiempo Real

```bash
journalctl -u hyrotrader -f
```

Presiona `Ctrl+C` para salir de los logs (el bot sigue corriendo).

---

## 📱 Verificar Alertas de Telegram

El bot debería enviarte un mensaje de "Bot iniciado" a tu Telegram.

Si no lo recibes:
1. Verifica que el bot esté corriendo: `sudo systemctl status hyrotrader`
2. Revisa los logs: `journalctl -u hyrotrader -n 50`
3. Verifica el token en `.env`: `cat .env | grep TELEGRAM`

---

## 🛠️ Comandos Útiles del Servicio

```bash
# Iniciar el bot
sudo systemctl start hyrotrader

# Detener el bot
sudo systemctl stop hyrotrader

# Reiniciar el bot
sudo systemctl restart hyrotrader

# Ver estado
sudo systemctl status hyrotrader

# Ver logs (últimos 100)
journalctl -u hyrotrader -n 100

# Ver logs en tiempo real
journalctl -u hyrotrader -f

# Ver logs de hoy
journalctl -u hyrotrader --since today

# Deshabilitar inicio automático
sudo systemctl disable hyrotrader
```

---

## 🎮 Uso Diario

### Por la Mañana
```bash
# Ver estado del bot
sudo systemctl status hyrotrader

# Ver actividad reciente
journalctl -u hyrotrader --since "1 hour ago"
```

### Actualizar Configuración

1. Editar `.env`:
   ```bash
   nano .env
   ```

2. Reiniciar bot:
   ```bash
   sudo systemctl restart hyrotrader
   ```

### Si el Bot se Traba

```bash
# Detener
sudo systemctl stop hyrotrader

# Esperar 5 segundos
sleep 5

# Iniciar de nuevo
sudo systemctl start hyrotrader

# Verificar
sudo systemctl status hyrotrader
```

---

## 📊 Qué Esperar

### Primeros Minutos
El bot:
1. ✅ Se conecta a Bybit Testnet
2. ✅ Te envía mensaje de inicio a Telegram
3. ✅ Empieza a analizar BTCUSDT y ETHUSDT cada 60 segundos
4. ✅ Calcula scores de confluencia
5. ⏳ Espera un setup válido (score ≥70)

### Logs que Verás
```json
{"level":"INFO","message":"🔄 Ciclo #1 - 18:45:23"}
{"level":"INFO","message":"📊 Analizando BTCUSDT..."}
{"level":"INFO","message":"  💰 Precio: $69,828.30"}
{"level":"INFO","message":"  🎯 RSI(14): 52.3"}
{"level":"INFO","message":"  🎲 Score de Confluencia: 45/100"}
{"level":"INFO","message":"  ⚠️  Score insuficiente (mínimo 70)"}
```

### Alertas de Telegram
Recibirás notificaciones cuando:
- ✅ Bot inicia/detiene
- ✅ Se detecta un setup válido (score ≥70)
- ⚠️ Alertas importantes (drawdown, problemas)

**NOTA:** El bot NO ejecutará órdenes reales aún - solo analiza el mercado y reporta oportunidades.

---

## 🐛 Troubleshooting

### "Failed to start hyrotrader.service"
```bash
# Ver el error exacto
journalctl -u hyrotrader -n 20

# Verificar que el binario existe
ls -l /home/nova/bot_10k/target/release/hyrotrader-bot

# Verificar permisos
chmod +x /home/nova/bot_10k/target/release/hyrotrader-bot
```

### "Connection refused" o errores de API
```bash
# Verificar credenciales
cat .env | grep EXCHANGE

# Test manual de API
curl "https://api-testnet.bybit.com/v5/market/time"
```

### Bot se reinicia continuamente
```bash
# Ver por qué crashea
journalctl -u hyrotrader -n 100

# Posibles causas:
# - API keys incorrectas
# - Problemas de red
# - Bug en el código
```

---

## 🎯 Próximos Pasos

1. **Iniciar el bot:**
   ```bash
   sudo systemctl start hyrotrader
   ```

2. **Monitorear primeras horas:**
   ```bash
   journalctl -u hyrotrader -f
   ```

3. **Verificar Telegram:**
   - Deberías recibir mensaje de inicio
   - Luego reportes periódicos de análisis

4. **Dejar correr 24-48h:**
   - Ver qué oportunidades detecta
   - Verificar que no crashee
   - Revisar logs para errores

5. **Cuando estés listo para trading real:**
   - Completar implementación de órdenes
   - Hacer backtesting
   - Probar en paper trading 10 días
   - Solo entonces pasar a live con dinero real

---

## 📞 Comandos de Emergencia

### Detener Bot INMEDIATAMENTE
```bash
sudo systemctl stop hyrotrader
sudo systemctl disable hyrotrader
```

### Eliminar Servicio Completamente
```bash
sudo systemctl stop hyrotrader
sudo systemctl disable hyrotrader
sudo rm /etc/systemd/system/hyrotrader.service
sudo systemctl daemon-reload
```

### Backup de Configuración
```bash
cp .env .env.backup
cp SETUP_COMPLETO.md ~/hyrotrader_setup_backup.md
```

---

## ✅ Checklist Final

Antes de dejar el bot corriendo sin supervisión:

- [ ] Bot inicia correctamente
- [ ] Telegram envía notificaciones
- [ ] Logs muestran análisis cada minuto
- [ ] No hay errores en logs
- [ ] Balance testnet en $10,000
- [ ] `EXCHANGE_TESTNET=true` verificado
- [ ] Revisé el bot cada 2-4 horas primeras 24h

---

**¡El bot está listo! 🚀**

Para iniciarlo ahora:
```bash
sudo systemctl start hyrotrader && journalctl -u hyrotrader -f
```
