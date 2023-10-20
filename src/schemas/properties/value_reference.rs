use super::*;
/// <https://schema.org/valueReference>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
	#[cfg(any(
		any(
			feature = "measurement-type-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	MeasurementTypeEnumeration(MeasurementTypeEnumeration),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
}
