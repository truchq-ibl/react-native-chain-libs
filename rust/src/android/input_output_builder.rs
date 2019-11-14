use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Address, Fee, Input, InputOutputBuilder, OutputPolicy, Payload, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputBuilderEmpty(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| RPtr::new(InputOutputBuilder::empty()).jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputBuilderAddInput(
  env: JNIEnv, _: JObject, io_builder: JRPtr, input: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let io_builder = io_builder.rptr(&env)?;
    let input = input.rptr(&env)?;
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(input.typed_ref::<Input>())
      .and_then(|(io_builder, input)| io_builder.add_input(input).into_result())
  })
  .map(|_| JObject::null())
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputBuilderAddOutput(
  env: JNIEnv, _: JObject, io_builder: JRPtr, address: JRPtr, value: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let io_builder = io_builder.rptr(&env)?;
    io_builder
      .typed_ref::<InputOutputBuilder>()
      .zip(address.owned::<Address>(&env))
      .zip(value.owned::<Value>(&env))
      .and_then(|((io_builder, address), value)| {
        io_builder.add_output(address, value).into_result()
      })
  })
  .map(|_| JObject::null())
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputBuilderBuild(
  env: JNIEnv, _: JObject, io_builder: JRPtr
) -> jobject {
  handle_exception_result(|| {
    io_builder
      .owned::<InputOutputBuilder>(&env)
      .map(|io_builder| io_builder.build())
      .and_then(|input_output| RPtr::new(input_output).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputOutputBuilderSealWithOutputPolicy(
  env: JNIEnv, _: JObject, io_builder: JRPtr, payload: JRPtr, fee_algorithm: JRPtr, policy: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let payload = payload.rptr(&env)?;
    io_builder
      .owned::<InputOutputBuilder>(&env)
      .zip(payload.typed_ref::<Payload>())
      .zip(fee_algorithm.owned::<Fee>(&env))
      .zip(policy.owned::<OutputPolicy>(&env))
      .and_then(|(((io_builder, payload), fee_algorithm), policy)| {
        io_builder.seal_with_output_policy(payload, fee_algorithm, policy).into_result()
      })
      .and_then(|input_output| RPtr::new(input_output).jptr(&env))
  })
  .jresult(&env)
}
