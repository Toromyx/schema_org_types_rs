use super::*;
/// <https://schema.org/leaseLength>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LeaseLengthProperty {
	#[cfg(any(
		any(feature = "duration-schema", feature = "general-schema-section"),
		doc
	))]
	Duration(Duration),
	#[cfg(any(
		any(
			feature = "quantitative-value-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	QuantitativeValue(QuantitativeValue),
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
	impl Serialize for LeaseLengthProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "duration-schema", feature = "general-schema-section"),
					doc
				))]
				LeaseLengthProperty::Duration(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "quantitative-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				LeaseLengthProperty::QuantitativeValue(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				LeaseLengthProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for LeaseLengthProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "duration-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Duration as Deserialize>::deserialize(deserializer),
				LeaseLengthProperty::Duration,
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
				LeaseLengthProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				LeaseLengthProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property leaseLength or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property leaseLength";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
