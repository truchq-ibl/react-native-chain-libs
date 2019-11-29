use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::Payload;

#[no_mangle]
pub extern "C" fn payload_no_payload(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| Payload::no_payload().rptr()).response(result, error)
}
