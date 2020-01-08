use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use crate::js_chain_libs::PoolId;

#[no_mangle]
pub unsafe extern "C" fn pool_id_from_hex(
  input: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| PoolId::from_hex(input.into_str()).into_result())
    .map(|tx_sign_data_hash| tx_sign_data_hash.rptr())
    .response(result, error)
}