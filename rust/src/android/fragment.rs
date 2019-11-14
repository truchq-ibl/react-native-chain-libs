use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jboolean, jobject};
use jni::JNIEnv;
use js_chain_libs::{Fragment, Transaction};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentFromTransaction(
  env: JNIEnv, _: JObject, tx_ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    tx_ptr
      .owned::<Transaction>(&env)
      .map(|tx| Fragment::from_transaction(tx))
      .and_then(|fragment| RPtr::new(fragment).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentGetTransaction(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    ptr
      .owned::<Fragment>(&env)
      .and_then(|fragment| fragment.get_transaction().into_result())
      .and_then(|tx| RPtr::new(tx).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentAsBytes(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| fragment.as_bytes().into_result())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsInitial(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_initial() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsTransaction(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_transaction() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsOwnerStakeDelegation(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_owner_stake_delegation() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsStakeDelegation(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_stake_delegation() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsPoolRegistration(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_pool_registration() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsPoolRetirement(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_pool_retirement() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsPoolUpdate(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_pool_update() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsOldUtxoDeclaration(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_old_utxo_declaration() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsUpdateProposal(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_update_proposal() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentIsUpdateVote(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .and_then(|fragment| (fragment.is_update_vote() as jboolean).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_fragmentId(
  env: JNIEnv, _: JObject, ptr: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Fragment>()
      .map(|fragment| fragment.id())
      .and_then(|id| RPtr::new(id).jptr(&env))
  })
  .jresult(&env)
}
