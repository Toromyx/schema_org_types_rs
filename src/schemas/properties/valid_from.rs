use super::*;
/// <https://schema.org/validFrom>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ValidFromProperty {
	Date(Date),
	DateTime(DateTime),
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
	impl Serialize for ValidFromProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ValidFromProperty::Date(ref inner) => inner.serialize(serializer),
				ValidFromProperty::DateTime(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ValidFromProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ValidFromProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Date as Deserialize>::deserialize(deserializer),
				ValidFromProperty::Date,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<DateTime as Deserialize>::deserialize(deserializer),
				ValidFromProperty::DateTime,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ValidFromProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property validFrom or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property validFrom";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
