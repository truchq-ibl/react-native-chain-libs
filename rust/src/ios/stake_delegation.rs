use super::data::DataPtr;
use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{DelegationType, PublicKey, StakeDelegation};

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_new(
  delegation_type: RPtr, account: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    delegation_type
      .typed_ref::<DelegationType>()
      .zip(account.typed_ref::<PublicKey>())
      .map(|(delegation_type, account)| StakeDelegation::new(delegation_type, account))
  })
  .map(|stake_delegation| stake_delegation.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_delegation_type(
  stake_delegation: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.delegation_type())
  })
  .map(|delegation_type| delegation_type.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_account(
  stake_delegation: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.account())
  })
  .map(|account_identifier| account_identifier.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_as_bytes(
  stake_delegation: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.as_bytes())
  })
  .map(|bytes| bytes.into())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_from_bytes(
  data: *const u8, len: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    StakeDelegation::from_bytes(std::slice::from_raw_parts(data, len)).into_result()
  })
  .map(|stake_delegation| stake_delegation.rptr())
  .response(result, error)
}
