use super::*;
/// <https://schema.org/variablesMeasured>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariablesMeasuredProperty {
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
