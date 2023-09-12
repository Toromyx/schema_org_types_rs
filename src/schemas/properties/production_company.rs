use super::*;
/// The production company or studio responsible for the item, e.g. series, video game, episode etc.
///
/// https://schema.org/productionCompany
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProductionCompanyProperty {
    #[cfg(any(feature = "organization-schema", feature = "general-schema-section"))]
    Organization(Organization),
}
