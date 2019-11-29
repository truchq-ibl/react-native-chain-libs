use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{
  Certificate, Inputs, Outputs, PayloadAuthData, TransactionBuilder, TransactionBuilderSetAuthData,
  TransactionBuilderSetIOs, TransactionBuilderSetWitness, Witnesses
};

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| TransactionBuilder::new())
    .map(|tx_builder| tx_builder.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_payload(
  tx_builder: &mut RPtr, cert: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder
      .owned::<TransactionBuilder>()
      .zip(cert.typed_ref::<Certificate>())
      .map(|(tx_builder, cert)| tx_builder.payload(cert))
  })
  .map(|tx_builder_set_ios| tx_builder_set_ios.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_no_payload(
  tx_builder: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder
      .owned::<TransactionBuilder>()
      .map(|tx_builder| TransactionBuilder::no_payload(tx_builder))
  })
  .map(|tx_builder_set_ios| tx_builder_set_ios.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_ios_set_ios(
  tx_builder_set_ios: &mut RPtr, inputs: RPtr, outputs: RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_ios
      .owned::<TransactionBuilderSetIOs>()
      .zip(inputs.typed_ref::<Inputs>())
      .zip(outputs.typed_ref::<Outputs>())
      .map(|((tx_builder_set_ios, inputs), outputs)| tx_builder_set_ios.set_ios(inputs, outputs))
  })
  .map(|tx_builder_set_witness| tx_builder_set_witness.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_witness_get_auth_data_for_witness(
  tx_builder_set_witness: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_witness
      .typed_ref::<TransactionBuilderSetWitness>()
      .map(|tx_builder_set_witness| tx_builder_set_witness.get_auth_data_for_witness())
  })
  .map(|tx_sign_data_hash| tx_sign_data_hash.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_witness_set_witnesses(
  tx_builder_set_witness: &mut RPtr, witnesses: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_witness
      .owned::<TransactionBuilderSetWitness>()
      .zip(witnesses.typed_ref::<Witnesses>())
      .map(|(tx_builder_set_witness, witnesses)| tx_builder_set_witness.set_witnesses(witnesses))
  })
  .map(|tx_builder_set_auth_data| tx_builder_set_auth_data.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_auth_data_get_auth_data(
  tx_builder_set_auth_data: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_auth_data
      .typed_ref::<TransactionBuilderSetAuthData>()
      .map(|tx_builder_set_auth_data| tx_builder_set_auth_data.get_auth_data())
  })
  .map(|transaction_binding_auth_data| transaction_binding_auth_data.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_auth_data_set_payload_auth(
  tx_builder_set_auth_data: &mut RPtr, auth: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_auth_data
      .owned::<TransactionBuilderSetAuthData>()
      .zip(auth.typed_ref::<PayloadAuthData>())
      .and_then(|(tx_builder_set_auth_data, auth)| {
        tx_builder_set_auth_data.set_payload_auth(auth).into_result()
      })
  })
  .map(|transaction| transaction.rptr())
  .response(result, error)
}
