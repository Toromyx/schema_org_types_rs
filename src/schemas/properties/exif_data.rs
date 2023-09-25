use super::*;
/// exif data for this object.
///
/// <https://schema.org/exifData>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ExifDataProperty {
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
