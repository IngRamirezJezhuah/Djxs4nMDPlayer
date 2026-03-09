#!/bin/bash

#Colores de la salida
verde='\033[0;32m'
azul='\033[0;34m'
amarillo='\033[1;33m'
nc='\033[0m'

#Pantalla de bienvenida
echo -e "${azul}
===============================================${nc}"
echo -e "${amarillo}
⣿⠛⠛⠛⠛⠻⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠛⢛⣿⠋⢀⡾⠃⠀⠀⠀⠀⢀⣤⣤⠤⠤⣤⣤⣀⣀⣀⣠⠶⡶⣤⣀⣠⠾⡷⣦⣀⣤⣤⡤⠤⠦⢤⣤⣄⡀⠀⢠⡶⢶⡄⠀⠀
⢠⡟⠁⣴⣿⢤⡄⣴⢶⠶⡆⠈⢷⡀⠀⠀⠀⠀⢀⣭⣫⠵⠥⠽⣄⣝⠵⢍⣘⣄⠳⣤⣀⠀⠀⢀⡤⠊⣽⠁⠀⠸⣇⠀⢿⠀⠀
⠸⢷⣴⣤⡤⠾⠇⣽⠋⠼⣷⠀⠈⢷⡄⢀⣤⡶⠋⠀⣀⡄⠤⠀⡲⡆⠀⠀⠈⠙⡄⠘⢮⢳⡴⠯⣀⢠⡏⠀⠀⠀⢻⠀⢸⠇⠀
⠀⠀⠀⠀⠀⠀⠀⠙⠛⠋⠉⢀⣴⠟⠉⢯⡞⡠⢲⠉⣼⠀⠀⡰⠁⡇⢀⢷⠀⣄⢵⠀⠈⡟⢄⠀⠀⠙⢷⣤⣤⣤⡿⢢⡿⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠟⠑⠊⠁⡼⣌⢠⢿⢸⢸⡀⢰⠁⡸⡇⡸⣸⢰⢈⠘⡄⠀⢸⠀⢣⡀⠀⠈⢮⢢⣏⣤⡾⠃⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣯⣴⠞⡠⣼⠁⡘⣾⠏⣿⢇⣳⣸⣞⣀⢱⣧⣋⣞⡜⢳⡇⠀⢸⠀⢆⢧⠀⠰⣄⢏⢧⣾⠁⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢹⡏⢰⠁⡻⠀⡟⡏⠉⠀⣀⠀⠀⠀⠀⣀⠁⠀⠉⠛⢽⠇⠀⣼⡆⠈⡆⠃⠀⡏⠻⣾⣽⣇⡀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠁⡇⠀⡇⡄⣿⠷⠿⠿⠛⠀⠀⠀⠀⠛⠻⠿⠿⠿⡜⢀⡴⡟⢸⣸⡼⠀⠀⡇⠀⡞⡆⢻⠙⢦⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡶⢀⣼⣿⣬⣽⠧⠬⠇⠀⠀⠀⠀⠀⠀⢞⣯⣭⢺⣔⣪⣾⣤⠺⡇⢳⠀⢠⣧⡾⠛⠛⠻⠶⠞⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠷⢿⠟⠉⡀⠈⢦⡀⠀⠀⣠⠖⠒⠒⢤⡀⠀⢀⡼⠿⢇⡣⢬⣶⠷⢿⣤⡾⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠷⠾⠷⠖⠛⠛⠲⠶⠿⠤⣤⠤⠤⢷⣶⠋⠀⠀⠀⣱⠞⠁⠀⠈⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠓⠒⠚⠋${nc}"
echo -e "${azul} Instalador de dependencias: Music-Dot${nc}"

echo -e "${azul}================================================${nc}"

#validacion de privilegios
echo -e "${verde}[1/4]${nc}" 
echo -ne "${amarillo}¿Deseas instalar las dependencias necesarias? (s/n): ${nc}"
read -r respuesta

if [[ ! "$respuesta" =~ ^[SsyY]$ ]]; then
  echo -e "${amarillo}a Instalacion cancelada. ¡Adios! Y(˶˃ ᵕ ˂˶)Y${nc}"
  exit 0
fi

echo -e "${verde}a Verificando permisos...${nc}"
if [ "$EUID" -ne 0 ]; then
  echo -e "${amarillo}a Se requiere privilegios avanzados, Por favor, Ingresa tu constraseña: ${nc}"
  sudo -v || exit 1
fi

#La madre que actualizacion de repositorios
echo -e "${verde}[2/4]Actualizando repositorios...${nc}"
sudo apt update
echo -e "${verde}..."
sleep 1

Dependencias=(
  "playerctl" "mpd" 
  "mpc" "socat" 
  "jq" "imagemagick" 
  "libpango1.0-dev" 
  "libcairo-dev" 
  "libgdk-pixbuf2.0-dev"
  "libgtk-3-dev" 
  "libglib2.0-dev" 
  "build-essential"
  "rustc" "cargo"
)

#Ehh esto es lo que revisa los paquetes y los instala
echo -e "${verde}[3/4] Instalando paquetes del sistema...${nc}"
sleep 1
for pkg in "${Dependencias[@]}"; do
  if dpkg -l | grep -q "^ii $pkg "; then 
    echo -e "${azul}==================================="
    echo -e "${verde}[Oki]${nc} $pkg ya esta instalado."
    echo -e "${azul}==================================="
  else
    echo -e "${verde}a Instalando $pkg...${nc}"
    sudo apt install -y "$pkg"
  fi
done

#Necesito que rust se copile para que funcione al instalar
echo -e "${verde}[4/4] Verificando Eww...${nc}"
sleep 1
if ! command -v eww &> /dev/null; then
  echo -e "${amarillo}EWW no encontrado. Procediendo a instalar via Cargo..."
  git clone https://github.com/elkowar/eww /tmp/eww
    cd /tmp/eww || exit
    cargo build --release --no-default-features --features=wayland
    sudo cp target/release/eww /usr/local/bin/
    echo -e "${verde}[Oki] EWW instalado correctamente.  esta en /tmp/eww${nc}"
  else
    echo -e "${verde}[Oki]${nc} EWW Ya esta presente en el sistema."
fi

echo -e "${verde}a Configurando el entorno de Eww...${nc}"
sleep 1
DIR="$(cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)"

#Enlace simbolico para que configure eww
ln -sfn "$DIR/music-dot/eww" ~/.config/eww

echo -e "${verde}[Oki] Enlace simbolico creadi en ~/.config/eww${nc}"

echo -e "${azul}
      !Dependencias listas!
    Music-Dot puede continuar.${nc}"

echo -e "${azul}===============================================${nc}"
sleep 1
