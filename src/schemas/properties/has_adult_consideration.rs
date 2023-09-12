use super::*;
/// Used to tag an item to be intended or suitable for consumption or use by adults only.
///
/// https://schema.org/hasAdultConsideration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasAdultConsiderationProperty {
    #[cfg(any(
        feature = "adult-oriented-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    AdultOrientedEnumeration(AdultOrientedEnumeration),
}
