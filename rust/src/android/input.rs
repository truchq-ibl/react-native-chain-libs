use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{Account, Input, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_inputFromAccount(
  env: JNIEnv, _: JObject, account: JRPtr, v: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let account = account.rptr(&env)?;
    account
      .typed_ref::<Account>()
      .zip(v.owned::<Value>(&env))
      .map(|(account, v)| Input::from_account(account, v))
      .and_then(|input| RPtr::new(input).jptr(&env))
  })
  .jresult(&env)
}
