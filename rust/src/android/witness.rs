use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Hash, PrivateKey, SpendingCounter, TransactionSignDataHash, Witness};

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
