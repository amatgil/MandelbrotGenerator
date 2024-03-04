# Simple mandelbrot set generator

# Usage
```shell
cargo run --release
```
Output will be `sortida.ppm`. Since `Netpbm` formats (e.g. `.ppm`) are a bit (just a tiny bit)
inefficient, they may be converted to a png like so:

```shell
convert sortida.ppm sortida.png # Needs ImageMagick to be installed
```

# How does it work?
It creates two threads:

- Thread 1:
For each pixel, calculate which of the two colors it should have (this is also
parallelized). Each calculated pixel is sent to thread 2.

- Thread 2:
Get the pixels (out of order) from thread 1.

With this:
* If the pixel we have is the one to write, it writes it and continues
* If not, store it in a binary heap.

This means the file may be written to continously and in order without holding 
the entire resulting file in memory at once: it may keep on writing as they're being generated.
