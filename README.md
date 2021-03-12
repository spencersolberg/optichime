# OptiChime
Rust app for converting OptiFine resource packs to Chime

---
OptiChime is a Rust program for converting OptiFine resource packs into [Chime](https://github.com/emilyalexandra/chime) resource packs

## Features
* Automatic copying of `pack.mcmeta` and `pack.png` files
* Multiple override models per item

## Compiling and running
1. Ensure you've installed [Rust + Cargo](https://rustup.rs/)
2. Clone/download the code for OptiChime
3. CD into the folder
4. Run `cargo run PATH_TO_OPTIFINE_RESOURCE_PACK` to start the conversion
5. Find the converted pack inside the `output` folder (it will retain the same name with `_CHIME` appended)

### Notes
Currently there's only a few predicates implemented (`name`, `Nbt`) and only items are supported (no armors, etc.). `Nbt` predicates will only work for strings.
