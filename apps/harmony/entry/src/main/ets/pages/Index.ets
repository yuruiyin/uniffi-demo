import { hilog } from '@kit.PerformanceAnalysisKit';
// import testNapi from 'libentry.so';
import rustLib, { Output, register, VersionInfoRequestModel, VersionInfoResponseModel,
  AppConfig,
  AppLog} from 'librust_lib.so';
import { bundleManager } from '@kit.AbilityKit';
import dayjs from "dayjs"

function log(tag: string, message: string) {
  hilog.info(0x0000, "rust" + tag, '%{public}s', message)
}

// 测试 rust init/register
@Component
struct RustInitItem {
  @State rustRes: number = 0

  // appConfig（供rust调用）
  appConfigFunc(err: Error | null): Promise<AppConfig> {
    return new Promise((resolve, reject) => {
      if (err != null) {
        reject(err)
        return
      }

      let bundleFlags = bundleManager.BundleFlag.GET_BUNDLE_INFO_DEFAULT
      bundleManager.getBundleInfoForSelf(bundleFlags)
        .then((data) => {
          log("getVersionName", "promise resolve")
          resolve({
            version: data.versionName,
            env: "fat",
            userId: "123",
          })
        })
        .catch((e: Error) => {
          reject(e)
        })
    });
  }

  // 当前page（供rust调用）
  currentPageFunc(err: Error | null): Promise<string> {
    return new Promise((resolve, reject) => {
      if (err != null) {
        reject(err)
        return
      }

      resolve("index page")
    });
  }

  // 日志（供rust调用）
  logFunc(err: Error | null, arg: AppLog): Promise<undefined> {
    return new Promise((resolve, reject) => {
      if (err != null) {
        reject(err)
        return
      }

      log(arg.tag, arg.message)
      resolve(undefined)
    });
	}

  // 系统时间（供rust调用）
  systemTimeFunc(err: Error | null): Promise<string> {
    return new Promise((resolve, reject) => {
      if (err != null) {
        reject(err)
        return
      }

      let formatDate = dayjs().format("YYYY-MM-DD HH:mm:ss")
      resolve(formatDate)
    });
  }


