use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Fee, Transaction, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_feeLinearFee(
  env: JNIEnv, _: JObject, constant: JRPtr, coefficient: JRPtr, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    constant
      .owned::<Value>(&env)
      .zip(coefficient.owned::<Value>(&env))
      .zip(certificate.owned::<Value>(&env))
      .map(|((constant, coefficient), certificate)| {
        Fee::linear_fee(constant, coefficient, certificate)
      })
      .and_then(|fee| RPtr::new(fee).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_feeCalculate(
  env: JNIEnv, _: JObject, fee: JRPtr, tx: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let fee = fee.rptr(&env)?;
    fee
      .typed_ref::<Fee>()
      .zip(tx.owned::<Transaction>(&env))
      .and_then(|(fee, tx)| fee.calculate(tx).ok_or(String::from("Cannot calculate fee")))
      .and_then(|value| RPtr::new(value).jptr(&env))
  })
  .jresult(&env)
}
