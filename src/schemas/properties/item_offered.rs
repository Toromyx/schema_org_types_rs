use super::*;
/// <https://schema.org/itemOffered>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItemOfferedProperty {
	/// <https://schema.org/AggregateOffer>
	AggregateOffer(AggregateOffer),
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Event>
	Event(Event),
	/// <https://schema.org/MenuItem>
	MenuItem(MenuItem),
	/// <https://schema.org/Product>
	Product(Product),
	/// <https://schema.org/Service>
	Service(Service),
	/// <https://schema.org/Trip>
	Trip(Trip),
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
	impl Serialize for ItemOfferedProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ItemOfferedProperty::AggregateOffer(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::Event(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::MenuItem(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::Product(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::Service(ref inner) => inner.serialize(serializer),
				ItemOfferedProperty::Trip(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ItemOfferedProperty::SerdeFail(ref inner) => inner.serialize(serializer),
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
			if let Ok(ok) = Result::map(
				<AggregateOffer as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::AggregateOffer,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Event as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Event,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<MenuItem as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::MenuItem,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Product,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Service as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Service,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Trip as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::Trip,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ItemOfferedProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property itemOffered or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property itemOffered";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
