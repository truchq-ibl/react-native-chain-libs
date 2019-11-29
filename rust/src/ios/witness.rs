use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{
  Bip32PrivateKey, Hash, PrivateKey, SpendingCounter, TransactionSignDataHash, Witness, Witnesses
};

#[no_mangle]
pub unsafe extern "C" fn witness_for_account(
  genesis_hash: RPtr, transaction_id: RPtr, secret_key: RPtr, account_spending_counter: RPtr,
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<PrivateKey>())
      .zip(account_spending_counter.typed_ref::<SpendingCounter>())
      .map(|(((genesis_hash, transaction_id), secret_key), account_spending_counter)| {
        Witness::for_account(genesis_hash, transaction_id, secret_key, account_spending_counter)
      })
  })
  .map(|witness| witness.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witness_for_utxo(
  genesis_hash: RPtr, transaction_id: RPtr, secret_key: RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<PrivateKey>())
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_utxo(genesis_hash, transaction_id, secret_key)
      })
  })
  .map(|witness| witness.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn witness_for_legacy_icarus_utxo(
  genesis_hash: RPtr, transaction_id: RPtr, secret_key: RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    genesis_hash
      .typed_ref::<Hash>()
      .zip(transaction_id.typed_ref::<TransactionSignDataHash>())
      .zip(secret_key.typed_ref::<Bip32PrivateKey>())
      .map(|((genesis_hash, transaction_id), secret_key)| {
        Witness::for_legacy_icarus_utxo(genesis_hash, transaction_id, secret_key)
      })
  })
  .map(|witness| witness.rptr())
  .response(result, error)
}

#[no_mangle]
pub extern "C" fn witnesses_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| Witnesses::new().rptr()).response(result, error)
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
  .map(|witness| witness.rptr())
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
