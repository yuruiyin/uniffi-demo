package com.example.uniffitest.utils

import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Locale

/**
 *
 * Created by kongran on 2024/7/18
 * E-Mail Address：kongran@gaoding.com
 */
object TimeUtil {

    /**
     * 获取格式化时后的时间
     */
    fun getFormatTime(): String {
        // 获取当前时间
        val currentTime = Calendar.getInstance().time

        // 定义日期格式
        val dateFormat = SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.getDefault())

        // 格式化时间
        return dateFormat.format(currentTime)
    }

}