use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{
  Certificate, Inputs, Outputs, PayloadAuthData, TransactionBuilder, TransactionBuilderSetAuthData,
  TransactionBuilderSetIOs, TransactionBuilderSetWitness, Witnesses
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderNew(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| TransactionBuilder::new().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderPayload(
  env: JNIEnv, _: JObject, tx_builder: JRPtr, cert: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let cert = cert.rptr(&env)?;
    tx_builder
      .owned::<TransactionBuilder>(&env)
      .zip(cert.typed_ref::<Certificate>())
      .map(|(tx_builder, cert)| tx_builder.payload(cert))
      .and_then(|tx_builder_set_ios| tx_builder_set_ios.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderNoPayload(
  env: JNIEnv, _: JObject, tx_builder: JRPtr
) -> jobject {
  handle_exception_result(|| {
    tx_builder
      .owned::<TransactionBuilder>(&env)
      .and_then(|tx_builder| TransactionBuilder::no_payload(tx_builder).rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSetIOsSetIOs(
  env: JNIEnv, _: JObject, tx_builder_set_ios: JRPtr, inputs: JRPtr, outputs: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let inputs = inputs.rptr(&env)?;
    let outputs = outputs.rptr(&env)?;
    tx_builder_set_ios
      .owned::<TransactionBuilderSetIOs>(&env)
      .zip(inputs.typed_ref::<Inputs>())
      .zip(outputs.typed_ref::<Outputs>())
      .map(|((tx_builder_set_ios, inputs), outputs)| tx_builder_set_ios.set_ios(inputs, outputs))
      .and_then(|tx_builder_set_witness| tx_builder_set_witness.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSetWitnessGetAuthDataForWitness(
  env: JNIEnv, _: JObject, tx_builder_set_witness: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_builder_set_witness = tx_builder_set_witness.rptr(&env)?;
    tx_builder_set_witness
      .typed_ref::<TransactionBuilderSetWitness>()
      .map(|tx_builder_set_witness| tx_builder_set_witness.get_auth_data_for_witness())
      .and_then(|tx_sign_data_hash| tx_sign_data_hash.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSetWitnessSetWitnesses(
  env: JNIEnv, _: JObject, tx_builder_set_witness: JRPtr, witnesses: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let witnesses = witnesses.rptr(&env)?;
    tx_builder_set_witness
      .owned::<TransactionBuilderSetWitness>(&env)
      .zip(witnesses.typed_ref::<Witnesses>())
      .map(|(tx_builder_set_witness, witnesses)| tx_builder_set_witness.set_witnesses(witnesses))
      .and_then(|tx_builder_set_auth_data| tx_builder_set_auth_data.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSetAuthDataGetAuthData(
  env: JNIEnv, _: JObject, tx_builder_set_auth_data: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let tx_builder_set_auth_data = tx_builder_set_auth_data.rptr(&env)?;
    tx_builder_set_auth_data
      .typed_ref::<TransactionBuilderSetAuthData>()
      .map(|tx_builder_set_auth_data| tx_builder_set_auth_data.get_auth_data())
      .and_then(|transaction_binding_auth_data| transaction_binding_auth_data.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_transactionBuilderSetAuthDataSetPayloadAuth(
  env: JNIEnv, _: JObject, tx_builder_set_auth_data: JRPtr, auth: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let auth = auth.rptr(&env)?;
    tx_builder_set_auth_data
      .owned::<TransactionBuilderSetAuthData>(&env)
      .zip(auth.typed_ref::<PayloadAuthData>())
      .and_then(|(tx_builder_set_auth_data, auth)| {
        tx_builder_set_auth_data.set_payload_auth(auth).into_result()
      })
      .and_then(|transaction| transaction.rptr().jptr(&env))
  })
  .jresult(&env)
}
