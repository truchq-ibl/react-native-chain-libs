use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use js_chain_libs::{DelegationType, PublicKey, StakeDelegation};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationNew(
  env: JNIEnv, _: JObject, delegation_type: JRPtr, account: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let delegation_type = delegation_type.rptr(&env)?;
    let account = account.rptr(&env)?;
    delegation_type
      .typed_ref::<DelegationType>()
      .zip(account.typed_ref::<PublicKey>())
      .map(|(delegation_type, account)| StakeDelegation::new(delegation_type, account))
      .and_then(|stake_delegation| stake_delegation.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationDelegationType(
  env: JNIEnv, _: JObject, stake_delegation: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let stake_delegation = stake_delegation.rptr(&env)?;
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.delegation_type())
      .and_then(|delegation_type| delegation_type.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationAccount(
  env: JNIEnv, _: JObject, stake_delegation: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let stake_delegation = stake_delegation.rptr(&env)?;
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.account())
      .and_then(|account_identifier| account_identifier.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationAsBytes(
  env: JNIEnv, _: JObject, stake_delegation: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let stake_delegation = stake_delegation.rptr(&env)?;
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| stake_delegation.as_bytes())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_stakeDelegationFromBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| StakeDelegation::from_bytes(&bytes).into_result())
      .and_then(|stake_delegation| stake_delegation.rptr().jptr(&env))
  })
  .jresult(&env)
}
