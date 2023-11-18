use super::*;
/// <https://schema.org/serviceArea>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
pub enum ServiceAreaProperty {
	/// <https://schema.org/AdministrativeArea>
	AdministrativeArea(AdministrativeArea),
	/// <https://schema.org/GeoShape>
	GeoShape(GeoShape),
	/// <https://schema.org/Place>
	Place(Place),
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
	impl Serialize for ServiceAreaProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ServiceAreaProperty::AdministrativeArea(ref inner) => inner.serialize(serializer),
				ServiceAreaProperty::GeoShape(ref inner) => inner.serialize(serializer),
				ServiceAreaProperty::Place(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ServiceAreaProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ServiceAreaProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<AdministrativeArea as Deserialize>::deserialize(deserializer),
				ServiceAreaProperty::AdministrativeArea,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<GeoShape as Deserialize>::deserialize(deserializer),
				ServiceAreaProperty::GeoShape,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				ServiceAreaProperty::Place,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ServiceAreaProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property serviceArea or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property serviceArea";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
