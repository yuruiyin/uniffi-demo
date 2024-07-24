## 构建流程
### 构建 iOS

#### 生成 xcframework 产物
```shell
chmod +x ./scripts/build-ios-framework.sh
./scripts/build-ios-framework.sh
```
上述命令会执行如下操作：
1. 生成 .a 产物
2. 通过 uniffi-rs 生成 swift binding 文件 rust_lib.swift
3. 生成 xcframework 产物

#### 将 xcframework 应用到 iOS 工程中
构建完之后，将 out/rust_lib_framework.xcframework 和 rust_lib.swift（out目录下任意一个 target 下的 rust_lib.swift 均可） 拖入到 iOS 工程中，这样就可以在工程中使用 rust_lib 了。

### 构建 Android

#### 生成 so 产物 & 拷贝产物到 Android 工程中
```shell
chmod +x ./scripts/build-android.sh
./scripts/build-android.sh
```
上述命令会执行如下操作：
1. 生成 so 产物
2. 通过 uniffi-rs 生成 kotlin binding 文件 uniffi/rust_lib/rust_lib.kt
3. 拷贝 so 产物到 Android 工程中
4. 拷贝 rust_lib.kt 文件到 Android 工程中

### 构建 HarmonyOS
```shell
chmod +x ./scripts/build-harmony.sh
./scripts/build-harmony.sh
```
上述命令会执行如下操作：
1. 通过ohos-rs(基于napi-rs) 生成 so 产物 & rust接口声明文件 index.d.ts
2. 拷贝 so 产物到 HarmonyOS 工程中
3. 拷贝 rust接口声明文件 index.d.ts 到 HarmonyOS 工程 entry/libs/rust_lib 下
4. 在鸿蒙工程 entry/libs/rust_lib 目录下生成 oh-package.json5 文件，来告知鸿蒙工程这个是一个 package 且它的入口文件地址
5. 往鸿蒙工程 entry/oh-package.json5 中添加依赖, 以让ArkTS可以引用到 librust_lib.so 的index.d.ts。便于符号跳转和代码提示
