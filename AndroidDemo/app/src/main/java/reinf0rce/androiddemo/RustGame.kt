package reinf0rce.androiddemo

class RustGame {

    companion object {
        init {
            System.loadLibrary("appdemo")
        }
        @JvmStatic
        private external fun createRandomNumber()
        @JvmStatic
        private external fun guess(input: Int): String
    }

    fun randomNum() {
        createRandomNumber()
    }

    fun guessNum(input: Int): String {
        return guess(input)
    }
}