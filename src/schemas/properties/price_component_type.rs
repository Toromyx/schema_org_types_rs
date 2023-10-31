use super::*;
/// <https://schema.org/priceComponentType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PriceComponentTypeProperty {
	#[cfg(any(
		any(
			feature = "price-component-type-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	PriceComponentTypeEnumeration(PriceComponentTypeEnumeration),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PriceComponentTypeProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "price-component-type-enumeration-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PriceComponentTypeProperty::PriceComponentTypeEnumeration(ref inner) => {
					inner.serialize(serializer)
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for PriceComponentTypeProperty {
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
					feature = "price-component-type-enumeration-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PriceComponentTypeEnumeration as Deserialize>::deserialize(deserializer),
				PriceComponentTypeProperty::PriceComponentTypeEnumeration,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property priceComponentType",
			))
		}
	}
}
