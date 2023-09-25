use super::*;
/// nonprofitStatus indicates the legal status of a non-profit organization in its primary place of business.
///
/// <https://schema.org/nonprofitStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NonprofitStatusProperty {
    #[cfg(any(
        any(feature = "nonprofit-type-schema", feature = "pending-schema-section"),
        doc
    ))]
    NonprofitType(NonprofitType),
}
