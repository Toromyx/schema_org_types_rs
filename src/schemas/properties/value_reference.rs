use super::*;
/// <https://schema.org/valueReference>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ValueReferenceProperty {
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(
        any(feature = "enumeration-schema", feature = "general-schema-section"),
        doc
    ))]
    Enumeration(Enumeration),
    #[cfg(any(
        any(
            feature = "measurement-type-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(
        any(
            feature = "qualitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QualitativeValue(QualitativeValue),
    #[cfg(any(
        any(
            feature = "quantitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QuantitativeValue(QuantitativeValue),
    #[cfg(any(
        any(
            feature = "structured-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    StructuredValue(StructuredValue),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
