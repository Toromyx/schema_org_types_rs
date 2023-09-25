use super::*;
/// Indicates a correction to a [[CreativeWork]], either via a [[CorrectionComment]], textually or in another document.
///
/// <https://schema.org/correction>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum CorrectionProperty {
    #[cfg(any(
        any(
            feature = "correction-comment-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    CorrectionComment(CorrectionComment),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
