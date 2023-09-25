use super::*;
/// An application that can complete the request.
///
/// https://schema.org/actionApplication
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ActionApplicationProperty {
    #[cfg(any(
        any(
            feature = "software-application-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SoftwareApplication(SoftwareApplication),
}
