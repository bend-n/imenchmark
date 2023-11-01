## a series of benchmarks, comparing [imageproc](https://crates.io/crates/imageproc) and [fimg](https://crates.io/crates/fimg)

![](./affine.svg)
![](./resizing.svg)
![](./drawing.svg)

### how to run benchmarks

step 1: unhew. `cargo install --git https://github.com/bend-n/hew && hew data.png data.imgbuf`
step 2: run. `cargo bench`