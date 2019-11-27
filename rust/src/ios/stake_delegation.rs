use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
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
  .map(|stake_delegation| RPtr::new(stake_delegation))
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
  .map(|delegation_type| RPtr::new(delegation_type))
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
  .map(|account_identifier| RPtr::new(account_identifier))
  .response(result, error)
}
