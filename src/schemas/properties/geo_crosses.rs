use super::*;
/// <https://schema.org/geoCrosses>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GeoCrossesProperty {
	#[cfg(any(
		any(
			feature = "geospatial-geometry-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	GeospatialGeometry(GeospatialGeometry),
	#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
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
	impl Serialize for GeoCrossesProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "geospatial-geometry-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				GeoCrossesProperty::GeospatialGeometry(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
				GeoCrossesProperty::Place(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				GeoCrossesProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for GeoCrossesProperty {
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
					feature = "geospatial-geometry-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<GeospatialGeometry as Deserialize>::deserialize(deserializer),
				GeoCrossesProperty::GeospatialGeometry,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				GeoCrossesProperty::Place,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				GeoCrossesProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property geoCrosses or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property geoCrosses";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
