use super::*;
/// nonprofitStatus indicates the legal status of a non-profit organization in its primary place of business.
///
/// https://schema.org/nonprofitStatus
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NonprofitStatusProperty {
    #[cfg(any(feature = "nonprofit-type-schema", feature = "pending-schema-section"))]
    NonprofitType(NonprofitType),
}
