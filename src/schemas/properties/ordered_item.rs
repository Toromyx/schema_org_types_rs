use super::*;
/// <https://schema.org/orderedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OrderedItemProperty {
	OrderItem(OrderItem),
	Product(Product),
	Service(Service),
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
	impl Serialize for OrderedItemProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				OrderedItemProperty::OrderItem(ref inner) => inner.serialize(serializer),
				OrderedItemProperty::Product(ref inner) => inner.serialize(serializer),
				OrderedItemProperty::Service(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				OrderedItemProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for OrderedItemProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<OrderItem as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::OrderItem,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::Product,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Service as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::Service,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property orderedItem or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property orderedItem";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
