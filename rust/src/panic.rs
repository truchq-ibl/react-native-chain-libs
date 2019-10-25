use std::panic;

pub type Result<T> = std::result::Result<T, String>;

pub trait ToResult<T> {
  fn into_result(self) -> Result<T>;
}

pub fn handle_exception<F: FnOnce() -> R + panic::UnwindSafe, R>(func: F) -> Result<R> {
  handle_exception_result(|| Ok(func()))
}

pub fn handle_exception_result<F: FnOnce() -> Result<R> + panic::UnwindSafe, R>(
  func: F
) -> Result<R> {
  panic::catch_unwind(func)
    .map_err(|err| {
      if let Some(string) = err.downcast_ref::<String>() {
        string.clone()
      } else if let Some(string) = err.downcast_ref::<&'static str>() {
        String::from(*string)
      } else {
        format!("Reason: {:?}", err)
      }
    })
    .and_then(|res| res)
}

pub fn hide_exceptions() {
  panic::set_hook(Box::new(|_| {}));
}
