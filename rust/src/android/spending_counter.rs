use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use js_chain_libs::SpendingCounter;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_spendingCounterZero(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| SpendingCounter::zero().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_spendingCounterFromU32(
  env: JNIEnv, _: JObject, counter: jint
) -> jobject {
  handle_exception_result(|| SpendingCounter::from_u32(counter as u32).rptr().jptr(&env))
    .jresult(&env)
}
