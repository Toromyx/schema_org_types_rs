use super::*;
/// The operating organization, if different from the provider.  This enables the representation of services that are provided by an organization, but operated by another organization like a subcontractor.
///
/// <https://schema.org/serviceOperator>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ServiceOperatorProperty {
    #[cfg(any(
        any(feature = "organization-schema", feature = "general-schema-section"),
        doc
    ))]
    Organization(Organization),
}
