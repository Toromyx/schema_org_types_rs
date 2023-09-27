use super::*;
/// <https://schema.org/funding>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FundingProperty {
    #[cfg(any(any(feature = "grant-schema", feature = "pending-schema-section"), doc))]
    Grant(Grant),
}
