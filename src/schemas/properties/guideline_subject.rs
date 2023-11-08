use super::*;
/// <https://schema.org/guidelineSubject>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GuidelineSubjectProperty {
	#[cfg(any(
		any(
			feature = "medical-entity-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalEntity(MedicalEntity),
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
	impl Serialize for GuidelineSubjectProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "medical-entity-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				GuidelineSubjectProperty::MedicalEntity(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				GuidelineSubjectProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for GuidelineSubjectProperty {
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
					feature = "medical-entity-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalEntity as Deserialize>::deserialize(deserializer),
				GuidelineSubjectProperty::MedicalEntity,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				GuidelineSubjectProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property guidelineSubject or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property guidelineSubject";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
