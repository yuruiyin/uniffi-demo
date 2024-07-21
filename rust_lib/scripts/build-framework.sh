# make bindings

start_time=$(date +%s)
script_path=$(cd $(dirname $0); pwd)
project_path=$(readlink -f $script_path/../)
echo "start time: ${start_time}"
cd $project_path

# 构建 iOS .a产物
chmod +x ./scripts/generate-bindings-ios.sh
./scripts/generate-bindings-ios.sh

# 构建 iOS .xcframework 产物

DEST="./out/rust_lib_framework.xcframework"
ENV="release"
HEADER_DIR="out"
STATIC_LIB_NAME="librust_lib.a"
TARGET_DIR="target"

rm -rf $DEST

xcodebuild -create-xcframework \
  -library "${TARGET_DIR}/aarch64-apple-darwin/${ENV}/${STATIC_LIB_NAME}" \
  -headers "${HEADER_DIR}/aarch64-apple-darwin" \
  -library "${TARGET_DIR}/aarch64-apple-ios/${ENV}/${STATIC_LIB_NAME}" \
  -headers "${HEADER_DIR}/aarch64-apple-ios" \
  -library "${TARGET_DIR}/aarch64-apple-ios-sim/${ENV}/${STATIC_LIB_NAME}" \
  -headers "${HEADER_DIR}/aarch64-apple-ios-sim" \
  -output  $DEST