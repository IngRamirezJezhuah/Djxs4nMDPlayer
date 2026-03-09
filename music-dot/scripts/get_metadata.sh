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
  jq --unbuffered -c -n --arg title "$title" --arg artist "$artist" --arg status "$status" --arg cover "$COVER" '{title: $title, artist: $artist, status: $status, cover: $cover}'
}

while true; do
  get_data
  sleep 0.5
done

#playerctl metadata --format '{
#  "title": "{{tittle}}", 
#  "artist": "{{artist}}",
#  "status": "{{status}}",
#  "art": "{{mpris:artUrl}}"
#}' --follow | while read -r line: do
#  echo "$line"
#done

