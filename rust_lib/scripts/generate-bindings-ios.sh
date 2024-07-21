#!/bin/bash

#pushd rust_lib
cd ..
#  targets=("ios" "ios-sim" "darwin")
  targets=("ios-sim")
  for suffix in "${targets[@]}"
  do
    target="aarch64-apple-${suffix}"
    echo "Generating target=${target}"
    cargo build --target $target --release

    if [ $? -ne 0 ]; then
      echo "cargo build failed"
      exit 1
    fi  

    cargo run \
      --bin uniffi-bindgen generate \
      --library target/${target}/release/librust_lib.a \
      --language swift \
      --out-dir out/${target}

    if [ $? -ne 0 ]; then
      echo "cargo run failed"
      exit 1
    fi 

    mv out/${target}/rust_libFFI.modulemap out/${target}/module.modulemap
  done
#popd