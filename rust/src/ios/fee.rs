use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{Fee, Transaction, Value};

#[no_mangle]
pub unsafe extern "C" fn fee_linear_fee(
  constant: &mut RPtr, coefficient: &mut RPtr, certificate: &mut RPtr, result: &mut RPtr,
  error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    constant
      .owned::<Value>()
      .zip(coefficient.owned::<Value>())
      .zip(certificate.owned::<Value>())
      .map(|((constant, coefficient), certificate)| {
        Fee::linear_fee(constant, coefficient, certificate)
      })
  })
  .map(|fee| RPtr::new(fee))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn fee_calculate(
  fee: RPtr, tx: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    fee
      .typed_ref::<Fee>()
      .zip(tx.owned::<Transaction>())
      .and_then(|(fee, tx)| fee.calculate(tx).ok_or(String::from("Cannot calculate fee")))
  })
  .map(|value| RPtr::new(value))
  .response(result, error)
}
