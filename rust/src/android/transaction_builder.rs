use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Address, Input, TransactionBuilder, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderNewNoPayload(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| RPtr::new(TransactionBuilder::new_no_payload()).jptr(&env))
    .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderAddInput(
  env: JNIEnv, _: JObject, tx_builder: JRPtr, input: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_builder = tx_builder.rptr(&env)?;
    tx_builder
      .typed_ref::<TransactionBuilder>()
      .zip(input.owned::<Input>(&env))
      .map(|(tx_builder, input)| tx_builder.add_input(input))
  })
  .map(|_| JObject::null())
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderAddOutput(
  env: JNIEnv, _: JObject, tx_builder: JRPtr, address: JRPtr, value: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_builder = tx_builder.rptr(&env)?;
    tx_builder
      .typed_ref::<TransactionBuilder>()
      .zip(address.owned::<Address>(&env))
      .zip(value.owned::<Value>(&env))
      .map(|((tx_builder, address), value)| tx_builder.add_output(address, value))
  })
  .map(|_| JObject::null())
  .jresult(&env)
}
