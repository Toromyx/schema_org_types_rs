use super::*;
/// <https://schema.org/category>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CategoryProperty {
	/// <https://schema.org/CategoryCode>
	CategoryCode(CategoryCode),
	/// <https://schema.org/Thing>
	Thing(Thing),
	/// <https://schema.org/PhysicalActivityCategory>
	PhysicalActivityCategory(PhysicalActivityCategory),
	/// <https://schema.org/URL>
	Url(Url),
	/// <https://schema.org/Text>
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
	impl Serialize for CategoryProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				CategoryProperty::CategoryCode(ref inner) => inner.serialize(serializer),
				CategoryProperty::Thing(ref inner) => inner.serialize(serializer),
				CategoryProperty::PhysicalActivityCategory(ref inner) => {
					inner.serialize(serializer)
				}
				CategoryProperty::Url(ref inner) => inner.serialize(serializer),
				CategoryProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				CategoryProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for CategoryProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<CategoryCode as Deserialize>::deserialize(deserializer),
				CategoryProperty::CategoryCode,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Thing as Deserialize>::deserialize(deserializer),
				CategoryProperty::Thing,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<PhysicalActivityCategory as Deserialize>::deserialize(deserializer),
				CategoryProperty::PhysicalActivityCategory,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				CategoryProperty::Url,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				CategoryProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				CategoryProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property category or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property category";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
