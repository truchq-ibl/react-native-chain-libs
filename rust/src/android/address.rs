use crate::address::AddressDiscrimination;

use jni::JNIEnv;
use jni::objects::{JObject};
use jni::sys::{jobject, jint};

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_io_emurgo_chainlibs_Native_AddressDiscrimination(env: JNIEnv, _: JObject) -> jobject {
  static PUT: &str = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;";
  let class = env.find_class("java/util/HashMap").unwrap();
  let int_class = env.find_class("java/lang/Integer").unwrap();
  let map = env.new_object(class, "()V", &[]).unwrap();

  let prod_key = *env.new_string("Production").unwrap();
  let prod_int = env.new_object(int_class, "(I)V", &[(AddressDiscrimination::Production as jint).into()]).unwrap();
  env.call_method(map, "put", PUT, &[prod_key.into(), prod_int.into()]).unwrap();

  let test_key = *env.new_string("Test").unwrap();
  let test_int = env.new_object(int_class, "(I)V", &[(AddressDiscrimination::Test as jint).into()]).unwrap();
  env.call_method(map, "put", PUT, &[test_key.into(), test_int.into()]).unwrap();

  map.into_inner()
}