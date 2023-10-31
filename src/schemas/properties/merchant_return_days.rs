use super::*;
/// <https://schema.org/merchantReturnDays>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MerchantReturnDaysProperty {
	#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
	Date(Date),
	#[cfg(any(
		any(feature = "date-time-schema", feature = "general-schema-section"),
		doc
	))]
	DateTime(DateTime),
	#[cfg(any(
		any(feature = "integer-schema", feature = "general-schema-section"),
		doc
	))]
	Integer(Integer),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MerchantReturnDaysProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
				MerchantReturnDaysProperty::Date(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "date-time-schema", feature = "general-schema-section"),
					doc
				))]
				MerchantReturnDaysProperty::DateTime(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "integer-schema", feature = "general-schema-section"),
					doc
				))]
				MerchantReturnDaysProperty::Integer(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for MerchantReturnDaysProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Date as Deserialize>::deserialize(deserializer),
				MerchantReturnDaysProperty::Date,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "date-time-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DateTime as Deserialize>::deserialize(deserializer),
				MerchantReturnDaysProperty::DateTime,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "integer-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Integer as Deserialize>::deserialize(deserializer),
				MerchantReturnDaysProperty::Integer,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property merchantReturnDays",
			))
		}
	}
}
