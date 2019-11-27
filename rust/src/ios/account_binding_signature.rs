use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{AccountBindingSignature, PrivateKey, TransactionBindingAuthData};

#[no_mangle]
pub unsafe extern "C" fn account_binding_signature_new_single(
  private_key: RPtr, auth_data: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    private_key
      .typed_ref::<PrivateKey>()
      .zip(auth_data.typed_ref::<TransactionBindingAuthData>())
      .map(|(private_key, auth_data)| AccountBindingSignature::new_single(private_key, auth_data))
  })
  .map(|account_binding_signature| RPtr::new(account_binding_signature))
  .response(result, error)
}
