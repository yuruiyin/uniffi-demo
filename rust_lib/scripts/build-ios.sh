#!/bin/bash

# 构建类型 debug or release
build_type=$1
#build_type=release

# 根据构建类型获取当前构建profile dev or release
if [ "$build_type" = "debug" ]; then
  profile=dev
else
  profile=release
fi

start_time=$(date +%s)
script_path=$(
  cd $(dirname $0)
  pwd
)
project_path=$(readlink -f $script_path/../)
echo "start time: ${start_time}"
cd $project_path

#pushd rust_lib
#targets=("aarch64-apple-ios" "aarch64-apple-ios-sim" "aarch64-apple-darwin" "x86_64-apple-ios" "x86_64-apple-darwin")
targets=("aarch64-apple-ios" "aarch64-apple-ios-sim" "aarch64-apple-darwin")
# targets=("ios-sim")
for target in "${targets[@]}"; do
  echo "Generating target=${target}"
  cargo build --target $target --profile=$profile --features uniffi

  if [ $? -ne 0 ]; then
    echo "cargo build failed"
    exit 1
  fi

  cargo run \
    --bin uniffi-bindgen generate \
    --library target/${target}/$build_type/librust_lib.a \
    --language swift \
    --out-dir out/${target}

  if [ $? -ne 0 ]; then
    echo "cargo run failed"
    exit 1
  fi

  mv out/${target}/rust_libFFI.modulemap out/${target}/module.modulemap
done
#popd
