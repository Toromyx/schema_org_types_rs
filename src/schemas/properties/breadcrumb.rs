use super::*;
/// A set of links that can help a user understand and navigate a website hierarchy.
///
/// https://schema.org/breadcrumb
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BreadcrumbProperty {
    #[cfg(any(
        any(feature = "breadcrumb-list-schema", feature = "general-schema-section"),
        doc
    ))]
    BreadcrumbList(BreadcrumbList),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
