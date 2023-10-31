use super::*;
/// <https://schema.org/offers>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OffersProperty {
	#[cfg(any(
		any(feature = "demand-schema", feature = "general-schema-section"),
		doc
	))]
	Demand(Demand),
	#[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
	Offer(Offer),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for OffersProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "demand-schema", feature = "general-schema-section"),
					doc
				))]
				OffersProperty::Demand(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
				OffersProperty::Offer(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for OffersProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "demand-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Demand as Deserialize>::deserialize(deserializer),
				OffersProperty::Demand,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "offer-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Offer as Deserialize>::deserialize(deserializer),
				OffersProperty::Offer,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property offers",
			))
		}
	}
}
