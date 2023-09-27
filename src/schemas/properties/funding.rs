use super::*;
/// <https://schema.org/funding>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum FundingProperty {
    #[cfg(any(any(feature = "grant-schema", feature = "pending-schema-section"), doc))]
    Grant(Grant),
}
