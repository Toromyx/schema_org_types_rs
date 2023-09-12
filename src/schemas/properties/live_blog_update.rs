use super::*;
/// An update to the LiveBlog.
///
/// https://schema.org/liveBlogUpdate
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LiveBlogUpdateProperty {
    #[cfg(any(feature = "blog-posting-schema", feature = "general-schema-section"))]
    BlogPosting(BlogPosting),
}
