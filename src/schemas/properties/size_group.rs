use super::*;
/// <https://schema.org/sizeGroup>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SizeGroupProperty {
    #[cfg(any(
        any(
            feature = "size-group-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    SizeGroupEnumeration(SizeGroupEnumeration),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
