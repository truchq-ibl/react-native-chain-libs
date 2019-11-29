use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use js_chain_libs::TransactionSignDataHash;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionSignDataHashFromBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| TransactionSignDataHash::from_bytes(&bytes).into_result())
      .and_then(|tx_sign_data_hash| tx_sign_data_hash.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionSignDataHashFromHex(
  env: JNIEnv, _: JObject, input: JString
) -> jobject {
  handle_exception_result(|| {
    input
      .string(&env)
      .and_then(|input| TransactionSignDataHash::from_hex(&input).into_result())
      .and_then(|tx_sign_data_hash| tx_sign_data_hash.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionSignDataHashAsBytes(
  env: JNIEnv, _: JObject, tx_sign_data_hash: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_sign_data_hash = tx_sign_data_hash.rptr(&env)?;
    tx_sign_data_hash
      .typed_ref::<TransactionSignDataHash>()
      .map(|tx_sign_data_hash| tx_sign_data_hash.as_bytes())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
}
