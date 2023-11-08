use super::*;
/// <https://schema.org/variableMeasured>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum VariableMeasuredProperty {
	Property(Property),
	PropertyValue(PropertyValue),
	StatisticalVariable(StatisticalVariable),
	Text(Text),
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
	impl Serialize for VariableMeasuredProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				VariableMeasuredProperty::Property(ref inner) => inner.serialize(serializer),
				VariableMeasuredProperty::PropertyValue(ref inner) => inner.serialize(serializer),
				VariableMeasuredProperty::StatisticalVariable(ref inner) => {
					inner.serialize(serializer)
				}
				VariableMeasuredProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				VariableMeasuredProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for VariableMeasuredProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<Property as Deserialize>::deserialize(deserializer),
				VariableMeasuredProperty::Property,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<PropertyValue as Deserialize>::deserialize(deserializer),
				VariableMeasuredProperty::PropertyValue,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<StatisticalVariable as Deserialize>::deserialize(deserializer),
				VariableMeasuredProperty::StatisticalVariable,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				VariableMeasuredProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				VariableMeasuredProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property variableMeasured or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property variableMeasured";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
