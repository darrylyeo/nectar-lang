# Compile Rust parser to WebAssembly with wasm-pack
# cargo install wasm-pack
# wasm-pack build

# Compile Rust parser to WebAssembly with ssvmup
[ -x "$(command -v ssvmup)" ] || curl https://raw.githubusercontent.com/second-state/ssvmup/master/installer/init.sh -sSf | sh
ssvmup build --target deno

# Optimize .wasm file size
cargo install wasm-gc
wasm-gc target/wasm32-unknown-unknown/release/nectar_lib.wasm