use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use js_chain_libs::{
  Hash, PrivateKey, SpendingCounter, TransactionSignDataHash, Witness, Witnesses
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessForAccount(
  env: JNIEnv, _: JObject, genesis_hash: JRPtr, transaction_id: JRPtr, secret_key: JRPtr,
  account_spending_counter: JRPtr
) -> jobject {
  handle_exception_result(|| {
    genesis_hash
      .owned::<Hash>(&env)
      .zip(transaction_id.owned::<TransactionSignDataHash>(&env))
      .zip(secret_key.owned::<PrivateKey>(&env))
      .zip(account_spending_counter.owned::<SpendingCounter>(&env))
      .map(|(((genesis_hash, transaction_id), secret_key), account_spending_counter)| {
        Witness::for_account(genesis_hash, transaction_id, secret_key, account_spending_counter)
      })
      .and_then(|witness| RPtr::new(witness).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessForUtxo(
  env: JNIEnv, _: JObject, genesis_hash: JRPtr, transaction_id: JRPtr, secret_key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    genesis_hash
      .owned::<Hash>(&env)
      .zip(transaction_id.owned::<TransactionSignDataHash>(&env))
      .zip(secret_key.owned::<PrivateKey>(&env))
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_utxo(genesis_hash, transaction_id, secret_key)
      })
      .and_then(|witness| RPtr::new(witness).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessesNew(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| RPtr::new(Witnesses::new()).jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessesSize(
  env: JNIEnv, _: JObject, witnesses: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let witnesses = witnesses.rptr(&env)?;
    witnesses
      .typed_ref::<Witnesses>()
      .map(|witnesses| witnesses.size())
      .and_then(|size| size.into_jlong().jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessesGet(
  env: JNIEnv, _: JObject, witnesses: JRPtr, index: jlong
) -> jobject {
  handle_exception_result(|| {
    let witnesses = witnesses.rptr(&env)?;
    witnesses
      .typed_ref::<Witnesses>()
      .map(|witnesses| witnesses.get(usize::from_jlong(index)))
      .and_then(|witness| RPtr::new(witness).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessesAdd(
  env: JNIEnv, _: JObject, witnesses: JRPtr, item: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let witnesses = witnesses.rptr(&env)?;
    witnesses
      .typed_ref::<Witnesses>()
      .zip(item.owned::<Witness>(&env))
      .map(|(witnesses, item)| witnesses.add(item))
  })
  .map(|_| JObject::null())
  .jresult(&env)
}
