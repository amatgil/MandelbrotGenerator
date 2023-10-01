# Mandelbrot set

# Com s'utilitza
```shell
cargo run --release
```
La sortida serà `sortida.ppm`. Com els formats `Netpbm` (com `.ppm`) son una mica (només
una _miqueta_) ineficaços, es pot convertir a PNG amb:
```shell
convert sortida.ppm sortida.png # Necessita d'ImageMagick instal·lat
```

# Com funciona
Rust obre dos threads:

- Thread 1:
Per cada pixel, calcula quin dels dos colors ha de tindre (de manera
paral·lela). Cada pixel calculat s'envia al thread 2.

-  Thread 2:
Rep els pixels (no en ordre) dels thread 1. 

Amb aquest:
    * Si el pixel que tenim és el que toca escriure, l'escriu i continua
    * Si no, l'emmagatzema a un binari heap.
