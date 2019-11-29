use super::data::DataPtr;
use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::FragmentId;

#[no_mangle]
pub unsafe extern "C" fn fragment_id_calculate(
  data: *const u8, len: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception(|| FragmentId::calculate(std::slice::from_raw_parts(data, len)))
    .map(|fragment_id| fragment_id.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_id_as_bytes(
  fragment_id: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment_id.typed_ref::<FragmentId>().map(|fragment_id| fragment_id.as_bytes())
  })
  .map(|bytes| bytes.into())
  .response(result, error)
}
