use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{AccountBindingSignature, PrivateKey, TransactionBindingAuthData};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_accountBindingSignatureNewSingle(
  env: JNIEnv, _: JObject, private_key: JRPtr, auth_data: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let private_key = private_key.rptr(&env)?;
    let auth_data = auth_data.rptr(&env)?;
    private_key
      .typed_ref::<PrivateKey>()
      .zip(auth_data.typed_ref::<TransactionBindingAuthData>())
      .map(|(private_key, auth_data)| AccountBindingSignature::new_single(private_key, auth_data))
      .and_then(|account_binding_signature| account_binding_signature.rptr().jptr(&env))
  })
  .jresult(&env)
}
