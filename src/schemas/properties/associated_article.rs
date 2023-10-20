use super::*;
/// <https://schema.org/associatedArticle>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedArticleProperty {
	#[cfg(any(
		any(feature = "news-article-schema", feature = "general-schema-section"),
		doc
	))]
	NewsArticle(NewsArticle),
}
