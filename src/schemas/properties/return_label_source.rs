use super::*;
/// <https://schema.org/returnLabelSource>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ReturnLabelSourceProperty {
    #[cfg(any(
        any(
            feature = "return-label-source-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
}
