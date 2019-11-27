use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::*;
use crate::address::AddressDiscrimination;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtr;
use jni::objects::{JObject, JString};
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use js_chain_libs::{Address, PublicKey};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_AddressDiscrimination(
  env: JNIEnv, _: JObject
) -> jobject {
  static PUT: &str = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;";
  let class = env.find_class("java/util/HashMap").unwrap();
  let map = env.new_object(class, "()V", &[]).unwrap();

  let prod_key = *"Production".jstring(&env).unwrap();
  let prod_int = (AddressDiscrimination::Production as jint).jobject(&env).unwrap();
  env.call_method(map, "put", PUT, &[prod_key.into(), prod_int.into()]).unwrap();

  let test_key = *"Test".jstring(&env).unwrap();
  let test_int = (AddressDiscrimination::Test as jint).jobject(&env).unwrap();
  env.call_method(map, "put", PUT, &[test_key.into(), test_int.into()]).unwrap();

  map.into_inner()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Java_io_emurgo_chainlibs_Native_addressFromString(
  env: JNIEnv, _: JObject, string: JString
) -> jobject {
  handle_exception_result(|| {
    string
      .string(&env)
      .and_then(|rstr| Address::from_string(&rstr).into_result())
      .and_then(|address| RPtr::new(address).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressToString(
  env: JNIEnv, _: JObject, ptr: JRPtr, prefix: JString
) -> jobject {
  handle_exception_result(|| {
    let rptr = ptr.rptr(&env)?;
    rptr
      .typed_ref::<Address>()
      .zip(prefix.string(&env))
      .and_then(|(address, prefix)| address.to_string(&prefix).jstring(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressSingleFromPublicKey(
  env: JNIEnv, _: JObject, key_ptr: JRPtr, discrimination: jint
) -> jobject {
  handle_exception_result(|| {
    let key_ptr = key_ptr.rptr(&env)?;
    key_ptr
      .typed_ref::<PublicKey>()
      .map(|key| {
        Address::single_from_public_key(key, AddressDiscrimination::from(discrimination).into())
      })
      .and_then(|address| RPtr::new(address).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressDelegationFromPublicKey(
  env: JNIEnv, _: JObject, key_ptr: JRPtr, delegation_ptr: JRPtr, discrimination: jint
) -> jobject {
  handle_exception_result(|| {
    let key_ptr = key_ptr.rptr(&env)?;
    let delegation_ptr = delegation_ptr.rptr(&env)?;
    key_ptr
      .typed_ref::<PublicKey>()
      .zip(delegation_ptr.typed_ref::<PublicKey>())
      .map(|(key, delegation)| {
        Address::delegation_from_public_key(
          key,
          delegation,
          AddressDiscrimination::from(discrimination).into()
        )
      })
      .and_then(|address| RPtr::new(address).jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressAccountFromPublicKey(
  env: JNIEnv, _: JObject, key_ptr: JRPtr, discrimination: jint
) -> jobject {
  handle_exception_result(|| {
    let key_ptr = key_ptr.rptr(&env)?;
    key_ptr
      .typed_ref::<PublicKey>()
      .map(|key| {
        Address::account_from_public_key(key, AddressDiscrimination::from(discrimination).into())
      })
      .and_then(|address| RPtr::new(address).jptr(&env))
  })
  .jresult(&env)
}
