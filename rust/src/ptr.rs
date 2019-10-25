use crate::panic::{Result, ToResult};
use std::any::Any;
use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RPtr(*mut c_void);

impl From<RPtr> for usize {
  fn from(ptr: RPtr) -> Self {
    ptr.0 as usize
  }
}

impl From<usize> for RPtr {
  fn from(ptr: usize) -> Self {
    Self(ptr as *mut c_void)
  }
}

impl RPtr {
  pub fn new<T: Sized + 'static>(val: T) -> Self {
    let b: Box<Box<dyn Any>> = Box::new(Box::new(val));
    Self(Box::into_raw(b) as *mut c_void)
  }

  pub unsafe fn typed_ref<T: Sized + 'static>(&self) -> Result<&T> {
    if self.0.is_null() {
      return Err(String::from("Pointer is NULL"));
    }
    (self.0 as *mut Box<dyn Any>)
      .as_ref()
      .and_then(|any| any.downcast_ref::<T>())
      .ok_or_else(|| format!("Bad pointer: 0x{:x}", self.0 as usize))
  }

  pub unsafe fn owned<T: Sized + 'static>(mut self) -> Result<Box<T>> {
    if self.0.is_null() {
      return Err(String::from("Pointer is NULL"));
    }
    let boxed = *Box::from_raw(self.0 as *mut Box<dyn Any>);
    self.0 = std::ptr::null_mut();
    boxed.downcast::<T>().into_result()
  }

  pub unsafe fn free(&mut self) {
    if self.0.is_null() {
      return;
    }
    let _ = Box::from_raw(self.0 as *mut Box<dyn Any>);
    self.0 = std::ptr::null_mut();
  }
}
