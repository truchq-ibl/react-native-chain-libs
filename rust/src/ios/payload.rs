use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception;
use crate::ptr::RPtr;
use js_chain_libs::Payload;

#[no_mangle]
pub extern "C" fn payload_no_payload(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| RPtr::new(Payload::no_payload())).response(result, error)
}
