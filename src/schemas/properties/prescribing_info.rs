use super::*;
/// Link to prescribing information for the drug.
///
/// https://schema.org/prescribingInfo
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PrescribingInfoProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
