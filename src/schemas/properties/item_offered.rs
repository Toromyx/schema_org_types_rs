use super::*;
/// <https://schema.org/itemOffered>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItemOfferedProperty {
	#[cfg(any(
		any(feature = "aggregate-offer-schema", feature = "general-schema-section"),
		doc
	))]
	AggregateOffer(AggregateOffer),
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
	Event(Event),
	#[cfg(any(
		any(feature = "menu-item-schema", feature = "general-schema-section"),
		doc
	))]
	MenuItem(MenuItem),
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
	#[cfg(any(
		any(feature = "service-schema", feature = "general-schema-section"),
		doc
	))]
	Service(Service),
	#[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
	Trip(Trip),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ItemOfferedProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "aggregate-offer-schema", feature = "general-schema-section"),
					doc
				))]
				ItemOfferedProperty::AggregateOffer(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "creative-work-schema", feature = "general-schema-section"),
					doc
				))]
				ItemOfferedProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
				ItemOfferedProperty::Event(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "menu-item-schema", feature = "general-schema-section"),
					doc
				))]
				ItemOfferedProperty::MenuItem(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "product-schema", feature = "general-schema-section"),
					doc
				))]
				ItemOfferedProperty::Product(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "service-schema", feature = "general-schema-section"),
					doc
				))]
				ItemOfferedProperty::Service(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
				ItemOfferedProperty::Trip(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ItemOfferedProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "aggregate-offer-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AggregateOffer as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::AggregateOffer,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "creative-work-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Event as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Event,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "menu-item-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MenuItem as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::MenuItem,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "product-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Product,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "service-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Service as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Service,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "trip-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Trip as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Trip,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property itemOffered",
			))
		}
	}
}
