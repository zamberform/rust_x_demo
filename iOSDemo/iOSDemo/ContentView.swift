//
//  ContentView.swift
//  iOSDemo
//
//  Created by Reinf0rce on 2023/05/15.
//

import SwiftUI

struct ContentView: View {
    let rustGame = RustGame()
    
    @State private var guessValue = 0
    
    var body: some View {
        VStack {
            Spacer()
            Text("数当て")
                .font(.title)
                .underline()
            Spacer()
            Button(action: {
                rustGame.createRandomNumber()
                guessValue = 0
            }){
                Text("乱数生成")
                    .font(.title3)
                    .foregroundColor(Color.red)
                    .frame(width: 160, height: 40, alignment: .center)
                    .overlay(
                        RoundedRectangle(cornerRadius: 50)
                            .stroke(Color.yellow, lineWidth: 2)
                    )
            }
            TextField("数字を入力:", value: $guessValue, format: .number)
                .textFieldStyle(.roundedBorder)
                .font(.title)
                .multilineTextAlignment(.center)
                .padding()
            Text("*1-100の数字を入れてください。")
                .font(.subheadline)
                .foregroundColor(Color.gray)
            Spacer()
            Text("結果は： \(rustGame.guessNumber(input: Int32(guessValue))).")
                .font(.title)
                .foregroundColor(Color.mint)

            Spacer()
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
