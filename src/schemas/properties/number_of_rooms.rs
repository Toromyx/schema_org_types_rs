use super::*;
/// <https://schema.org/numberOfRooms>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum NumberOfRoomsProperty {
	#[cfg(any(
		any(
			feature = "quantitative-value-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	QuantitativeValue(QuantitativeValue),
	#[cfg(any(
		any(feature = "number-schema", feature = "general-schema-section"),
		doc
	))]
	Number(Number),
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
	impl Serialize for NumberOfRoomsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "quantitative-value-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfRoomsProperty::QuantitativeValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "number-schema", feature = "general-schema-section"),
					doc
				))]
				NumberOfRoomsProperty::Number(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				NumberOfRoomsProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for NumberOfRoomsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(
					feature = "quantitative-value-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<QuantitativeValue as Deserialize>::deserialize(deserializer),
				NumberOfRoomsProperty::QuantitativeValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "number-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Number as Deserialize>::deserialize(deserializer),
				NumberOfRoomsProperty::Number,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				NumberOfRoomsProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property numberOfRooms or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property numberOfRooms";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
