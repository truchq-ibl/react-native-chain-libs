use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Address, OutputPolicy};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_outputPolicyForget(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| RPtr::new(OutputPolicy::forget()).jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputPolicyOne(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    address
      .owned::<Address>(&env)
      .and_then(|address| RPtr::new(OutputPolicy::one(address)).jptr(&env))
  })
  .jresult(&env)
}
