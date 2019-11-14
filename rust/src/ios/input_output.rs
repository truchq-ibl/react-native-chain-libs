use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use js_chain_libs::InputOutput;

#[no_mangle]
pub unsafe extern "C" fn input_output_inputs(
  input_output: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    input_output.typed_ref::<InputOutput>().map(|input_output| input_output.inputs())
  })
  .map(|inputs| RPtr::new(inputs))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn input_output_outputs(
  input_output: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    input_output.typed_ref::<InputOutput>().map(|input_output| input_output.outputs())
  })
  .map(|outputs| RPtr::new(outputs))
  .response(result, error)
}
