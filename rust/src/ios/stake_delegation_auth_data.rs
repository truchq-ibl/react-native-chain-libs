use super::result::CResult;
use super::string::CharPtr;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use js_chain_libs::{AccountBindingSignature, StakeDelegationAuthData};

#[no_mangle]
pub unsafe extern "C" fn stake_delegation_auth_data_new(
  signature: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    signature
      .typed_ref::<AccountBindingSignature>()
      .map(|signature| StakeDelegationAuthData::new(signature))
  })
  .map(|stake_delegation_auth_data| RPtr::new(stake_delegation_auth_data))
  .response(result, error)
}
