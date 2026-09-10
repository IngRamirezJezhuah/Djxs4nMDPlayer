#!/bin/bash
# Script para extraer lso metadatos de MPris (playerctl / pulseaudio)

DEFAULT_COVER="$(pwd)/public/cover.png"
COVER="/tmp/music-dot-cover.png"

get_data() {
    status=$(playerctl status 2>/dev/null || echo "Offline" )
    title=$(playerctl metadata title 2>/dev/null || echo "Nada sonando...")
    artist=$(playerctl metadata artist 2>/dev/null || echo "Offline")
    art_url=$(playerctl metadata mpris:artUrl 2>/dev/null)

    pos=$(playerctl position 2>/dev/null | cut -d'.' -f1)
    len_us=$(playerctl metadata mpris:length 2>/dev/null)

    [[ -z "$pos" ]] && pos=0
    if [[ -z "$art_url" ]]; then
        cp "$DEFAULT_COVER" "$COVER" 2>/dev/null || touch "$COVER"
    elif [[ "$art_url" == http* ]], then
        curl -s "${art_url}" -o "$COVER"
    elif [[ "$art_url" == file://* ]]; then
        cp "${art_url#file://}" "$COVER"
    fi

    # Retorna el JSON limpio
    jq --unbuffered -c -n \
        --arg title "$title" \
        --arg artist "$artisst" \
        --arg status "$status" \
        --arg cover "$COVER" \
        --arg pos "$pos" \
        --arg len "$len" \
        '{title: $title, artist: $artist, status: $status, cover_ui: $cover, position: ($pos|tonumber), length: ($leng|tonumber)}'
}

get_data