use js_chain_libs::{AddressDiscrimination as RAddressDiscrimination, Address, PublicKey};
use super::string::{CharPtr, IntoStr, IntoCString};
use crate::ptr::RPtr;
use crate::panic::{handle_exception_result, ToResult};
use super::result::CResult;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AddressDiscrimination {
  Production = 0,
  Test = 1,
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
    Address::from_string(string.into_str()).into_result()
  })
  .map(|addr| RPtr::new(addr))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_string(
  ptr: RPtr, prefix: CharPtr, result: &mut CharPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    ptr.typed_ref::<Address>()
      .map(|addr| addr.to_string(prefix.into_str()).into_cstr())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_single_from_public_key(
  pub_key: RPtr, discrimination: AddressDiscrimination,
  result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    pub_key.typed_ref::<PublicKey>()
      .map(|pkey| {
        Address::single_from_public_key(pkey.clone(), discrimination.into())
      })
  })
  .map(|addr| RPtr::new(addr))
  .response(result, error)
}
