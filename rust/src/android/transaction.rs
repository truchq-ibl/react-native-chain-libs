use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::Transaction;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionId(
  env: JNIEnv, _: JObject, transaction: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let transaction = transaction.rptr(&env)?;
    transaction
      .typed_ref::<Transaction>()
      .and_then(|transaction| transaction.id().rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionInputs(
  env: JNIEnv, _: JObject, transaction: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let transaction = transaction.rptr(&env)?;
    transaction
      .typed_ref::<Transaction>()
      .and_then(|transaction| transaction.inputs().rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionOutputs(
  env: JNIEnv, _: JObject, transaction: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let transaction = transaction.rptr(&env)?;
    transaction
      .typed_ref::<Transaction>()
      .and_then(|transaction| transaction.outputs().rptr().jptr(&env))
  })
  .jresult(&env)
}
