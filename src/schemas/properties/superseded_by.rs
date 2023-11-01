use super::*;
/// <https://schema.org/supersededBy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SupersededByProperty {
	#[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
	Class(Class),
	#[cfg(any(
		any(feature = "enumeration-schema", feature = "general-schema-section"),
		doc
	))]
	Enumeration(Enumeration),
	#[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
	Property(Property),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for SupersededByProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
				SupersededByProperty::Class(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "enumeration-schema", feature = "general-schema-section"),
					doc
				))]
				SupersededByProperty::Enumeration(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
				SupersededByProperty::Property(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SupersededByProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(any(feature = "class-schema", feature = "meta-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Class as Deserialize>::deserialize(deserializer),
				SupersededByProperty::Class,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "enumeration-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Enumeration as Deserialize>::deserialize(deserializer),
				SupersededByProperty::Enumeration,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "property-schema", feature = "meta-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Property as Deserialize>::deserialize(deserializer),
				SupersededByProperty::Property,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property supersededBy",
			))
		}
	}
}
