use super::*;
/// <https://schema.org/numberOfCredits>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum NumberOfCreditsProperty {
	/// <https://schema.org/StructuredValue>
	StructuredValue(StructuredValue),
	/// <https://schema.org/Integer>
	Integer(Integer),
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
	impl Serialize for NumberOfCreditsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				NumberOfCreditsProperty::StructuredValue(ref inner) => inner.serialize(serializer),
				NumberOfCreditsProperty::Integer(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				NumberOfCreditsProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for NumberOfCreditsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<StructuredValue as Deserialize>::deserialize(deserializer),
				NumberOfCreditsProperty::StructuredValue,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Integer as Deserialize>::deserialize(deserializer),
				NumberOfCreditsProperty::Integer,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				NumberOfCreditsProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property numberOfCredits or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property numberOfCredits";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
