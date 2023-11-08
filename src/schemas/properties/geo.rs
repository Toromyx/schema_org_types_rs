use super::*;
/// <https://schema.org/geo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GeoProperty {
	#[cfg(any(
		any(feature = "geo-coordinates-schema", feature = "general-schema-section"),
		doc
	))]
	GeoCoordinates(GeoCoordinates),
	#[cfg(any(
		any(feature = "geo-shape-schema", feature = "general-schema-section"),
		doc
	))]
	GeoShape(GeoShape),
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
	impl Serialize for GeoProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "geo-coordinates-schema", feature = "general-schema-section"),
					doc
				))]
				GeoProperty::GeoCoordinates(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "geo-shape-schema", feature = "general-schema-section"),
					doc
				))]
				GeoProperty::GeoShape(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				GeoProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for GeoProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "geo-coordinates-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<GeoCoordinates as Deserialize>::deserialize(deserializer),
				GeoProperty::GeoCoordinates,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "geo-shape-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<GeoShape as Deserialize>::deserialize(deserializer),
				GeoProperty::GeoShape,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				GeoProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property geo or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str = "data did not match any variant of schema.org property geo";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
