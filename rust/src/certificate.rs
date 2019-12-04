use js_chain_libs::CertificateKind as RCertificateKind;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CertificateKind {
  StakeDelegation = 0,
  OwnerStakeDelegation = 1,
  PoolRegistration = 2,
  PoolRetirement = 3,
  PoolUpdate = 4
}

impl From<RCertificateKind> for CertificateKind {
  fn from(certificate_kind: RCertificateKind) -> Self {
    match certificate_kind {
      RCertificateKind::StakeDelegation => CertificateKind::StakeDelegation,
      RCertificateKind::OwnerStakeDelegation => CertificateKind::OwnerStakeDelegation,
      RCertificateKind::PoolRegistration => CertificateKind::PoolRegistration,
      RCertificateKind::PoolRetirement => CertificateKind::PoolRetirement,
      RCertificateKind::PoolUpdate => CertificateKind::PoolUpdate
    }
  }
}
