#!/bin/bash
#traductor de los colores de wallus , es que uso hyprland
sed 's/ = rgb(\(.*\))/: #\1;/g' ~/.config/hypr/wallust/wallust-hyprland.conf > "$(dirname "$0")/music-dot/eww/colors.scss"

#para que no se pierda playerctl y cambie entre web y nativo uso esto
playerctld daemon &

#Esta chingadera le da muerte a todos los procesos viejos de eww
killall eww 2>/dev/null

#la madre que levanta el daemon
eww daemon &

#no se como sean las compus asi que le dare un refresco para que abra la ventana

sleep 0.5
eww open music_win
