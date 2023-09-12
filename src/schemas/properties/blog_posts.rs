use super::*;
/// Indicates a post that is part of a [[Blog]]. Note that historically, what we term a "Blog" was once known as a "weblog", and that what we term a "BlogPosting" is now often colloquially referred to as a "blog".
///
/// https://schema.org/blogPosts
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BlogPostsProperty {
    #[cfg(any(feature = "blog-posting-schema", feature = "general-schema-section"))]
    BlogPosting(BlogPosting),
}
