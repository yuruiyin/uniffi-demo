# make bindings

start_time=$(date +%s)
script_path=$(cd $(dirname $0); pwd)
project_path=$(readlink -f $script_path/../)
echo "start time: ${start_time}"
cd $project_path

# 构建 iOS .a产物
build_type="debug"
dest="./out/rust_lib_framework.xcframework"
header_dir="out"
static_lib_name="librust_lib.a"
target_dir="target"

chmod +x ./scripts/build-ios.sh
./scripts/build-ios.sh $build_type

# 构建 iOS .xcframework 产物

rm -rf $dest

xcodebuild -create-xcframework \
  -library "${target_dir}/aarch64-apple-darwin/${build_type}/${static_lib_name}" \
  -headers "${header_dir}/aarch64-apple-darwin" \
  -library "${target_dir}/aarch64-apple-ios/${build_type}/${static_lib_name}" \
  -headers "${header_dir}/aarch64-apple-ios" \
  -library "${target_dir}/aarch64-apple-ios-sim/${build_type}/${static_lib_name}" \
  -headers "${header_dir}/aarch64-apple-ios-sim" \
  -output  $dest