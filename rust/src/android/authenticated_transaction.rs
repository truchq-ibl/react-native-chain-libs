use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::AuthenticatedTransaction;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_authenticatedTransactionTransaction(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<AuthenticatedTransaction>()
      .map(|auth_tx| RPtr::new(auth_tx.transaction()))
      .and_then(|rptr| rptr.jptr(&env))
  })
  .jresult(&env)
}
