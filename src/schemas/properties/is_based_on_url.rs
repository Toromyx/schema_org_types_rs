use super::*;
/// A resource that was used in the creation of this resource. This term can be repeated for multiple sources. For example, http://example.com/great-multiplication-intro.html.
///
/// https://schema.org/isBasedOnUrl
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IsBasedOnUrlProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "product-schema", feature = "general-schema-section"))]
    Product(Product),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
