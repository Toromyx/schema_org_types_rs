use super::*;
/// Nationality of the person.
///
/// <https://schema.org/nationality>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NationalityProperty {
    #[cfg(any(
        any(feature = "country-schema", feature = "general-schema-section"),
        doc
    ))]
    Country(Country),
}
