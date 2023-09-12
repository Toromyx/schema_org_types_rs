use super::*;
/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
///
/// https://schema.org/jurisdiction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum JurisdictionProperty {
    #[cfg(any(
        feature = "administrative-area-schema",
        feature = "general-schema-section"
    ))]
    AdministrativeArea(AdministrativeArea),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
