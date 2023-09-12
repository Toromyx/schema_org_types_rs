use super::*;
/// Indicates a Web page or service by URL, for product return.
///
/// https://schema.org/productReturnLink
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProductReturnLinkProperty {
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
