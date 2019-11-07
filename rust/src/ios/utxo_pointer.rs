use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{FragmentId, UtxoPointer, Value};

#[no_mangle]
pub unsafe extern "C" fn utxo_pointer_new(
  fragment_id: &mut RPtr, output_index: u8, value: &mut RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment_id
      .owned::<FragmentId>()
      .zip(value.owned::<Value>())
      .map(|(fragment_id, value)| UtxoPointer::new(fragment_id, output_index, value))
  })
  .map(|utxo_pointer| RPtr::new(utxo_pointer))
  .response(result, error)
}
