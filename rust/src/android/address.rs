use super::primitive::ToPrimitiveObject;
use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::*;
use crate::address::{AddressDiscrimination, AddressKind};
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jint, jobject};
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
pub extern "C" fn Java_io_emurgo_chainlibs_Native_AddressKind(env: JNIEnv, _: JObject) -> jobject {
  static PUT: &str = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;";
  let class = env.find_class("java/util/HashMap").unwrap();
  let map = env.new_object(class, "()V", &[]).unwrap();

  let put = |key_name: &str, variant: AddressKind| {
    let key = *key_name.jstring(&env).unwrap();
    let int = (variant as jint).jobject(&env).unwrap();
    env.call_method(map, "put", PUT, &[key.into(), int.into()]).unwrap();
  };

  put("Single", AddressKind::Single);
  put("Group", AddressKind::Group);
  put("Account", AddressKind::Account);
  put("Multisig", AddressKind::Multisig);

  map.into_inner()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressFromBytes(
  env: JNIEnv, _: JObject, bytes: jbyteArray
) -> jobject {
  handle_exception_result(|| {
    env
      .convert_byte_array(bytes)
      .into_result()
      .and_then(|bytes| Address::from_bytes((&bytes[..]).into()).into_result())
      .and_then(|address| address.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressAsBytes(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.as_bytes())
      .and_then(|bytes| env.byte_array_from_slice(&bytes).into_result())
      .map(|arr| JObject::from(arr))
  })
  .jresult(&env)
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
      .and_then(|address| address.rptr().jptr(&env))
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
      .and_then(|address| address.rptr().jptr(&env))
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
      .and_then(|address| address.rptr().jptr(&env))
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
      .and_then(|address| address.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressGetDiscrimination(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.get_discrimination().into())
      .and_then(|discrimination: AddressDiscrimination| (discrimination as jint).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressGetKind(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.get_kind().into())
      .and_then(|address_kind: AddressKind| (address_kind as jint).jobject(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressToSingleAddress(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.to_single_address())
      .and_then(|single_address| single_address.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressToGroupAddress(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.to_group_address())
      .and_then(|group_address| group_address.rptr().jptr(&env))
  })
  .jresult(&env)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_addressToAccountAddress(
  env: JNIEnv, _: JObject, address: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let address = address.rptr(&env)?;
    address
      .typed_ref::<Address>()
      .map(|address| address.to_account_address())
      .and_then(|account_address| account_address.rptr().jptr(&env))
  })
  .jresult(&env)
}
