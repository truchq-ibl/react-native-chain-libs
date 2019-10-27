use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jobject;

use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::*;

use crate::js_chain_libs::PublicKey;

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_io_emurgo_chainlibs_Native_publicKeyFromBech32(
  env: JNIEnv, _: JObject, bech32_str: JString
) -> jobject {
  handle_exception_result(|| {
    let rstr = bech32_str.string(&env)?;
    let val = PublicKey::from_bech32(&rstr).into_result()?;
    RPtr::new(val).jptr(&env)
  }).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_io_emurgo_chainlibs_Native_publicKeyAsBytes(
  env: JNIEnv, _: JObject, ptr: JObject
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    let pubkey = rptr.typed_ref::<PublicKey>()?;
    env
      .byte_array_from_slice(&pubkey.as_bytes())
      .map(|arr| JObject::from(arr))
      .into_result()
  }).jresult(&env)
}