use super::*;
/// Nationality of the person.
///
/// https://schema.org/nationality
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NationalityProperty {
    #[cfg(any(feature = "country-schema", feature = "general-schema-section"))]
    Country(Country),
}
