use super::*;
/// <https://schema.org/status>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum StatusProperty {
	#[cfg(any(
		any(
			feature = "event-status-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	EventStatusType(EventStatusType),
	#[cfg(any(
		any(
			feature = "medical-study-status-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalStudyStatus(MedicalStudyStatus),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
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
	impl Serialize for StatusProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "event-status-type-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				StatusProperty::EventStatusType(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-study-status-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				StatusProperty::MedicalStudyStatus(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				StatusProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				StatusProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for StatusProperty {
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
					feature = "event-status-type-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<EventStatusType as Deserialize>::deserialize(deserializer),
				StatusProperty::EventStatusType,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-study-status-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalStudyStatus as Deserialize>::deserialize(deserializer),
				StatusProperty::MedicalStudyStatus,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				StatusProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				StatusProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property status or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property status";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
