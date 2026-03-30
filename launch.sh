#!/bin/bash
#======================================================================
#Nota esta madre la hice basandome en el uso de un entorno en hyprland,como tengo chingaderas que son, hyprp el rofi, y otras weas asi use los colores y esas mamadas para que se viera aca que chulo que precioso, luego trabajare en una que este en base x11 para quienes no usen un gestor de ventana, pero ahi va
#======================================================================
#Aqui defino la ruta del proyecto de manera dinamica por que pues uso un chingo de archivos que usan eww y para no batallar al ejecutar dirnamw "0" apunta a la carpeta donde esta el launch.sh
#=====================================================================
PROJECT_DIR="$(dirname "$0")"
CFG_DIR="$PROJECT_DIR/music-dot/eww"

#El traductor de los colores de wallus , es que uso hyprland y asi usamos los colores del wallpaper, ojo si tu tienes las configuraciones de esa wea en otra carpeta porfa bro modificala a la que tu tienes
grep "=" ~/.config/hypr/wallust/wallust-hyprland.conf | sed 's/ = rgb(\(.*\))/: #\1;/g' | tr -d '\r' > "$(dirname "$0")/music-dot/eww/colors.scss"
#↳ en español lo que hace es que busca el archivo en tus config de color 
#y una vez lo encuentra a partir de eso va a crear un archico copia de 
#ese en la carpeta de este proyecto y le da el nombre de colors.scss 
#por si le quieres meter mano ahi puedes buscarlo 

#para que no se pierda playerctl y cambie entre web y nativo uso esto
#killall eww 2>/dev/null
#playerctld daemon &
 if ! pidof eww > /dev/null; then
   # ↳ esta madre lo que hace es que revisa si el eww esta chambeando
   playerctld daemon & 
   # ↳ entonces si esta chambeando pues pide al reproductor que busque el anterior que ya estaba haciendo chamba de musica y lo pone para reproducir
   eww --config "$CFG_DIR" daemon &
   #↳ ahora si ya prende el cacharro a chambear
   sleep 0.5
   #↳hay que darle tiempo a que se muestre segun un profe eso es bueno como experiencia de user algo asi decia
 fi

#Esta chingadera le da muerte a todos los procesos viejos de eww : killall eww 2>/dev/null <-- la guardo por si acaso para futuros casos
#no se como sean las compus de otros asi que le dare un refresco para que abra la ventana y la recargue con el cambio de pantalla
eww --config "$CFG_DIR" reload
eww --config "$CFG_DIR" open --toggle music_win

