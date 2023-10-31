use super::*;
/// <https://schema.org/areaServed>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AreaServedProperty {
	#[cfg(any(
		any(
			feature = "administrative-area-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	AdministrativeArea(AdministrativeArea),
	#[cfg(any(
		any(feature = "geo-shape-schema", feature = "general-schema-section"),
		doc
	))]
	GeoShape(GeoShape),
	#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
	Place(Place),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
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
				#[cfg(any(
					any(
						feature = "administrative-area-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AreaServedProperty::AdministrativeArea(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "geo-shape-schema", feature = "general-schema-section"),
					doc
				))]
				AreaServedProperty::GeoShape(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
				AreaServedProperty::Place(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				AreaServedProperty::Text(ref inner) => inner.serialize(serializer),
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
			#[cfg(any(
				any(
					feature = "administrative-area-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AdministrativeArea as Deserialize>::deserialize(deserializer),
				AreaServedProperty::AdministrativeArea,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "geo-shape-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<GeoShape as Deserialize>::deserialize(deserializer),
				AreaServedProperty::GeoShape,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "place-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Place as Deserialize>::deserialize(deserializer),
				AreaServedProperty::Place,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				AreaServedProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property areaServed",
			))
		}
	}
}
