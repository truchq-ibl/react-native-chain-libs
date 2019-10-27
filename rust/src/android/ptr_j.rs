use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jlong;

#[cfg(target_pointer_width = "64")]
use std::mem;

use crate::ptr::RPtr;
use crate::panic::{Result, ToResult};
use super::string::ToString;

pub trait ToJniPtr {
  fn jptr<'a>(self, env: &'a JNIEnv) -> Result<JObject<'a>>;
}

pub trait FromJniPtr {
  fn rptr<'a>(self, env: &'a JNIEnv) -> Result<RPtr>;
}

impl<'a> FromJniPtr for JObject<'a> {
  fn rptr(self, env: &JNIEnv) -> Result<RPtr> {
    let class_obj = env
      .call_method(self, "getClass", "()Ljava/lang/Class;", &[])
      .and_then(|res| res.l())
      .into_result()?;
    let name = env
      .call_method(class_obj, "getSimpleName", "()Ljava/lang/String;", &[])
      .and_then(|res| res.l())
      .into_result()
      .and_then(|obj| JString::from(obj).string(env))?;
    if name != "RPtr" {
      return Err(format!("Wrong class: {}, expected RPtr", name));
    }
    env
      .get_field(self, "ptr", "J")
      .and_then(|res| res.j())
      .map(|iptr| {
        #[cfg(target_pointer_width = "32")]
        {
          (iptr as usize).into()
        }
        #[cfg(target_pointer_width = "64")]
        unsafe {
          mem::transmute::<jlong, usize>(iptr).into()
        }
      })
      .into_result()
  }
}

impl ToJniPtr for RPtr {
  fn jptr<'a>(self, env: &'a JNIEnv) -> Result<JObject<'a>> {
    let ptr = {
      #[cfg(target_pointer_width = "32")]
      {
        usize::from(self) as jlong
      }
      #[cfg(target_pointer_width = "64")]
      unsafe {
        mem::transmute::<usize, jlong>(self.into())
      }
    };
    env.find_class("io/emurgo/chainlibs/RPtr")
      .and_then(|class| env.new_object(class, "(J)V", &[ptr.into()]))
      .into_result()
  }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn Java_io_emurgo_chainlibs_Native_ptrFree(
  env: JNIEnv, _: JObject, ptr: JObject
) {
  ptr.rptr(&env).unwrap().free();
}
