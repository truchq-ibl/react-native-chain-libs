use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jboolean, jobject};
use jni::JNIEnv;
use js_chain_libs::Balance;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_balanceIsPositive(
  env: JNIEnv, _: JObject, balance: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let balance = balance.rptr(&env)?;
    balance
      .typed_ref::<Balance>()
      .and_then(|balance| (balance.is_positive() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_balanceIsNegative(
  env: JNIEnv, _: JObject, balance: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let balance = balance.rptr(&env)?;
    balance
      .typed_ref::<Balance>()
      .and_then(|balance| (balance.is_negative() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_balanceIsZero(
  env: JNIEnv, _: JObject, balance: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let balance = balance.rptr(&env)?;
    balance.typed_ref::<Balance>().and_then(|balance| (balance.is_zero() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_balanceGetValue(
  env: JNIEnv, _: JObject, balance: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let balance = balance.rptr(&env)?;
    balance.typed_ref::<Balance>().and_then(|balance| RPtr::new(balance.get_value()).jptr(&env))
  })
  .jresult(&env)
}
