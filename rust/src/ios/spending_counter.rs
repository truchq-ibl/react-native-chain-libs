use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::RPtr;
use js_chain_libs::SpendingCounter;

#[no_mangle]
pub extern "C" fn spending_counter_zero(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(SpendingCounter::zero())).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn spending_counter_from_u32(
  counter: u32, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| RPtr::new(SpendingCounter::from_u32(counter))).response(result, error)
}
