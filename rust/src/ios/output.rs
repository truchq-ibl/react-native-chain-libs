use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Output, Outputs};

#[no_mangle]
pub unsafe extern "C" fn output_address(
  output: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| output.typed_ref::<Output>().map(|output| output.address()))
    .map(|address| address.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn output_value(
  output: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| output.typed_ref::<Output>().map(|output| output.value()))
    .map(|value| value.rptr())
    .response(result, error)
}

#[no_mangle]
pub extern "C" fn outputs_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| Outputs::new().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn outputs_size(
  outputs: RPtr, result: &mut usize, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| outputs.typed_ref::<Outputs>().map(|outputs| outputs.size()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn outputs_get(
  outputs: RPtr, index: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| outputs.typed_ref::<Outputs>().map(|outputs| outputs.get(index)))
    .map(|output| output.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn outputs_add(outputs: RPtr, item: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| {
    outputs
      .typed_ref::<Outputs>()
      .zip(item.owned::<Output>())
      .map(|(outputs, item)| outputs.add(item))
  })
  .response(&mut (), error)
}
