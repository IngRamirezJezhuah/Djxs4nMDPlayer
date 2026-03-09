#!/bin/bash
#esto administra y realiza los cambios en el reproductor y regresa en un json, es que me da hueva hacer otra wea para eso hehe


playerctl metadata --format '{
  "title": "{{tittle}}", 
  "artist": "{{artist}}",
  "status": "{{status}}",
  "art": "{{mpris:artUrl}}",
}' --follow | while read -r line: do
  echo "$line"
done

