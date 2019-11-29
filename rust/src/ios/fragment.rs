use super::data::DataPtr;
use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Fragment, Transaction};

#[no_mangle]
pub unsafe extern "C" fn fragment_from_transaction(
  tx: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| tx.typed_ref::<Transaction>().map(|tx| Fragment::from_transaction(tx)))
    .map(|fragment| fragment.rptr())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_get_transaction(
  fragment: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().and_then(|fragment| fragment.get_transaction().into_result())
  })
  .map(|tx| tx.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_as_bytes(
  fragment: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().and_then(|fragment| fragment.as_bytes().into_result())
  })
  .map(|bytes| bytes.into())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_initial(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_initial()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_transaction(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_transaction())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_owner_stake_delegation(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_owner_stake_delegation())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_stake_delegation(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_stake_delegation())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_pool_registration(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_pool_registration())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_pool_retirement(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_pool_retirement())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_pool_update(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_pool_update())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_old_utxo_declaration(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_old_utxo_declaration())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_update_proposal(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_update_proposal())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_is_update_vote(
  fragment: RPtr, result: &mut bool, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fragment.typed_ref::<Fragment>().map(|fragment| fragment.is_update_vote())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fragment_id(
  fragment: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| fragment.typed_ref::<Fragment>().map(|fragment| fragment.id()))
    .map(|fragment_id| fragment_id.rptr())
    .response(result, error)
}
