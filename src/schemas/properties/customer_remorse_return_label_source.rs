use super::*;
/// <https://schema.org/customerRemorseReturnLabelSource>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CustomerRemorseReturnLabelSourceProperty {
	#[cfg(any(
		any(
			feature = "return-label-source-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	ReturnLabelSourceEnumeration(ReturnLabelSourceEnumeration),
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
	impl Serialize for CustomerRemorseReturnLabelSourceProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "return-label-source-enumeration-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CustomerRemorseReturnLabelSourceProperty::ReturnLabelSourceEnumeration(
					ref inner,
				) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				CustomerRemorseReturnLabelSourceProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for CustomerRemorseReturnLabelSourceProperty {
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
					feature = "return-label-source-enumeration-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ReturnLabelSourceEnumeration as Deserialize>::deserialize(deserializer),
				CustomerRemorseReturnLabelSourceProperty::ReturnLabelSourceEnumeration,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				CustomerRemorseReturnLabelSourceProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property customerRemorseReturnLabelSource or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str = "data did not match any variant of schema.org property customerRemorseReturnLabelSource";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
