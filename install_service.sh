#!/bin/bash

echo "🔧 Instalando HyroTrader Bot como servicio systemd..."
echo ""

# Verificar que el binario existe
if [ ! -f "target/release/hyrotrader-bot" ]; then
    echo "❌ Error: El binario no existe. Compilando primero..."
    cargo build --release

    if [ $? -ne 0 ]; then
        echo "❌ Error en la compilación"
        exit 1
    fi
fi

echo "✅ Binario encontrado"

# Copiar archivo de servicio
echo "📝 Copiando archivo de servicio..."
sudo cp hyrotrader.service /etc/systemd/system/

# Recargar systemd
echo "🔄 Recargando systemd..."
sudo systemctl daemon-reload

# Habilitar servicio para inicio automático
echo "⚡ Habilitando servicio..."
sudo systemctl enable hyrotrader.service

# Mostrar estado
echo ""
echo "="*60
echo "✅ Servicio instalado correctamente!"
echo "="*60
echo ""
echo "Comandos disponibles:"
echo "  sudo systemctl start hyrotrader    # Iniciar bot"
echo "  sudo systemctl stop hyrotrader     # Detener bot"
echo "  sudo systemctl restart hyrotrader  # Reiniciar bot"
echo "  sudo systemctl status hyrotrader   # Ver estado"
echo "  journalctl -u hyrotrader -f        # Ver logs en tiempo real"
echo ""
echo "Para iniciar el bot ahora:"
echo "  sudo systemctl start hyrotrader"
echo ""
