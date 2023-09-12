use super::*;
/// Indicates a correction to a [[CreativeWork]], either via a [[CorrectionComment]], textually or in another document.
///
/// https://schema.org/correction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CorrectionProperty {
    #[cfg(any(
        feature = "correction-comment-schema",
        feature = "pending-schema-section"
    ))]
    CorrectionComment(CorrectionComment),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
