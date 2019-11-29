use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::InputOutput;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputInputs(
  env: JNIEnv, _: JObject, input_output: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let input_output = input_output.rptr(&env)?;
    input_output
      .typed_ref::<InputOutput>()
      .map(|input_output| input_output.inputs())
      .and_then(|inputs| inputs.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputOutputs(
  env: JNIEnv, _: JObject, input_output: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let input_output = input_output.rptr(&env)?;
    input_output
      .typed_ref::<InputOutput>()
      .map(|input_output| input_output.outputs())
      .and_then(|outputs| outputs.rptr().jptr(&env))
  })
  .jresult(&env)
}
