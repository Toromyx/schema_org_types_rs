use super::*;
/// <https://schema.org/isBasedOnUrl>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
pub enum IsBasedOnUrlProperty {
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Product>
	Product(Product),
	/// <https://schema.org/URL>
	Url(Url),
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
	impl Serialize for IsBasedOnUrlProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				IsBasedOnUrlProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				IsBasedOnUrlProperty::Product(ref inner) => inner.serialize(serializer),
				IsBasedOnUrlProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				IsBasedOnUrlProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IsBasedOnUrlProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				IsBasedOnUrlProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				IsBasedOnUrlProperty::Product,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				IsBasedOnUrlProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				IsBasedOnUrlProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property isBasedOnUrl or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property isBasedOnUrl";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
