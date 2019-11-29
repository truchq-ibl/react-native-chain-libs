use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Address, OutputPolicy};

#[no_mangle]
pub unsafe extern "C" fn output_policy_forget(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| OutputPolicy::forget().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn output_policy_one(
  address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().map(|address| OutputPolicy::one(address))
  })
  .map(|output_policy| output_policy.rptr())
  .response(result, error)
}
