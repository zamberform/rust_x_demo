package reinf0rce.androiddemo

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.core.widget.doAfterTextChanged
import com.google.android.material.snackbar.Snackbar
import reinf0rce.androiddemo.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val rustGame = RustGame()

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        binding.createBtn.setOnClickListener {
            rustGame.randomNum()
        }

        binding.toolbar.title = "数当て"

        binding.guessNumber.doAfterTextChanged {

            val inputStr = it.toString()
            if (inputStr.isNotEmpty()) {
                binding.result.text = rustGame.guessNum(inputStr.toInt())
            }
        }
    }
}