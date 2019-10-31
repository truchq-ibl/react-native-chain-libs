use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use js_chain_libs::{Output, Outputs};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputAddress(
  env: JNIEnv, _: JObject, output: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let output = output.rptr(&env)?;
    output
      .typed_ref::<Output>()
      .map(|output| output.address())
      .and_then(|address| RPtr::new(address).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputValue(
  env: JNIEnv, _: JObject, output: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let output = output.rptr(&env)?;
    output
      .typed_ref::<Output>()
      .map(|output| output.value())
      .and_then(|value| RPtr::new(value).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputsSize(
  env: JNIEnv, _: JObject, outputs: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let outputs = outputs.rptr(&env)?;
    outputs
      .typed_ref::<Outputs>()
      .map(|outputs| outputs.size())
      .and_then(|size| size.into_jlong().jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_outputsGet(
  env: JNIEnv, _: JObject, outputs: JRPtr, index: jlong
) -> jobject {
  handle_exception_result(|| {
    let outputs = outputs.rptr(&env)?;
    outputs
      .typed_ref::<Outputs>()
      .map(|outputs| outputs.get(usize::from_jlong(index)))
      .and_then(|output| RPtr::new(output).jptr(&env))
  })
  .jresult(&env)
}
