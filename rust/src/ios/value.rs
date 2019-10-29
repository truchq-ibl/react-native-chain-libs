use super::result::CResult;
use super::string::*;
use crate::js_chain_libs::Value;
use crate::panic::*;
use crate::ptr::RPtr;

#[no_mangle]
pub unsafe extern "C" fn value_checked_add(
  value: RPtr, other: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    let val = value.typed_ref::<Value>()?;
    let oth = other.typed_ref::<Value>()?;
    val.checked_add(oth).map(|val| RPtr::new(val)).into_result()
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn value_from_str(
  chars: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    Value::from_str(chars.into_str()).map(|val| RPtr::new(val)).into_result()
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn value_from_u64(u: u64, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(Value::from(u))).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn value_to_str(
  value: RPtr, result: &mut CharPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| value.typed_ref::<Value>().map(|val| val.to_str().into_cstr()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn value_checked_sub(
  value: RPtr, other: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    let val = value.typed_ref::<Value>()?;
    let oth = other.typed_ref::<Value>()?;
    val.checked_sub(oth).into_result().map(|val| RPtr::new(val))
  })
  .response(result, error)
}
