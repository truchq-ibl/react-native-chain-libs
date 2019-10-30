use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{Transaction, TransactionFinalizer, Witness};

#[no_mangle]
pub unsafe extern "C" fn transaction_finalizer_new(
  transaction: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    transaction.owned::<Transaction>().map(|transaction| TransactionFinalizer::new(transaction))
  })
  .map(|tx_finalizer| RPtr::new(tx_finalizer))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_finalizer_get_tx_sign_data_hash(
  tx_finalizer: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_finalizer
      .typed_ref::<TransactionFinalizer>()
      .map(|tx_finalizer| tx_finalizer.get_tx_sign_data_hash())
  })
  .map(|tx_sign_data_hash| RPtr::new(tx_sign_data_hash))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_finalizer_set_witness(
  tx_finalizer: RPtr, index: usize, witness: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_finalizer
      .typed_ref::<TransactionFinalizer>()
      .zip(witness.owned::<Witness>())
      .and_then(|(tx_finalizer, witness)| tx_finalizer.set_witness(index, witness).into_result())
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_finalizer_build(
  tx_finalizer: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_finalizer
      .owned::<TransactionFinalizer>()
      .and_then(|tx_finalizer| tx_finalizer.build().into_result())
  })
  .map(|auth_tx| RPtr::new(auth_tx))
  .response(result, error)
}
