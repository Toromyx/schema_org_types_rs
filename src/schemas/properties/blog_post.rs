use super::*;
/// A posting that is part of this blog.
///
/// <https://schema.org/blogPost>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BlogPostProperty {
    #[cfg(any(
        any(feature = "blog-posting-schema", feature = "general-schema-section"),
        doc
    ))]
    BlogPosting(BlogPosting),
}
