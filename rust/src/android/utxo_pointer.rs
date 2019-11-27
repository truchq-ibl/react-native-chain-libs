use super::ptr_j::*;
use super::result::ToJniResult;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use jni::objects::JObject;
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use js_chain_libs::{FragmentId, UtxoPointer, Value};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_chainlibs_Native_utxoPointerNew(
  env: JNIEnv, _: JObject, fragment_id: JRPtr, output_index: jint, value: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let fragment_id = fragment_id.rptr(&env)?;
    let value = value.rptr(&env)?;
    fragment_id.typed_ref::<FragmentId>().zip(value.typed_ref::<Value>()).and_then(
      |(fragment_id, value)| {
        RPtr::new(UtxoPointer::new(fragment_id, output_index as u8, value)).jptr(&env)
      }
    )
  })
  .jresult(&env)
}
