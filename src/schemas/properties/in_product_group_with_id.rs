use super::*;
/// Indicates the [[productGroupID]] for a [[ProductGroup]] that this product [[isVariantOf]].
///
/// https://schema.org/inProductGroupWithID
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InProductGroupWithIdProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
