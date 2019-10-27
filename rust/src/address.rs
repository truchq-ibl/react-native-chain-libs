use js_chain_libs::{AddressDiscrimination as RAddressDiscrimination};

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