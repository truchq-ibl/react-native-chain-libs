use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{
  Hash, PrivateKey, SpendingCounter, TransactionSignDataHash, Witness, Witnesses
};

#[no_mangle]
pub unsafe extern "C" fn witness_for_account(
  genesis_hash: &mut RPtr, transaction_id: &mut RPtr, secret_key: &mut RPtr,
  account_spending_counter: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    genesis_hash
      .owned::<Hash>()
      .zip(transaction_id.owned::<TransactionSignDataHash>())
      .zip(secret_key.owned::<PrivateKey>())
      .zip(account_spending_counter.owned::<SpendingCounter>())
      .map(|(((genesis_hash, transaction_id), secret_key), account_spending_counter)| {
        Witness::for_account(genesis_hash, transaction_id, secret_key, account_spending_counter)
      })
  })
  .map(|witness| RPtr::new(witness))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witness_for_utxo(
  genesis_hash: &mut RPtr, transaction_id: &mut RPtr, secret_key: &mut RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    genesis_hash
      .owned::<Hash>()
      .zip(transaction_id.owned::<TransactionSignDataHash>())
      .zip(secret_key.owned::<PrivateKey>())
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_utxo(genesis_hash, transaction_id, secret_key)
      })
  })
  .map(|witness| RPtr::new(witness))
  .response(result, error)
}

#[no_mangle]
pub extern "C" fn witnesses_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(Witnesses::new())).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witnesses_size(
  witnesses: RPtr, result: &mut usize, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| witnesses.typed_ref::<Witnesses>().map(|witnesses| witnesses.size()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witnesses_get(
  witnesses: RPtr, index: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    witnesses.typed_ref::<Witnesses>().map(|witnesses| witnesses.get(index))
  })
  .map(|witness| RPtr::new(witness))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witnesses_add(
  witnesses: RPtr, item: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    witnesses
      .typed_ref::<Witnesses>()
      .zip(item.owned::<Witness>())
      .map(|(witnesses, item)| witnesses.add(item))
  })
  .response(&mut (), error)
}
