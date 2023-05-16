//
//  RustGame.swift
//  iOSDemo
//
//  Created by Reinf0rce on 2023/05/15.
//

import Foundation

class RustGame {

    func createRandomNumber() {
        create_secret_number()
    }
    func guessNumber(input: Int32) -> String {
        let result = guess(Int32(input))
        let swift_result = String(cString: result!)
        return swift_result
    }
}

