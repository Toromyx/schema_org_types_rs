use super::*;
/// <https://schema.org/applicableCountry>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ApplicableCountryProperty {
    #[cfg(any(
        any(feature = "country-schema", feature = "general-schema-section"),
        doc
    ))]
    Country(Country),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
