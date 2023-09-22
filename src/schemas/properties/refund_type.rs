use super::*;
/// A refund type, from an enumerated list.
///
/// https://schema.org/refundType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RefundTypeProperty {
    #[cfg(any(
        feature = "refund-type-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    RefundTypeEnumeration(RefundTypeEnumeration),
}