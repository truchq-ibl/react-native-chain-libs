use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception, handle_exception_result};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{DelegationRatio, DelegationType, PoolId};

#[no_mangle]
pub extern "C" fn delegation_type_non_delegated(result: &mut RPtr, error: &mut CharPtr) -> bool {
  handle_exception(|| DelegationType::non_delegated().rptr()).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delegation_type_full(
  pool_id: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    pool_id.typed_ref::<PoolId>().map(|pool_id| DelegationType::full(pool_id))
  })
  .map(|delegation_type| delegation_type.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delegation_type_ratio(
  r: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| r.typed_ref::<DelegationRatio>().map(|r| DelegationType::ratio(r)))
    .map(|delegation_type| delegation_type.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delegation_type_get_kind(
  delegation_type: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    delegation_type.typed_ref::<DelegationType>().map(|delegation_type| delegation_type.get_kind())
  })
  .map(|delegation_kind| delegation_kind.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delegation_type_get_full(
  delegation_type: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    delegation_type.typed_ref::<DelegationType>().map(|delegation_type| delegation_type.get_full())
  })
  .map(|pool_id| pool_id.rptr())
  .response(result, error)
}
