#!/bin/zsh

start_time=$(date +%s)
script_path=$(
  cd $(dirname $0)
  pwd
)
project_path=$(readlink -f $script_path/../)
echo "start time: ${start_time}"
cd $project_path

# 1. build android
chmod +x ./scripts/build-android.sh
./scripts/build-android.sh
if [ $? -ne 0 ]; then
  echo "build android failed"
  exit 1
fi

# 2. build ios xcframework
chmod +x ./scripts/build-ios-framework.sh
./scripts/build-ios-framework.sh
if [ $? -ne 0 ]; then
  echo "build ios failed"
  exit 1
fi

# 3. build harmony
chmod +x ./scripts/build-harmony.sh
./scripts/build-harmony.sh
if [ $? -ne 0 ]; then
  echo "build harmony failed"
  exit 1
fi

echo "batch build success 总耗时: `expr $(date +%s) - ${start_time}`s"
