use carbon_core::{borsh, CarbonDeserialize};

#[repr(u8)]
#[derive(
    CarbonDeserialize,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
    Default,
)]
pub enum CurveType {
    LinearV1,
    #[default]
    ConstantProductV1,
}
