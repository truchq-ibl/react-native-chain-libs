use super::data::DataPtr;
use super::result::CResult;
use super::string::{CharPtr, IntoCString, IntoStr};
use crate::address::{AddressDiscrimination, AddressKind};
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::{RPtr, RPtrRepresentable};
use js_chain_libs::{Address, PublicKey};

#[no_mangle]
pub unsafe extern "C" fn address_as_bytes(
  address: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| address.typed_ref::<Address>().map(|address| address.as_bytes()))
    .map(|bytes| bytes.into())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_from_string(
  string: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    Address::from_string(string.into_str()).map(|addr| addr.rptr()).into_result()
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
  key: RPtr, discrimination: AddressDiscrimination, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key
      .typed_ref::<PublicKey>()
      .map(|key| Address::single_from_public_key(key, discrimination.into()))
      .map(|addr| addr.rptr())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_delegation_from_public_key(
  key: RPtr, delegation: RPtr, discrimination: AddressDiscrimination, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key
      .typed_ref::<PublicKey>()
      .zip(delegation.typed_ref::<PublicKey>())
      .map(|(key, delegation)| {
        Address::delegation_from_public_key(key, delegation, discrimination.into())
      })
      .map(|address| address.rptr())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_account_from_public_key(
  key: RPtr, discrimination: AddressDiscrimination, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    key
      .typed_ref::<PublicKey>()
      .map(|key| Address::account_from_public_key(key, discrimination.into()))
      .map(|address| address.rptr())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_get_discrimination(
  address: RPtr, result: &mut AddressDiscrimination, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().map(|address| address.get_discrimination())
  })
  .map(|discrimination| discrimination.into())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_get_kind(
  address: RPtr, result: &mut AddressKind, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| address.typed_ref::<Address>().map(|address| address.get_kind()))
    .map(|address_kind| address_kind.into())
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_single_address(
  address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().map(|address| address.to_single_address())
  })
  .map(|single_address| single_address.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_group_address(
  address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().map(|address| address.to_group_address())
  })
  .map(|group_address| group_address.rptr())
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn address_to_account_address(
  address: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    address.typed_ref::<Address>().map(|address| address.to_account_address())
  })
  .map(|account_address| account_address.rptr())
  .response(result, error)
}
