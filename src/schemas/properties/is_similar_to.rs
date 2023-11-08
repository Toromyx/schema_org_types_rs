use super::*;
/// <https://schema.org/isSimilarTo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IsSimilarToProperty {
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
	impl Serialize for IsSimilarToProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "product-schema", feature = "general-schema-section"),
					doc
				))]
				IsSimilarToProperty::Product(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "service-schema", feature = "general-schema-section"),
					doc
				))]
				IsSimilarToProperty::Service(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				IsSimilarToProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IsSimilarToProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "product-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				IsSimilarToProperty::Product,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "service-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Service as Deserialize>::deserialize(deserializer),
				IsSimilarToProperty::Service,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				IsSimilarToProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property isSimilarTo or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property isSimilarTo";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
