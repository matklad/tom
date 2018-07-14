set -e

cargo +nightly build \
      --manifest-path ./tom_wasm/Cargo.toml \
      --target wasm32-unknown-unknown \
      --release

TARGET_DIR=tom_wasm/target/wasm32-unknown-unknown/release

wasm-opt -Os --dce \
         $TARGET_DIR/tom_wasm.wasm \
         -o $TARGET_DIR/main.wasm

wasm-bindgen --no-modules --no-typescript \
             $TARGET_DIR/main.wasm \
             --out-dir .

