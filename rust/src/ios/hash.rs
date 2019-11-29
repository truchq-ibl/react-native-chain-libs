use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::Hash;

#[no_mangle]
pub extern "C" fn hash_from_hex(
  hex_string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| Hash::from_hex(hex_string.into_str()).into_result())
    .map(|hash| hash.rptr())
    .response(result, error)
}
