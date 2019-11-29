use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Account, Input, Inputs, Value};

#[no_mangle]
pub unsafe extern "C" fn input_from_account(
  account: RPtr, v: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    account
      .typed_ref::<Account>()
      .zip(v.typed_ref::<Value>())
      .map(|(account, v)| Input::from_account(account, v))
  })
  .map(|input| input.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_value(input: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| input.typed_ref::<Input>().map(|input| input.value()))
    .map(|value| value.rptr())
    .response(result, error)
}

#[no_mangle]
pub extern "C" fn inputs_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| Inputs::new().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn inputs_size(
  inputs: RPtr, result: &mut usize, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| inputs.typed_ref::<Inputs>().map(|inputs| inputs.size()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn inputs_get(
  inputs: RPtr, index: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| inputs.typed_ref::<Inputs>().map(|inputs| inputs.get(index)))
    .map(|input| input.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn inputs_add(inputs: RPtr, item: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| {
    inputs.typed_ref::<Inputs>().zip(item.owned::<Input>()).map(|(inputs, item)| inputs.add(item))
  })
  .response(&mut (), error)
}
