package com.example.uniffitest

import android.annotation.SuppressLint
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.example.uniffitest.ui.theme.UniffiTestTheme
import com.example.uniffitest.utils.TimeUtil
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import uniffi.rust_lib.AndroidConfig
import uniffi.rust_lib.AndroidDelegate
import uniffi.rust_lib.CallbackTrait
import uniffi.rust_lib.Input
import kotlin.coroutines.coroutineContext

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            UniffiTestTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
//                    Greeting("Android, from rust: add(1, 5) = ${output.result}")
                    Greeting("Android")
                }
            }
        }
    }
}

@SuppressLint("CoroutineCreationDuringComposition")
@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    val scope = rememberCoroutineScope()

    // 初始化rust，将callback传到rust侧，rust侧持有，这样rust可以直接调用kotlin进而获取到android平台的能力
    scope.launch {
        uniffi.rust_lib.register(object : AndroidDelegate {
            override suspend fun getAndroidConfig(): AndroidConfig {
                return AndroidConfig(
                    version = Build.VERSION.SDK_INT,
                    brand = Build.BRAND,
                    model = Build.MODEL,
                )
            }

            override suspend fun getCurrentActivity(): String {
                return "MainActivity"
            }

            override suspend fun logD(tag: String, message: String) {
                Log.d(tag, message)
            }

            override suspend fun getSystemTime(): String {
                // 获取格式化后的系统时间 如 2024-01-01 12:00:00
                return TimeUtil.getFormatTime()
            }

        })
    }


    val addNum1 = 7
    val addNum2 = 6

    // 加法：input和output都是一个对象
    val addRes = uniffi.rust_lib.addInput(Input(addNum1, addNum2))

    // 乘法
    val multiplyRes = uniffi.rust_lib.MathManager().multiply(addNum1, addNum2)

    // 回调函数
    uniffi.rust_lib.MathManager().testCallback(object : CallbackTrait {
        override fun call(message: String) {
            println("callback from rust: $message")
        }
    })

    // async
    scope.launch(Dispatchers.IO) {
        val res = uniffi.rust_lib.asyncAdd(4, 2)
        println("async add result: $res")
    }

    // 测试rust的result
    scope.launch {
        try {
            val res = uniffi.rust_lib.isOdd(-1)
        } catch (e: Exception) {
            println("too small res: $e")
        }
        try {
            val res = uniffi.rust_lib.isOdd(101)
        } catch (e: Exception) {
            println("too big res: $e")
        }

        val oddRes = uniffi.rust_lib.isOdd(97)
        println("odd res: $oddRes")
        val evenRes = uniffi.rust_lib.isOdd(98)
        println("even res: $evenRes")
    }

    Text(
        text = "Hello from rust: add($addNum1, $addNum2) = ${addRes.result}, multiply($addNum1, $addNum2) = ${multiplyRes}!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    UniffiTestTheme {
        Greeting("Android2222")
    }
}