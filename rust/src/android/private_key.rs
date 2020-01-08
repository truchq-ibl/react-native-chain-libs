use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use js_chain_libs::PrivateKey;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_privateKeyFromBech32(
  env: JNIEnv, _: JObject, bech32_str: JString
) -> jobject {
  handle_exception_result(|| {
    bech32_str
      .string(&env)
      .and_then(|bech32_str| PrivateKey::from_bech32(&bech32_str).into_result())
      .and_then(|private_key| private_key.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_privateKeyToPublic(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr.typed_ref::<PrivateKey>().and_then(|private_key| private_key.to_public().rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_privateKeyFromExtendedBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| PrivateKey::from_extended_bytes(&bytes).into_result())
      .and_then(|private_key| private_key.rptr().jptr(&env))
  })
  .jresult(&env)
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_privateKeyFromNomalBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| PrivateKey::from_normal_bytes(&bytes).into_result())
      .and_then(|private_key| private_key.rptr().jptr(&env))
  })
  .jresult(&env)
}