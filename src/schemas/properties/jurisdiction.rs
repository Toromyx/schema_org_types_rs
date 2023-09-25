use super::*;
/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
///
/// <https://schema.org/jurisdiction>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum JurisdictionProperty {
    #[cfg(any(
        any(
            feature = "administrative-area-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AdministrativeArea(AdministrativeArea),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
