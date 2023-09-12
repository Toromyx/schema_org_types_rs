use super::*;
/// The variableMeasured property can indicate (repeated as necessary) the  variables that are measured in some dataset, either described as text or as pairs of identifier and description using PropertyValue, or more explicitly as a [[StatisticalVariable]].
///
/// https://schema.org/variableMeasured
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariableMeasuredProperty {
    #[cfg(any(feature = "property-schema", feature = "meta-schema-section"))]
    Property(Property),
    #[cfg(any(feature = "property-value-schema", feature = "general-schema-section"))]
    PropertyValue(PropertyValue),
    #[cfg(any(
        feature = "statistical-variable-schema",
        feature = "pending-schema-section"
    ))]
    StatisticalVariable(StatisticalVariable),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
