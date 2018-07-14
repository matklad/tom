set -ex

cargo build \
      --manifest-path ./tom_wasm/Cargo.toml \
      --target wasm32-unknown-unknown \
      --release

TARGET_DIR=tom_wasm/target/wasm32-unknown-unknown/release

cp $TARGET_DIR/tom_wasm.wasm $TARGET_DIR/main.wasm

wasm-bindgen --no-modules --no-typescript \
             $TARGET_DIR/main.wasm \
             --out-dir .

wasm-opt -Os --dce main_bg.wasm -o main_bg.wasm

