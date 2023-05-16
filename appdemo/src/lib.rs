use std::os::raw::c_char;
use std::ffi::CString;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::cmp::Ordering;
use rand::Rng;

pub struct Game {
    number: i32,
}

// Global
static GUESS_NUMBER: Lazy<Mutex<Game>> = Lazy::new(|| {
    Mutex::new(Game {
        number: 0,
    })
});

#[no_mangle]
pub extern fn create_secret_number() {
    GUESS_NUMBER.lock().unwrap().number = rand::thread_rng().gen_range(1..100)
}

#[no_mangle]
pub extern fn guess(input: i32) -> *mut c_char {
    let result = match input.cmp(&GUESS_NUMBER.lock().unwrap().number) {
        Ordering::Less => "小さい",       //小さすぎ！
        Ordering::Greater => "大きい",      //大きすぎ！
        Ordering::Equal => "やった!",        //やったね！
    };

    CString::new(result).unwrap().into_raw()
}

// Androidだけのため
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::JClass;
    use self::jni::sys::{jstring, jint};
    use std::os::raw::c_int;

    #[no_mangle]
    pub unsafe extern fn Java_reinf0rce_androiddemo_RustGame_guess(env: JNIEnv, _: JClass, android_input: jint) -> jstring {
        let i32_value: i32 = android_input as c_int as i32;
        let result = guess(i32_value);
        let result_pointer = CString::from_raw(result);
        let output = env.new_string(result_pointer.to_str().unwrap()).expect("Couldn't get result from Android Function!");
        output.into_raw()
    }

    #[no_mangle]
    pub unsafe extern fn Java_reinf0rce_androiddemo_RustGame_createRandomNumber(_: JNIEnv, _: JClass) {
        create_secret_number()
    }
}