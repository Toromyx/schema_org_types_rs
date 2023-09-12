use super::*;
/// A resource from which this work is derived or from which it is a modification or adaptation.
///
/// https://schema.org/isBasedOn
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsBasedOnProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