  build() {
    Column() {
      // rust init/register
      Button('rust init')
        .onClick(async () => {
          try {
            await register(this.appConfigFunc, this.currentPageFunc, this.logFunc, this.systemTimeFunc)
          } catch(e) {
            log("RustInitItem", JSON.stringify(e))
          }
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust add
@Component
struct RustAddItem {
  @State rustAddRes: number = 0

  build() {
    Column() {
      // rust add
      Text(`rust add res = ${this.rustAddRes}`)
        .margin({
          top: 20,
        })

      Button('rust add')
        .onClick(() => {
          this.rustAddRes = rustLib.add(1, 2)
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust async add
@Component
struct RustAsyncAddItem {
  @State rustAsyncAddRes: number = 0

  build() {
    Column() {
      // rust async add
      Text(`rust async add res = ${this.rustAsyncAddRes}`)
        .margin({
          top: 20,
        })

      Button('rust async add')
        .onClick(async () => {
          this.rustAsyncAddRes = await rustLib.asyncAdd(1, 2)
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust add input(入参和出参都是结构体)
@Component
struct RustAddInputItem {
  @State rustAddInputRes: number = 0

  build() {
    Column() {
      // rust add input(即入参和出参都是结构体)
      Text(`rust add input res = ${this.rustAddInputRes}`)
        .margin({
          top: 20,
        })

      Button('rust add input')
        .onClick(() => {
          let addOuput: Output = rustLib.addInput({
            left: 3,
            right: 4,
          })
          this.rustAddInputRes = addOuput.result
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust call js function
@Component
struct RustCallJsItem {
  @State rustJsCallbackRes: string = ""

  build() {
    Column() {
      // rust js_callback
      Text(`rust js callback get version name res = ${this.rustJsCallbackRes}`)
        .margin({
          top: 20,
        })

      Button('rust js callback')
        .onClick(async () => {
          this.rustJsCallbackRes = rustLib.getVersionName((input: string) => {
            log("jsCallbackGetVersionName", input)
            return "1.0.1"
          })
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust call js async function
@Component
struct RustCallAsyncJsItem {
  @State rustJsCallbackAsyncRes: string = ""

  async getVersionName(): Promise<string> {
    let bundleFlags = bundleManager.BundleFlag.GET_BUNDLE_INFO_DEFAULT
    return new Promise((resolve, reject) => {
      bundleManager.getBundleInfoForSelf(bundleFlags)
        .then((data) => {
          log("getVersionName", 'promise resolve')
          resolve(data.versionName)
        })
        .catch((e: Error) => {
          reject(e)
        })
    });
  }

  build() {
    Column() {
      // rust js_callback async
      Text(`rust js callback get version name async res = ${this.rustJsCallbackAsyncRes}`)
        .margin({
          top: 20,
        })

      Button('rust js callback async')
        .onClick(async () => {
          let getVersionNamePromise = this.getVersionName()
          log("getVersionName", 'after get promise')
          this.rustJsCallbackAsyncRes = await rustLib.getVersionNameAsync(getVersionNamePromise)
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust call js function (出参入参都是struct)
@Component
struct RustCallJsParamStructItem {
  @State rustJsCallbackParamStructRes: string = ""

  build() {
    Column() {
      // rust js_callback with struct param
      Text(`rust js callback get version info res = ${this.rustJsCallbackParamStructRes}`)
        .margin({
          top: 20,
        })

      Button('rust js callback with struct param')
        .onClick(async () => {
          let versionInfoResponseModel =
            rustLib.getVersionInfo((request: VersionInfoRequestModel): VersionInfoResponseModel => {
              log("jsCallbackGetVersionInfo", `request_model: ${JSON.stringify(request)}`)
              return {
                versionName: "1.0.1",
                versionCode: 2,
              }
            })

          this.rustJsCallbackParamStructRes = JSON.stringify(versionInfoResponseModel)
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

// 测试 rust call js async function (出参入参都是struct)
@Component
struct RustCallAsyncJsParamStructItem {
  @State rustJsCallbackAsyncParamStructRes: string = ""

  getVersionNameWithInput(err: Error | null, requestModel: VersionInfoRequestModel): Promise<VersionInfoResponseModel> {
    log("getVersionNameWithInput", `request_model: ${JSON.stringify(requestModel)}`)
    return new Promise((resolve, reject) => {
      if (err != null) {
        reject(err)
        return
      }
      if (requestModel.intParam == 100) {
        resolve({
          versionName: "1.0.1",
          versionCode: 101,
        })
        return
      }

      let bundleFlags = bundleManager.BundleFlag.GET_BUNDLE_INFO_DEFAULT
      bundleManager.getBundleInfoForSelf(bundleFlags)
        .then((data) => {
          log("getVersionName", 'promise resolve')
          resolve({
            versionName: data.versionName,
            versionCode: data.versionCode,
          })
        })
        .catch((e: Error) => {
          reject(e)
        })
    });
  }

  build() {
    Column() {
      // rust js_callback async
      Text(`rust js callback get version info(param struct) async res = ${this.rustJsCallbackAsyncParamStructRes}`)
        .margin({
          top: 20,
        })

      Button('rust js callback async param struct')
        .onClick(async () => {
          try {
            let responseModel = await rustLib.getVersionInfoAsyncWithInput(this.getVersionNameWithInput)
            this.rustJsCallbackAsyncParamStructRes = JSON.stringify(responseModel)
          } catch (e) {
            log("getVersionInfoAsyncWithInput", 'error code: ${e.code}, error message: ${e.message}')
          }
        })
        .margin({
          top: 20,
        })
    }
    .width('100%')
  }
}

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  build() {
    List() {

      ListItem() {
        // 测试 rust init
        RustInitItem()
      }

      ListItem() {
        // 测试 rust add
        RustAddItem()
      }

      ListItem() {
        // 测试 rust async add
        RustAsyncAddItem()
      }

      ListItem() {
        // 测试 rust add input(入参和出参都是结构体)
        RustAddInputItem()
      }

      ListItem() {
        // 测试 rust call js function
        RustCallJsItem()
      }

      ListItem() {
        // 测试 rust call js async function
        RustCallAsyncJsItem()
      }

      ListItem() {
        // 测试 rust call js function （带struct入参和出参）
        RustCallJsParamStructItem()
      }

      ListItem() {
        // 测试 rust call js async function （带struct入参和出参）
        RustCallAsyncJsParamStructItem()
      }
    }
    .height('100%')
    .width('100%')
    .padding(10)
  }
}
