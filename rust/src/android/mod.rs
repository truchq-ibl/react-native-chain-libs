
mod ptr_j;
mod result;
mod string;
mod value;
mod public_key;
mod address;

pub use value::*;
pub use public_key::*;
pub use address::*;

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_io_emurgo_chainlibs_Native_initLibrary(_env: jni::JNIEnv, _: jni::objects::JObject) {
  crate::panic::hide_exceptions();
}