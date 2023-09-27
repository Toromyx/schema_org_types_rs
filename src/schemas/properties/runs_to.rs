use super::*;
/// <https://schema.org/runsTo>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RunsToProperty {
    #[cfg(any(
        any(feature = "vessel-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Vessel(Vessel),
}
