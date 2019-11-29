use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Address, OutputPolicy};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_outputPolicyForget(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| OutputPolicy::forget().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputPolicyOne(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address.typed_ref::<Address>().and_then(|address| OutputPolicy::one(address).rptr().jptr(&env))
  })
  .jresult(&env)
}
