use super::*;
/// <https://schema.org/orderedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OrderedItemProperty {
	#[cfg(any(
		any(feature = "order-item-schema", feature = "general-schema-section"),
		doc
	))]
	OrderItem(OrderItem),
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
				#[cfg(any(
					any(feature = "order-item-schema", feature = "general-schema-section"),
					doc
				))]
				OrderedItemProperty::OrderItem(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "product-schema", feature = "general-schema-section"),
					doc
				))]
				OrderedItemProperty::Product(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "service-schema", feature = "general-schema-section"),
					doc
				))]
				OrderedItemProperty::Service(ref inner) => inner.serialize(serializer),
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
			#[cfg(any(
				any(feature = "order-item-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<OrderItem as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::OrderItem,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "product-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::Product,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "service-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Service as Deserialize>::deserialize(deserializer),
				OrderedItemProperty::Service,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property orderedItem",
			))
		}
	}
}
