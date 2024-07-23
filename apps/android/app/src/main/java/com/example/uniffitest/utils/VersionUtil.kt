package com.example.uniffitest.utils

import android.content.Context
import android.content.pm.PackageManager

/**
 * 版本号相关工具方法
 * Created by yuruiyin on 2024/7/22
 */

fun Context.getVersionName(): String {
    return try {
        val packageInfo = this.packageManager.getPackageInfo(this.packageName, 0)
        packageInfo.versionName
    } catch (e: PackageManager.NameNotFoundException) {
        e.printStackTrace()
        "null"
    }
}