use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result};
use crate::ptr::RPtr;
use js_chain_libs::{Address, OutputPolicy};

#[no_mangle]
pub unsafe extern "C" fn output_policy_forget(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(OutputPolicy::forget())).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn output_policy_one(
  address: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| address.owned::<Address>().map(|address| OutputPolicy::one(address)))
    .map(|output_policy| RPtr::new(output_policy))
    .response(result, error)
}
