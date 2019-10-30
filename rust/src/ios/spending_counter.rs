use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::RPtr;
use js_chain_libs::SpendingCounter;

#[no_mangle]
pub extern "C" fn spending_counter_zero(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(SpendingCounter::zero())).response(result, error)
}
