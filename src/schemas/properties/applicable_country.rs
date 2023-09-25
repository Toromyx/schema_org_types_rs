use super::*;
/// A country where a particular merchant return policy applies to, for example the two-letter ISO 3166-1 alpha-2 country code.
///
/// <https://schema.org/applicableCountry>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ApplicableCountryProperty {
    #[cfg(any(
        any(feature = "country-schema", feature = "general-schema-section"),
        doc
    ))]
    Country(Country),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
