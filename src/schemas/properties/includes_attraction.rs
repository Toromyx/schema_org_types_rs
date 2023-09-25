use super::*;
/// Attraction located at destination.
///
/// <https://schema.org/includesAttraction>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum IncludesAttractionProperty {
    #[cfg(any(
        any(
            feature = "tourist-attraction-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    TouristAttraction(TouristAttraction),
}
