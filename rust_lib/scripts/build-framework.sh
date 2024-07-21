# make bindings

# 构建 iOS .a产物
chmod +x generate-bindings-ios.sh
./generate-bindings-ios.sh

# 构建 iOS .xcframework 产物
cd ..

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