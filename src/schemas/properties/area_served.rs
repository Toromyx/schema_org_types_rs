use super::*;
/// <https://schema.org/areaServed>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AreaServedProperty {
	AdministrativeArea(AdministrativeArea),
	GeoShape(GeoShape),
	Place(Place),
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
	impl Serialize for AreaServedProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				AreaServedProperty::AdministrativeArea(ref inner) => inner.serialize(serializer),
				AreaServedProperty::GeoShape(ref inner) => inner.serialize(serializer),
				AreaServedProperty::Place(ref inner) => inner.serialize(serializer),
				AreaServedProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AreaServedProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AreaServedProperty {
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
				AreaServedProperty::AdministrativeArea,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<GeoShape as Deserialize>::deserialize(deserializer),
				AreaServedProperty::GeoShape,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				AreaServedProperty::Place,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				AreaServedProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AreaServedProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property areaServed or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property areaServed";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
