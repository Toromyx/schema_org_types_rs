use super::*;
/// The country. For example, USA. You can also provide the two-letter [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1).
///
/// https://schema.org/addressCountry
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AddressCountryProperty {
    #[cfg(any(feature = "country-schema", feature = "general-schema-section"))]
    Country(Country),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
