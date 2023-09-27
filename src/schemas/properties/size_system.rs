use super::*;
/// <https://schema.org/sizeSystem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
