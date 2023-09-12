use super::*;
/// Target Operating System / Product to which the code applies.  If applies to several versions, just the product name can be used.
///
/// https://schema.org/targetProduct
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TargetProductProperty {
    #[cfg(any(
        feature = "software-application-schema",
        feature = "general-schema-section"
    ))]
    SoftwareApplication(SoftwareApplication),
}
