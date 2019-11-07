use super::result::CResult;
use super::string::CharPtr;
use crate::js_chain_libs::Balance;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;

#[no_mangle]
pub unsafe extern "C" fn balance_is_positive(
  balance: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| balance.typed_ref::<Balance>().map(|balance| balance.is_positive()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn balance_is_negative(
  balance: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| balance.typed_ref::<Balance>().map(|balance| balance.is_negative()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn balance_is_zero(
  balance: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| balance.typed_ref::<Balance>().map(|balance| balance.is_zero()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn balance_get_value(
  balance: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| balance.typed_ref::<Balance>().map(|balance| balance.get_value()))
    .map(|value| RPtr::new(value))
    .response(result, error)
}
