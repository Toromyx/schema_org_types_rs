use super::*;
/// A set of links that can help a user understand and navigate a website hierarchy.
///
/// https://schema.org/breadcrumb
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BreadcrumbProperty {
    #[cfg(any(feature = "breadcrumb-list-schema", feature = "general-schema-section"))]
    BreadcrumbList(BreadcrumbList),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
