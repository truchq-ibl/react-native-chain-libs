use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use js_chain_libs::{Transaction, TransactionFinalizer, Witness};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionFinalizerNew(
  env: JNIEnv, _: JObject, transaction: JRPtr
) -> jobject {
  handle_exception_result(|| {
    transaction
      .owned::<Transaction>(&env)
      .and_then(|transaction| RPtr::new(TransactionFinalizer::new(transaction)).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionFinalizerGetTxSignDataHash(
  env: JNIEnv, _: JObject, tx_finalizer: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_finalizer = tx_finalizer.rptr(&env)?;
    tx_finalizer
      .typed_ref::<TransactionFinalizer>()
      .and_then(|tx_finalizer| RPtr::new(tx_finalizer.get_tx_sign_data_hash()).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionFinalizerSetWitness(
  env: JNIEnv, _: JObject, tx_finalizer: JRPtr, index: jlong, witness: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_finalizer = tx_finalizer.rptr(&env)?;
    tx_finalizer.typed_ref::<TransactionFinalizer>().zip(witness.owned::<Witness>(&env)).and_then(
      |(tx_finalizer, witness)| {
        tx_finalizer.set_witness(usize::from_jlong(index), witness).into_result()
      }
    )
  })
  .map(|_| JObject::null())
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionFinalizerBuild(
  env: JNIEnv, _: JObject, tx_finalizer: JRPtr
) -> jobject {
  handle_exception_result(|| {
    tx_finalizer
      .owned::<TransactionFinalizer>(&env)
      .and_then(|tx_finalizer| tx_finalizer.build().into_result())
      .and_then(|auth_tx| RPtr::new(auth_tx).jptr(&env))
  })
  .jresult(&env)
}
