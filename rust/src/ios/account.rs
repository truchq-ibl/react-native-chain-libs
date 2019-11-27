use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use js_chain_libs::{Account, Address, PublicKey};

#[no_mangle]
pub unsafe extern "C" fn account_from_address(
  address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().and_then(|address| Account::from_address(address).into_result())
  })
  .map(|account| RPtr::new(account))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn account_single_from_public_key(
  key: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key.typed_ref::<PublicKey>().map(|key| Account::single_from_public_key(key))
  })
  .map(|account| RPtr::new(account))
  .response(result, error)
}
