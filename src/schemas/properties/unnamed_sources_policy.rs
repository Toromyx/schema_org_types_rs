use super::*;
/// For an [[Organization]] (typically a [[NewsMediaOrganization]]), a statement about policy on use of unnamed sources and the decision process required.
///
/// <https://schema.org/unnamedSourcesPolicy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum UnnamedSourcesPolicyProperty {
    #[cfg(any(
        any(feature = "creative-work-schema", feature = "general-schema-section"),
        doc
    ))]
    CreativeWork(CreativeWork),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
