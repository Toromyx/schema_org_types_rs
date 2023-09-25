use super::*;
/// A description of the variant cover
/// for the issue, if the issue is a variant printing. For example, "Bryan Hitch
/// Variant Cover" or "2nd Printing Variant".
///
/// https://schema.org/variantCover
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum VariantCoverProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
