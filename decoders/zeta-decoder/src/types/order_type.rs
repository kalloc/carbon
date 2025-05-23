use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum OrderType {
    Limit,
    PostOnly,
    FillOrKill,
    ImmediateOrCancel,
    PostOnlySlide,
    PostOnlyFront,
}
