use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use super::data::DataPtr;
use crate::js_chain_libs::PublicKey;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;

#[no_mangle]
pub unsafe extern "C" fn public_key_as_bytes(
  pub_key: RPtr, result: &mut DataPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    pub_key.typed_ref::<PublicKey>()
      .map(|pkey| pkey.as_bytes().into())
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn public_key_from_bech32(
  bech32_str: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    PublicKey::from_bech32(bech32_str.into_str()).into_result()
  })
  .map(|pkey| RPtr::new(pkey))
  .response(result, error)
}