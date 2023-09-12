use super::*;
/// A country where a particular merchant return policy applies to, for example the two-letter ISO 3166-1 alpha-2 country code.
///
/// https://schema.org/applicableCountry
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ApplicableCountryProperty {
    #[cfg(any(feature = "country-schema", feature = "general-schema-section"))]
    Country(Country),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
