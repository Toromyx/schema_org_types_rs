use super::*;
/// The geographic area associated with the audience.
///
/// <https://schema.org/geographicArea>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum GeographicAreaProperty {
    #[cfg(any(
        any(
            feature = "administrative-area-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AdministrativeArea(AdministrativeArea),
}
