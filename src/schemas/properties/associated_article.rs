use super::*;
/// A NewsArticle associated with the Media Object.
///
/// https://schema.org/associatedArticle
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AssociatedArticleProperty {
    #[cfg(any(
        any(feature = "news-article-schema", feature = "general-schema-section"),
        doc
    ))]
    NewsArticle(NewsArticle),
}
