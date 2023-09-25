use super::*;
/// The method (from an enumeration) by which the customer obtains a return shipping label for a defect product.
///
/// https://schema.org/itemDefectReturnLabelSource
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ItemDefectReturnLabelSourceProperty {
    #[cfg(any(
        any(
            feature = "return-label-source-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
}
