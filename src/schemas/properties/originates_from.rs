use super::*;
/// The vasculature the lymphatic structure originates, or afferents, from.
///
/// https://schema.org/originatesFrom
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OriginatesFromProperty {
    #[cfg(any(feature = "vessel-schema", feature = "health-lifesci-schema-section"))]
    Vessel(Vessel),
}