use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception_result;
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::Transaction;

#[no_mangle]
pub unsafe extern "C" fn transaction_id(
  transaction: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    transaction.typed_ref::<Transaction>().map(|transaction| transaction.id())
  })
  .map(|tx_sign_data_hash| tx_sign_data_hash.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_inputs(
  transaction: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    transaction.typed_ref::<Transaction>().map(|transaction| transaction.inputs())
  })
  .map(|inputs| inputs.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_outputs(
  transaction: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    transaction.typed_ref::<Transaction>().map(|transaction| transaction.outputs())
  })
  .map(|outputs| outputs.rptr())
  .response(result, error)
}
