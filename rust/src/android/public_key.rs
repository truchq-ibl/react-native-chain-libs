use jni::objects::{JObject, JString};
use jni::sys::jobject;
use jni::JNIEnv;

use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::*;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;

use crate::js_chain_libs::PublicKey;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_publicKeyFromBech32(
  env: JNIEnv, _: JObject, bech32_str: JString
) -> jobject {
  handle_exception_result(|| {
    let rstr = bech32_str.string(&env)?;
    let val = PublicKey::from_bech32(&rstr).into_result()?;
    val.rptr().jptr(&env)
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_publicKeyAsBytes(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    let pubkey = rptr.typed_ref::<PublicKey>()?;
    env.byte_array_from_slice(&pubkey.as_bytes()).map(|arr| JObject::from(arr)).into_result()
  })
  .jresult(&env)
}
