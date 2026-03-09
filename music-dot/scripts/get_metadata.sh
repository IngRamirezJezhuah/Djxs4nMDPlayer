#!/bin/bash
#esto administra y realiza los cambios en el reproductor y regresa en un json, es que me da hueva hacer otra wea para eso hehe


#imagen por defecto si no hay caratula
DEFAULT_COVER="$(pwd)/../eww/default.png"
COVER="/tmp/music-dot-cover.png"

get_data() {
  #esta madre busca la info del reproductor activo (la chingadera en la que esuches musica no seas pendejo xd)
  status=$(playerctl status 2>/dev/null || echo "Offline")
  title=$(playerctl metadata title 2>/dev/null || echo "Nada Sonando...")
  artist=$(playerctl metadata artist 2>/dev/null || echo "...")
  art_url=$(playerctl metadata mpris:artUrl 2>/dev/null)

  #Con la nueva mare del cava ahora extraigo los tiempos brutos
  pos=$(playerctl position 2>/dev/null || cut -d'.' -f1)
  len_us=$(playerctl metadata mpris:length 2>/dev/null)
  #segun lei puedo usar awk para convertirlo a segundo entero por que len_raw viene en microsegundos
  #position=$(echo "$post_raw" | awk '{print int($1)}')
  #length=$(echo "$len_us" | awk '{print int($1/1000000)}')
  [[ -z "$pos" ]] && pos=0
  if [[ -z "$len_us" ]]; then len=100; else len=$((len_us / 1000000)); fi

  #logica de la caratula pa que se muestre pues
  if [[ -z  "$art_url" ]]; then
      cp "$DEFAULT_COVER" "$COVER" 2>/dev/null || touch "$COVER"
    elif [[ "$art_url" == http* ]]; then
      curl -s "$art_url" -o "$COVER"
    elif [[ "$art_url" == file://* ]]; then
      #esta es la madre local la copia por que soy flojo
      cp "${art_url#file://}" "$COVER"
  fi
  #Esta chingadera genera un json limpio con un jq a prueba de errores por que estoy hasta la chingada de depurar typos a la verga
  jq --unbuffered -c -n --arg title "$title" --arg artist "$artist" --arg status "$status" --arg cover "$COVER" --arg pos "$position" --arg len "$length" '{title: $title, artist: $artist, status: $status, cover: $cover, position: $pos, length: $len}'
}

while true; do
  get_data
  sleep 0.5
done

#forma de lectura de jq
#playerctl metadata --format '{
#  "title": "{{tittle}}", 
#  "artist": "{{artist}}",
#  "status": "{{status}}",
#  "art": "{{mpris:artUrl}}"
#}' --follow | while read -r line: do
#  echo "$line"
#done

