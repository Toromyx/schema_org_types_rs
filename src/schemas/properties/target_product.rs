use super::*;
/// Target Operating System / Product to which the code applies.  If applies to several versions, just the product name can be used.
///
/// <https://schema.org/targetProduct>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TargetProductProperty {
    #[cfg(any(
        any(
            feature = "software-application-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    SoftwareApplication(SoftwareApplication),
}
