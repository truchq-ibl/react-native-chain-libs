use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{AccountBindingSignature, StakeDelegationAuthData};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationAuthDataNew(
  env: JNIEnv, _: JObject, signature: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let signature = signature.rptr(&env)?;
    signature
      .typed_ref::<AccountBindingSignature>()
      .map(|signature| StakeDelegationAuthData::new(signature))
      .and_then(|stake_delegation_auth_data| RPtr::new(stake_delegation_auth_data).jptr(&env))
  })
  .jresult(&env)
}
