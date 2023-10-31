use super::*;
/// <https://schema.org/itemDefectReturnFees>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ItemDefectReturnFeesProperty {
	#[cfg(any(
		any(
			feature = "return-fees-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	ReturnFeesEnumeration(ReturnFeesEnumeration),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ItemDefectReturnFeesProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "return-fees-enumeration-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ItemDefectReturnFeesProperty::ReturnFeesEnumeration(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ItemDefectReturnFeesProperty {
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
					feature = "return-fees-enumeration-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ReturnFeesEnumeration as Deserialize>::deserialize(deserializer),
				ItemDefectReturnFeesProperty::ReturnFeesEnumeration,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property itemDefectReturnFees",
			))
		}
	}
}
