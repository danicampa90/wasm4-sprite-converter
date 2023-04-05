# Wasm4 Sprite Converter
A small texture conversion tool for converting an image in png to Rust source code containing the sprite data for WASM4.

The tool takes in input a tile map in .png format (for example, "example.png") and a file decribing its content ("example.yaml") and ouputs Rust source code to the standard output.

It supports:
- 1bpp format (2 colors)
- 2bpp format (4 colors)
- Custom input color palette mapping. Each color in the source image can be mapped to any palette index in the output.
- Mixing output format (1bpp and 2bpp) in a single image.


Supported output languagues: Rust only for now. For adding more support look at src\output_langs\rust_encoder.rs. Contributions are welcome!

Limitations:
- Each input image has a fixed grid size. This makes it easier to use for most of the cases, but also prevents converting tightly packed textures with no clear alignment.
  - I reccommend using multiple separate files if converting grids of differing sizes.
  - For specifiying pixel sizing instead of using a grid you can set the grid size to (1,1).
- The tool was built relatively quickly, so the code might be a bit difficult to read in places. It was still built to be easy to extend.


## Building 
Simply get the latest (nightly) version of rust and run

```bash
cargo build
```

The resulting binary is in the `target/` folder

## Running
You can build and run the example in a single command:

```bash
cargo run -- .\example.yaml .\example.png -o example.rs
```
