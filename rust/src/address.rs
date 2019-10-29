use js_chain_libs::AddressDiscrimination as RAddressDiscrimination;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AddressDiscrimination {
  Production = 0,
  Test = 1
}

impl From<AddressDiscrimination> for RAddressDiscrimination {
  fn from(ad: AddressDiscrimination) -> Self {
    match ad {
      AddressDiscrimination::Production => RAddressDiscrimination::Production,
      AddressDiscrimination::Test => RAddressDiscrimination::Test
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
