use super::*;
/// Countries for which the application is not supported. You can also provide the two-letter ISO 3166-1 alpha-2 country code.
///
/// https://schema.org/countriesNotSupported
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CountriesNotSupportedProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
