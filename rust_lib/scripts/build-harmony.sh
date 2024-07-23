#!/bin/zsh

# 构建类型 debug or release
build_type=debug
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

# 1. 构建 harmony-os 产物，目前arm32构建有问题，先忽略
#targets=("aarch64-unknown-linux-ohos" "x86_64-unknown-linux-ohos", "armv7-unknown-linux-ohos")
targets=("aarch64-unknown-linux-ohos" "x86_64-unknown-linux-ohos")

# 构建 rust target 和 ohos target 的对应关系
declare -A target_map_ohos
target_map_ohos["aarch64-unknown-linux-ohos"]="arm64-v8a"
target_map_ohos["armv7-unknown-linux-ohos"]="armeabi-v7a"
target_map_ohos["x86_64-unknown-linux-ohos"]="x86_64"

# 构建 rust target 和 ohos target 的对应关系
declare -A target_map_ohrs
target_map_ohrs["aarch64-unknown-linux-ohos"]="arm64"
target_map_ohrs["armv7-unknown-linux-ohos"]="arm32"
target_map_ohrs["x86_64-unknown-linux-ohos"]="x64"

for target in "${targets[@]}"; do
  echo "Generating target=${target}"
  # 1. 构建产物
  target_ohrs=${target_map_ohrs["${target}"]}
  echo "target_ohrs: ${target_ohrs}"
#  ohrs build --release --arch=$target_ohrs -- --features napi
  ohrs build --arch=$target_ohrs -- --profile=$profile --features napi

  if [ $? -ne 0 ]; then
    echo "ohrs build failed"
    exit 1
  fi

  # 2. 将so产物拷贝到 entry/libs 目录下
  harmony_libs_dir=../apps/harmony/entry/libs
  # 如果 鸿蒙target目录不存在，则创建
  harmony_libs_target_dir=${harmony_libs_dir}/${target_map_ohos["${target}"]}
  if [ ! -d ${harmony_libs_target_dir} ]; then
    mkdir -p ${harmony_libs_target_dir}
  fi

  cp ./target/${target}/$build_type/librust_lib.so ${harmony_libs_target_dir}/

  # 3. 在 entry/libs 目录下创建 rust_lib 目录 (如果不存在）
  if [ ! -d ${harmony_libs_dir}/rust_lib ]; then
    mkdir -p ${harmony_libs_dir}/rust_lib
  fi

  # 4. 将 index.d.ts 文件拷贝 entry/libs/rust_lib 目录下
  cp ./dist/index.d.ts ${harmony_libs_dir}/rust_lib/index.d.ts

  # 5. 生成 鸿蒙包的 json文件（oh-package.json5）格式如下：
  ohos_package_json5=${harmony_libs_dir}/rust_lib/oh-package.json5
  # shellcheck disable=SC2089
  json_content='{
      "name": "librust_lib.so",
      "types": "./index.d.ts",
      "version": "1.0.0",
      "description": "Rust lib."
  }'
  echo "$json_content" > $ohos_package_json5

  # 6. 往entry/oh-package.json5中添加依赖, 以让ArkTS可以引用到 librust_lib.so 的index.d.ts。便于知道so中有暴露的对象和接口
  # 添加依赖指的是在entry/oh-package.json5中的dependencies字段中添加依赖(如果不存在的话): "librust_lib.so": "file:./libs/rust_lib"
  harmony_entry_package_json=../apps/harmony/entry/oh-package.json5
  jq '.dependencies += {"librust_lib.so": "file:./libs/rust_lib"}' $harmony_entry_package_json > temp.json && mv temp.json $harmony_entry_package_json
done
