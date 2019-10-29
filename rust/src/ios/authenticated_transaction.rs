use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use js_chain_libs::AuthenticatedTransaction;

#[no_mangle]
pub unsafe extern "C" fn authenticated_transaction_transaction(
  auth_tx: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    auth_tx.typed_ref::<AuthenticatedTransaction>().map(|auth_tx| auth_tx.transaction())
  })
  .map(|tx| RPtr::new(tx))
  .response(result, error)
}
