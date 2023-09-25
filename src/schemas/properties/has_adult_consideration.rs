use super::*;
/// Used to tag an item to be intended or suitable for consumption or use by adults only.
///
/// <https://schema.org/hasAdultConsideration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasAdultConsiderationProperty {
    #[cfg(any(
        any(
            feature = "adult-oriented-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    AdultOrientedEnumeration(AdultOrientedEnumeration),
}
