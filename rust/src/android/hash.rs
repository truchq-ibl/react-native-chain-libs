use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::Hash;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_hashFromHex(
  env: JNIEnv, _: JObject, hex_string: JString
) -> jobject {
  handle_exception_result(|| {
    hex_string
      .string(&env)
      .and_then(|hex_string| Hash::from_hex(&hex_string).into_result())
      .and_then(|hash| hash.rptr().jptr(&env))
  })
  .jresult(&env)
}
