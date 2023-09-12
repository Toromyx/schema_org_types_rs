use super::*;
/// Indicates the kind of statistic represented by a [[StatisticalVariable]], e.g. mean, count etc. The value of statType is a property, either from within Schema.org (e.g. [[count]], [[median]], [[marginOfError]], [[maxValue]], [[minValue]]) or from other compatible (e.g. RDF) systems such as DataCommons.org or Wikidata.org.
///
/// https://schema.org/statType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StatTypeProperty {
    #[cfg(any(feature = "property-schema", feature = "meta-schema-section"))]
    Property(Property),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
