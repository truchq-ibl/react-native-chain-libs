use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use crate::certificate::CertificateKind;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtrRepresentable;
use jni::objects::JObject;
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use js_chain_libs::{Certificate, PoolRegistration, PoolRetirement, StakeDelegation};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_CertificateKind(
  env: JNIEnv, _: JObject
) -> jobject {
  static PUT: &str = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;";
  let class = env.find_class("java/util/HashMap").unwrap();
  let map = env.new_object(class, "()V", &[]).unwrap();

  let put = |key_name: &str, variant: CertificateKind| {
    let key = *key_name.jstring(&env).unwrap();
    let int = (variant as jint).jobject(&env).unwrap();
    env.call_method(map, "put", PUT, &[key.into(), int.into()]).unwrap();
  };

  put("StakeDelegation", CertificateKind::StakeDelegation);
  put("OwnerStakeDelegation", CertificateKind::OwnerStakeDelegation);
  put("PoolRegistration", CertificateKind::PoolRegistration);
  put("PoolRetirement", CertificateKind::PoolRetirement);
  put("PoolUpdate", CertificateKind::PoolUpdate);

  map.into_inner()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateStakeDelegation(
  env: JNIEnv, _: JObject, stake_delegation: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let stake_delegation = stake_delegation.rptr(&env)?;
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| Certificate::stake_delegation(stake_delegation))
      .and_then(|certificate| certificate.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateStakePoolRegistration(
  env: JNIEnv, _: JObject, pool_registration: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let pool_registration = pool_registration.rptr(&env)?;
    pool_registration
      .typed_ref::<PoolRegistration>()
      .map(|pool_registration| Certificate::stake_pool_registration(pool_registration))
      .and_then(|certificate| certificate.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateStakePoolRetirement(
  env: JNIEnv, _: JObject, pool_retirement: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let pool_retirement = pool_retirement.rptr(&env)?;
    pool_retirement
      .typed_ref::<PoolRetirement>()
      .map(|pool_retirement| Certificate::stake_pool_retirement(pool_retirement))
      .and_then(|certificate| certificate.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateGetType(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .map(|certificate| certificate.get_type().into())
      .and_then(|certificate_kind: CertificateKind| (certificate_kind as jint).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateGetStakeDelegation(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_stake_delegation().into_result())
      .and_then(|stake_delegation| stake_delegation.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateGetOwnerStakeDelegation(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_owner_stake_delegation().into_result())
      .and_then(|owner_stake_delegation| owner_stake_delegation.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateGetPoolRegistration(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_pool_registration().into_result())
      .and_then(|pool_registration| pool_registration.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_certificateGetPoolRetirement(
  env: JNIEnv, _: JObject, certificate: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let certificate = certificate.rptr(&env)?;
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_pool_retirement().into_result())
      .and_then(|pool_retirement| pool_retirement.rptr().jptr(&env))
  })
  .jresult(&env)
}
