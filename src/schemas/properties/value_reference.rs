use super::*;
/// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
///
/// https://schema.org/valueReference
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ValueReferenceProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(feature = "enumeration-schema", feature = "general-schema-section"))]
    Enumeration(Enumeration),
    #[cfg(any(
        feature = "measurement-type-enumeration-schema",
        feature = "pending-schema-section"
    ))]
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    #[cfg(any(feature = "property-value-schema", feature = "general-schema-section"))]
    PropertyValue(PropertyValue),
    #[cfg(any(
        feature = "qualitative-value-schema",
        feature = "general-schema-section"
    ))]
    QualitativeValue(QualitativeValue),
    #[cfg(any(
        feature = "quantitative-value-schema",
        feature = "general-schema-section"
    ))]
    QuantitativeValue(QuantitativeValue),
    #[cfg(any(
        feature = "structured-value-schema",
        feature = "general-schema-section"
    ))]
    StructuredValue(StructuredValue),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
