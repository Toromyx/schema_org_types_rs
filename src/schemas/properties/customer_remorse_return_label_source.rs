use super::*;
/// <https://schema.org/customerRemorseReturnLabelSource>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CustomerRemorseReturnLabelSourceProperty {
    #[cfg(any(
        any(
            feature = "return-label-source-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
}
