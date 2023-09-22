use super::*;
/// An application that can complete the request.
///
/// https://schema.org/application
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ApplicationProperty {
    #[cfg(any(
        feature = "software-application-schema",
        feature = "general-schema-section"
    ))]
    SoftwareApplication(SoftwareApplication),
}