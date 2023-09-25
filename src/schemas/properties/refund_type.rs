use super::*;
/// A refund type, from an enumerated list.
///
/// <https://schema.org/refundType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RefundTypeProperty {
    #[cfg(any(
        any(
            feature = "refund-type-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    RefundTypeEnumeration(RefundTypeEnumeration),
}
