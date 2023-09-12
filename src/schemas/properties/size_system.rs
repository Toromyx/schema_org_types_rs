use super::*;
/// The size system used to identify a product's size. Typically either a standard (for example, "GS1" or "ISO-EN13402"), country code (for example "US" or "JP"), or a measuring system (for example "Metric" or "Imperial").
///
/// https://schema.org/sizeSystem
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SizeSystemProperty {
    #[cfg(any(
        feature = "size-system-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    SizeSystemEnumeration(SizeSystemEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
