#!/bin/bash

#Esta chingadera le da muerte a todo proceso viejo de eww
killall eww 2>/dev/null

#la madre que levanta el daemon
eww daemon &

#no se como sean las compus asi que le dare un refresco para que abra la ventana

sleep 0.5
eww open music_win
