use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use js_chain_libs::FragmentId;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIdCalculate(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .map(|bytes| FragmentId::calculate(&bytes))
      .and_then(|fragment_id| fragment_id.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIdAsBytes(
  env: JNIEnv, _: JObject, fragment_id: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let fragment_id = fragment_id.rptr(&env)?;
    fragment_id
      .typed_ref::<FragmentId>()
      .map(|fragment_id| fragment_id.as_bytes())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
}
