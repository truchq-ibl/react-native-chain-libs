use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{
  Inputs, Outputs, PayloadAuthData, TransactionBuilder, TransactionBuilderSetAuthData,
  TransactionBuilderSetIOs, TransactionBuilderSetWitness, Witnesses
};

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_new(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| TransactionBuilder::new())
    .map(|tx_builder| RPtr::new(tx_builder))
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
  .map(|tx_builder_set_ios| RPtr::new(tx_builder_set_ios))
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
  .map(|tx_builder_set_witness| RPtr::new(tx_builder_set_witness))
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
  .map(|tx_sign_data_hash| RPtr::new(tx_sign_data_hash))
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
  .map(|tx_builder_set_auth_data| RPtr::new(tx_builder_set_auth_data))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_set_auth_data_set_payload_auth(
  tx_builder_set_auth_data: &mut RPtr, auth: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder_set_auth_data
      .owned::<TransactionBuilderSetAuthData>()
      .zip(auth.owned::<PayloadAuthData>())
      .and_then(|(tx_builder_set_auth_data, auth)| {
        tx_builder_set_auth_data.set_payload_auth(auth).into_result()
      })
  })
  .map(|transaction| RPtr::new(transaction))
  .response(result, error)
}
