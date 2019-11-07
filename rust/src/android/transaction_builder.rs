use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Address, Fee, Input, OutputPolicy, TransactionBuilder, Value};

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

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderGetBalance(
  env: JNIEnv, _: JObject, tx_builder: JRPtr, fee: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_builder = tx_builder.rptr(&env)?;
    let fee = fee.rptr(&env)?;
    tx_builder
      .typed_ref::<TransactionBuilder>()
      .zip(fee.typed_ref::<Fee>())
      .and_then(|(tx_builder, fee)| tx_builder.get_balance(fee).into_result())
      .and_then(|balance| RPtr::new(balance).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSealWithOutputPolicy(
  env: JNIEnv, _: JObject, tx_builder: JRPtr, fee: JRPtr, output_policy: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let fee = fee.rptr(&env)?;
    tx_builder
      .owned::<TransactionBuilder>(&env)
      .zip(fee.typed_ref::<Fee>())
      .zip(output_policy.owned::<OutputPolicy>(&env))
      .and_then(|((tx_builder, fee), output_policy)| {
        tx_builder.seal_with_output_policy(fee, output_policy).into_result()
      })
      .and_then(|transaction| RPtr::new(transaction).jptr(&env))
  })
  .jresult(&env)
}
