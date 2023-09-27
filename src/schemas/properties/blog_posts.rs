use super::*;
/// <https://schema.org/blogPosts>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BlogPostsProperty {
    #[cfg(any(
        any(feature = "blog-posting-schema", feature = "general-schema-section"),
        doc
    ))]
    BlogPosting(BlogPosting),
}
