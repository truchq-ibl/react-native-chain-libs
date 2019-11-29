use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::handle_exception_result;
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use js_chain_libs::{DelegationRatio, DelegationType, PoolId};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_delegationTypeNonDelegated(
  env: JNIEnv, _: JObject
) -> jobject {
  handle_exception_result(|| DelegationType::non_delegated().rptr().jptr(&env)).jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_delegationTypeFull(
  env: JNIEnv, _: JObject, pool_id: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let pool_id = pool_id.rptr(&env)?;
    pool_id
      .typed_ref::<PoolId>()
      .map(|pool_id| DelegationType::full(pool_id))
      .and_then(|delegation_type| delegation_type.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_delegationTypeRatio(
  env: JNIEnv, _: JObject, r: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let r = r.rptr(&env)?;
    r.typed_ref::<DelegationRatio>()
      .map(|r| DelegationType::ratio(r))
      .and_then(|delegation_type| delegation_type.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_delegationTypeGetKind(
  env: JNIEnv, _: JObject, delegation_type: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let delegation_type = delegation_type.rptr(&env)?;
    delegation_type
      .typed_ref::<DelegationType>()
      .map(|delegation_type| delegation_type.get_kind())
      .and_then(|delegation_kind| delegation_kind.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_delegationTypeGetFull(
  env: JNIEnv, _: JObject, delegation_type: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let delegation_type = delegation_type.rptr(&env)?;
    delegation_type
      .typed_ref::<DelegationType>()
      .map(|delegation_type| delegation_type.get_full())
      .and_then(|pool_id| pool_id.rptr().jptr(&env))
  })
  .jresult(&env)
}
