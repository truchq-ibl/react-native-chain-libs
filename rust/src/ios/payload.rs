use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception,handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Payload, Certificate};

#[no_mangle]
pub extern "C" fn payload_no_payload(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| Payload::no_payload().rptr()).response(result, error)
}


#[no_mangle]
pub unsafe extern "C" fn payload_certificate(certificate: RPtr, result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception_result(|| {
    certificate
      .typed_ref::<Certificate>()
      .map(|certificate| 
        Payload::certificate(certificate)
      )
  })
  .map(|certificate| certificate.rptr())
  .response(result, error)
}

