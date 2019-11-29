use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::TransactionSignDataHash;

#[no_mangle]
pub unsafe extern "C" fn transaction_sign_data_hash_from_bytes(
  data: *const u8, len: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    TransactionSignDataHash::from_bytes(std::slice::from_raw_parts(data, len)).into_result()
  })
  .map(|tx_sign_data_hash| tx_sign_data_hash.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_sign_data_hash_from_hex(
  input: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| TransactionSignDataHash::from_hex(input.into_str()).into_result())
    .map(|tx_sign_data_hash| tx_sign_data_hash.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn transaction_sign_data_hash_as_bytes(
  tx_sign_data_hash: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    tx_sign_data_hash
      .typed_ref::<TransactionSignDataHash>()
      .map(|tx_sign_data_hash| tx_sign_data_hash.as_bytes())
  })
  .map(|bytes| bytes.into())
  .response(result, error)
}
