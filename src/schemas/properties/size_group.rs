use super::*;
/// The size group (also known as "size type") for a product's size. Size groups are common in the fashion industry to define size segments and suggested audiences for wearable products. Multiple values can be combined, for example "men's big and tall", "petite maternity" or "regular"
///
/// https://schema.org/sizeGroup
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SizeGroupProperty {
    #[cfg(any(
        feature = "size-group-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    SizeGroupEnumeration(SizeGroupEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
