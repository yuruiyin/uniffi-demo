//
//  ContentView.swift
//  UniffiTest
//
//  Created by 余瑞银 on 2024/7/20.
//

import SwiftUI

class RustDelegate: AppDelegate {
    func getAppConfig() async -> AppConfig {
        let version = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String
        return AppConfig(version: version!, env: "fat", userId: "123456")
    }
    
    func getCurrentPage() async -> String {
        return "ios page"
    }
    
    func logD(tag: String, message: String) async {
        print("\(tag), \(message)")
    }
    
    func getSystemTime() async -> String {
        let currentDate = Date()
        let dateFormatter = DateFormatter()
        dateFormatter.dateStyle = .medium
        dateFormatter.timeStyle = .medium

        let formattedDate = dateFormatter.string(from: currentDate)
        return formattedDate
    }
    
}

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, worlduuu!")
                .onAppear(perform: {
                    Task {
                        // 初始化rust，将callback传到rust侧，rust侧持有，这样rust可以直接调用swift进而获取到iOS平台的能力
                        do {
                            try await register(delegate: RustDelegate())
                        } catch let error {
                            print("init error: \(error)")
                        }
//                        try! await `init`(delegate: RustDelegate())
                        
                        // async add
                        let asyncAddRes = await asyncAdd(left: 1, right: 2)
                        print("asyncAdd: \(asyncAddRes)")
                        
                        // async minus
                        let asyncMinusRes = await asyncMinus(left: 6, right: 2)
                        print("asyncMinus: \(asyncMinusRes)")
                        
                        // 乘法
                        let mathManager = MathManager()
                        let multiplyRes = mathManager.multiply(left: 2, right: 3)
                        print("multiply: \(multiplyRes)")
                        
                        // 回调函数
                        class Callback: CallbackTrait {
                            func call(message: String) {
                                print("message: \(message)")
                            }
                        }
                        mathManager.testCallback(callback: Callback())
                        
                        //
                    }
                })
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
