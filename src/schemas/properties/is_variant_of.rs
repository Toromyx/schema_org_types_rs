use super::*;
/// <https://schema.org/isVariantOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum IsVariantOfProperty {
	#[cfg(any(
		any(feature = "product-group-schema", feature = "pending-schema-section"),
		doc
	))]
	ProductGroup(ProductGroup),
	#[cfg(any(
		any(feature = "product-model-schema", feature = "general-schema-section"),
		doc
	))]
	ProductModel(ProductModel),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for IsVariantOfProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "product-group-schema", feature = "pending-schema-section"),
					doc
				))]
				IsVariantOfProperty::ProductGroup(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "product-model-schema", feature = "general-schema-section"),
					doc
				))]
				IsVariantOfProperty::ProductModel(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for IsVariantOfProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "product-group-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ProductGroup as Deserialize>::deserialize(deserializer),
				IsVariantOfProperty::ProductGroup,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "product-model-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ProductModel as Deserialize>::deserialize(deserializer),
				IsVariantOfProperty::ProductModel,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property isVariantOf",
			))
		}
	}
}
