use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use js_chain_libs::{Certificate, PoolRegistration, PoolRetirement, StakeDelegation};

#[no_mangle]
pub unsafe extern "C" fn certificate_stake_delegation(
  stake_delegation: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    stake_delegation
      .typed_ref::<StakeDelegation>()
      .map(|stake_delegation| Certificate::stake_delegation(stake_delegation))
  })
  .map(|certificate| RPtr::new(certificate))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_stake_pool_registration(
  pool_registration: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    pool_registration
      .typed_ref::<PoolRegistration>()
      .map(|pool_registration| Certificate::stake_pool_registration(pool_registration))
  })
  .map(|certificate| RPtr::new(certificate))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_stake_pool_retirement(
  pool_retirement: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    pool_retirement
      .typed_ref::<PoolRetirement>()
      .map(|pool_retirement| Certificate::stake_pool_retirement(pool_retirement))
  })
  .map(|certificate| RPtr::new(certificate))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_get_type(
  certificate: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    certificate.typed_ref::<Certificate>().map(|certificate| certificate.get_type())
  })
  .map(|certificate_type| RPtr::new(certificate_type))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_get_stake_delegation(
  certificate: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_stake_delegation().into_result())
  })
  .map(|stake_delegation| RPtr::new(stake_delegation))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_get_owner_stake_delegation(
  certificate: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_owner_stake_delegation().into_result())
  })
  .map(|owner_stake_delegation| RPtr::new(owner_stake_delegation))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_get_pool_registration(
  certificate: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_pool_registration().into_result())
  })
  .map(|pool_registration| RPtr::new(pool_registration))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn certificate_get_pool_retirement(
  certificate: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    certificate
      .typed_ref::<Certificate>()
      .and_then(|certificate| certificate.get_pool_retirement().into_result())
  })
  .map(|pool_retirement| RPtr::new(pool_retirement))
  .response(result, error)
}
