use super::*;
/// <https://schema.org/valueReference>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
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
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ValueReferenceProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "defined-term-schema", feature = "pending-schema-section"),
					doc
				))]
				ValueReferenceProperty::DefinedTerm(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "enumeration-schema", feature = "general-schema-section"),
					doc
				))]
				ValueReferenceProperty::Enumeration(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "property-value-schema", feature = "general-schema-section"),
					doc
				))]
				ValueReferenceProperty::PropertyValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "qualitative-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValueReferenceProperty::QualitativeValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "quantitative-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValueReferenceProperty::QuantitativeValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "structured-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValueReferenceProperty::StructuredValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "measurement-type-enumeration-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ValueReferenceProperty::MeasurementTypeEnumeration(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				ValueReferenceProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ValueReferenceProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ValueReferenceProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "defined-term-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DefinedTerm as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::DefinedTerm,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "enumeration-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Enumeration as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::Enumeration,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "property-value-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PropertyValue as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::PropertyValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "qualitative-value-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<QualitativeValue as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::QualitativeValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "quantitative-value-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<QuantitativeValue as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "structured-value-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<StructuredValue as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::StructuredValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "measurement-type-enumeration-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MeasurementTypeEnumeration as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::MeasurementTypeEnumeration,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ValueReferenceProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property valueReference or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property valueReference";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
