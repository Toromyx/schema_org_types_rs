use super::*;
/// Indicates a property used as a constraint. For example, in the definition of a [[StatisticalVariable]]. The value is a property, either from within Schema.org or from other compatible (e.g. RDF) systems such as DataCommons.org or Wikidata.org.
///
/// https://schema.org/constraintProperty
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ConstraintPropertyProperty {
    #[cfg(any(feature = "property-schema", feature = "meta-schema-section"))]
    Property(Property),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
