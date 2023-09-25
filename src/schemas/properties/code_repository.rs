use super::*;
/// Link to the repository where the un-compiled, human readable code and related code is located (SVN, GitHub, CodePlex).
///
/// <https://schema.org/codeRepository>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CodeRepositoryProperty {
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
