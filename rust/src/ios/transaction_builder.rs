use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{Address, Input, TransactionBuilder, Value};

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_new_no_payload(
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| RPtr::new(TransactionBuilder::new_no_payload())).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_add_input(
  tx_builder: RPtr, input: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder
      .typed_ref::<TransactionBuilder>()
      .zip(input.owned::<Input>())
      .map(|(tx_builder, input)| tx_builder.add_input(input))
  })
  .response(&mut (), error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_builder_add_output(
  tx_builder: RPtr, address: &mut RPtr, value: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_builder
      .typed_ref::<TransactionBuilder>()
      .zip(address.owned::<Address>())
      .zip(value.owned::<Value>())
      .map(|((tx_builder, address), value)| tx_builder.add_output(address, value))
  })
  .response(&mut (), error)
}
