#!/bin/bash

#en este lo que estoy haceindo es traducir los numeros de cava en caracteres de bloque visuales

dict=" ▂▃▄▅▆▇█"
config_file="$(dirname "$0")/../eww/cava.conf"

#entonces aqui ejecuto cava y tomo los datos que escupe para los datos linea por linea
if [ ! -f "$config_file" ]; then
  echo "Error: No se encuentra cava.conf"
  exit 1
fi

cava -p "$config_file" | while read -r line; do
  #aqui cambio los punto y coma por espacios para la bash lo lea
  line="${line//;/ }"
  output=""

  #entonces de esa madre por cada numero hay un bloque que corresponde a su nivel lo que esta en el dict
  for v in $line; do
    #no quiero pinches errores asi que esto es por si cava se pone raro
    if [[ "$v" =~ ^[0-7]$ ]]; then
      output="${output}${dict:$v:1}"
    fi
  done
  #ahora si todo sale bien se "supone" que debe darme la liena por bloquespara que el eww sepa leerlos
  echo "$output"
done
