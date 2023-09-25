use super::*;
/// The variableMeasured property can indicate (repeated as necessary) the  variables that are measured in some dataset, either described as text or as pairs of identifier and description using PropertyValue, or more explicitly as a [[StatisticalVariable]].
///
/// <https://schema.org/variableMeasured>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum VariableMeasuredProperty {
    #[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
    Property(Property),
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(
        any(
            feature = "statistical-variable-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    StatisticalVariable(StatisticalVariable),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
