use super::*;
/// <https://schema.org/size>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SizeProperty {
	#[cfg(any(
		any(feature = "defined-term-schema", feature = "pending-schema-section"),
		doc
	))]
	DefinedTerm(DefinedTerm),
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
			feature = "size-specification-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	SizeSpecification(SizeSpecification),
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
	impl Serialize for SizeProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "defined-term-schema", feature = "pending-schema-section"),
					doc
				))]
				SizeProperty::DefinedTerm(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "quantitative-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SizeProperty::QuantitativeValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "size-specification-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SizeProperty::SizeSpecification(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				SizeProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				SizeProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SizeProperty {
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
				SizeProperty::DefinedTerm,
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
				SizeProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "size-specification-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<SizeSpecification as Deserialize>::deserialize(deserializer),
				SizeProperty::SizeSpecification,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				SizeProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				SizeProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property size or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str = "data did not match any variant of schema.org property size";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
