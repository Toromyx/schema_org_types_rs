use super::*;
/// The country where the product has to be sent to for returns, for example "Ireland" using the [[name]] property of [[Country]]. You can also provide the two-letter [ISO 3166-1 alpha-2 country code](http://en.wikipedia.org/wiki/ISO_3166-1). Note that this can be different from the country where the product was originally shipped from or sent to.
///
/// https://schema.org/returnPolicyCountry
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ReturnPolicyCountryProperty {
    #[cfg(any(feature = "country-schema", feature = "general-schema-section"))]
    Country(Country),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
