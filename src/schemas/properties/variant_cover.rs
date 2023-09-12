use super::*;
/// A description of the variant cover
/// for the issue, if the issue is a variant printing. For example, "Bryan Hitch
/// Variant Cover" or "2nd Printing Variant".
///
/// https://schema.org/variantCover
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariantCoverProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
