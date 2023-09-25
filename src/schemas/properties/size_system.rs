use super::*;
/// The size system used to identify a product's size. Typically either a standard (for example, "GS1" or "ISO-EN13402"), country code (for example "US" or "JP"), or a measuring system (for example "Metric" or "Imperial").
///
/// <https://schema.org/sizeSystem>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SizeSystemProperty {
    #[cfg(any(
        any(
            feature = "size-system-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    SizeSystemEnumeration(SizeSystemEnumeration),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
