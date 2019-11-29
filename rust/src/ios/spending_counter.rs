use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::SpendingCounter;

#[no_mangle]
pub extern "C" fn spending_counter_zero(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| SpendingCounter::zero().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn spending_counter_from_u32(
  counter: u32, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| SpendingCounter::from_u32(counter).rptr()).response(result, error)
}
