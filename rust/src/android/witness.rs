use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use js_chain_libs::{
  Bip32PrivateKey, Hash, PrivateKey, SpendingCounter, TransactionSignDataHash, Witness, Witnesses
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessForAccount(
  env: JNIEnv, _: JObject, genesis_hash: JRPtr, transaction_id: JRPtr, secret_key: JRPtr,
  account_spending_counter: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let genesis_hash = genesis_hash.rptr(&env)?;
    let transaction_id = transaction_id.rptr(&env)?;
    let secret_key = secret_key.rptr(&env)?;
    let account_spending_counter = account_spending_counter.rptr(&env)?;
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<PrivateKey>())
      .zip(account_spending_counter.typed_ref::<SpendingCounter>())
      .map(|(((genesis_hash, transaction_id), secret_key), account_spending_counter)| {
        Witness::for_account(genesis_hash, transaction_id, secret_key, account_spending_counter)
      })
      .and_then(|witness| witness.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessForUtxo(
  env: JNIEnv, _: JObject, genesis_hash: JRPtr, transaction_id: JRPtr, secret_key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let genesis_hash = genesis_hash.rptr(&env)?;
    let transaction_id = transaction_id.rptr(&env)?;
    let secret_key = secret_key.rptr(&env)?;
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<PrivateKey>())
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_utxo(genesis_hash, transaction_id, secret_key)
      })
      .and_then(|witness| witness.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessForLegacyIcarusUtxo(
  env: JNIEnv, _: JObject, genesis_hash: JRPtr, transaction_id: JRPtr, secret_key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let genesis_hash = genesis_hash.rptr(&env)?;
    let transaction_id = transaction_id.rptr(&env)?;
    let secret_key = secret_key.rptr(&env)?;
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<Bip32PrivateKey>())
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_legacy_icarus_utxo(genesis_hash, transaction_id, secret_key)
      })
      .and_then(|witness| witness.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_witnessesNew(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| Witnesses::new().rptr().jptr(&env)).jresult(&env)
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
      .and_then(|witness| witness.rptr().jptr(&env))
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
