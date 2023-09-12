use super::*;
/// The method (from an enumeration) by which the customer obtains a return shipping label for a defect product.
///
/// https://schema.org/itemDefectReturnLabelSource
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemDefectReturnLabelSourceProperty {
    #[cfg(any(
        feature = "return-label-source-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
}
