use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use js_chain_libs::{Address, AddressDiscrimination as RAddressDiscrimination, PublicKey};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AddressDiscrimination {
  Production = 0,
  Test = 1
}

impl From<AddressDiscrimination> for RAddressDiscrimination {
  fn from(ad: AddressDiscrimination) -> Self {
    match ad {
      AddressDiscrimination::Production => RAddressDiscrimination::Production,
      AddressDiscrimination::Test => RAddressDiscrimination::Test
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn address_from_string(
  string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    Address::from_string(string.into_str()).map(|addr| RPtr::new(addr)).into_result()
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_string(
  ptr: RPtr, prefix: CharPtr, result: &mut CharPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    ptr.typed_ref::<Address>().map(|addr| addr.to_string(prefix.into_str()).into_cstr())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_single_from_public_key(
  key: &mut RPtr, discrimination: AddressDiscrimination, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key
      .owned::<PublicKey>()
      .map(|key| Address::single_from_public_key(*key, discrimination.into()))
      .map(|addr| RPtr::new(addr))
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_delegation_from_public_key(
  key: &mut RPtr, delegation: &mut RPtr, discrimination: AddressDiscrimination, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    let delegation = delegation.owned::<PublicKey>()?;
    key.owned::<PublicKey>().map(|key| {
      Address::delegation_from_public_key(*key, *delegation, discrimination.into())
    })
    .map(|address| RPtr::new(address))
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_account_from_public_key(
  key: &mut RPtr, discrimination: AddressDiscrimination, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key
      .owned::<PublicKey>()
      .map(|key| Address::account_from_public_key(*key, discrimination.into()))
      .map(|address| RPtr::new(address))
  })
  .response(result, error)
}
