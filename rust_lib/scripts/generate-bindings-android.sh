#!/bin/zsh

#pushd rust_lib
cd ..
# targets=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android")
targets=("aarch64-linux-android")

# 构建 rust target 和 android target 的对应关系
# aarch64-linux-android -> arm64-v8a
# armv7-linux-androideabi -> armeabi-v7a
# x86_64-linux-android -> x86_64
# if [ "${BASH_VERSINFO:-0}" -lt 4 ]; then
#   echo "Bash 版本需要 >= 4.0"
#   exit 1
# fi

declare -A target_map
target_map["aarch64-linux-android"]="arm64-v8a"
target_map["armv7-linux-androideabi"]="armeabi-v7a"
target_map["x86_64-linux-android"]="x86_64"

for target in "${targets[@]}"; do
  echo "Generating target=${target}"
  # 1. 构建产物
  cargo build --target $target --release

  if [ $? -ne 0 ]; then
    echo "cargo build failed"
    exit 1
  fi

  # 2. 生成kotlin bingdings(uniffi/rust_lib/rust_lib.kt)
  cargo run \
    --bin uniffi-bindgen generate \
    --library target/${target}/release/librust_lib.so \
    --language kotlin \
    --out-dir out/${target}

  if [ $? -ne 0 ]; then
    echo "cargo run failed"
    exit 1
  fi  

  # 3. 将生成的 uniffi 目录拷贝到 android 工程中
  cp -r out/${target}/uniffi ../apps/android/app/src/main/java/

  # 4. 将so文件拷贝到 android 工程中
  android_target=${target_map["${target}"]}
  echo "android_target: ${android_target}"
  target_dir="../apps/android/app/libs/${android_target}"
  if [ ! -d ${target_dir} ]; then
    mkdir -p ${target_dir}
  fi
  cp target/${target}/release/librust_lib.so ${target_dir}/librust_lib.so
  # mv out/${target}/shared_libFFI.modulemap out/${target}/module.modulemap
done
#popd
