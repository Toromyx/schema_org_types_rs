use super::*;
/// The size group (also known as "size type") for a product's size. Size groups are common in the fashion industry to define size segments and suggested audiences for wearable products. Multiple values can be combined, for example "men's big and tall", "petite maternity" or "regular"
///
/// <https://schema.org/sizeGroup>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
