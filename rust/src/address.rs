use js_chain_libs::AddressDiscrimination as RAddressDiscrimination;
use js_chain_libs::AddressKind as RAddressKind;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AddressDiscrimination {
  Production = 0,
  Test = 1
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AddressKind {
  Single = 0,
  Group = 1,
  Account = 2,
  Multisig = 3
}

impl From<AddressDiscrimination> for RAddressDiscrimination {
  fn from(ad: AddressDiscrimination) -> Self {
    match ad {
      AddressDiscrimination::Production => RAddressDiscrimination::Production,
      AddressDiscrimination::Test => RAddressDiscrimination::Test
    }
  }
}

impl From<RAddressDiscrimination> for AddressDiscrimination {
  fn from(ad: RAddressDiscrimination) -> Self {
    match ad {
      RAddressDiscrimination::Production => AddressDiscrimination::Production,
      RAddressDiscrimination::Test => AddressDiscrimination::Test
    }
  }
}

impl From<i32> for AddressDiscrimination {
  fn from(int: i32) -> Self {
    match int {
      x if x == Self::Production as i32 => Self::Production,
      x if x == Self::Test as i32 => Self::Test,
      _ => panic!("Unknown AddressDiscrimination enum value {}", int)
    }
  }
}

impl From<RAddressKind> for AddressKind {
  fn from(address_kind: RAddressKind) -> Self {
    match address_kind {
      RAddressKind::Single => AddressKind::Single,
      RAddressKind::Group => AddressKind::Group,
      RAddressKind::Account => AddressKind::Account,
      RAddressKind::Multisig => AddressKind::Multisig
    }
  }
}
