use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Account, Address, PublicKey};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_accountFromAddress(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .and_then(|address| Account::from_address(address).into_result())
      .and_then(|account| RPtr::new(account).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_accountSingleFromPublicKey(
  env: JNIEnv, _: JObject, key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let key = key.rptr(&env)?;
    key
      .typed_ref::<PublicKey>()
      .map(|key| Account::single_from_public_key(key))
      .and_then(|account| RPtr::new(account).jptr(&env))
  })
  .jresult(&env)
}
