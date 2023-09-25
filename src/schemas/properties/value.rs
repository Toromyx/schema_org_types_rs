use super::*;
/// The value of a [[QuantitativeValue]] (including [[Observation]]) or property value node.\n\n* For [[QuantitativeValue]] and [[MonetaryAmount]], the recommended type for values is 'Number'.\n* For [[PropertyValue]], it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.\n* Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.\n* Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
///
/// <https://schema.org/value>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ValueProperty {
    #[cfg(any(
        any(feature = "boolean-schema", feature = "general-schema-section"),
        doc
    ))]
    Boolean(Boolean),
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
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
