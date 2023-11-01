use super::*;
/// <https://schema.org/hasHealthAspect>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum HasHealthAspectProperty {
	#[cfg(any(
		any(
			feature = "health-aspect-enumeration-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	HealthAspectEnumeration(HealthAspectEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for HasHealthAspectProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "health-aspect-enumeration-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasHealthAspectProperty::HealthAspectEnumeration(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for HasHealthAspectProperty {
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
					feature = "health-aspect-enumeration-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<HealthAspectEnumeration as Deserialize>::deserialize(deserializer),
				HasHealthAspectProperty::HealthAspectEnumeration,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property hasHealthAspect",
			))
		}
	}
}
