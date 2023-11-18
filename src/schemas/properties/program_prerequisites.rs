use super::*;
/// <https://schema.org/programPrerequisites>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ProgramPrerequisitesProperty {
	/// <https://schema.org/AlignmentObject>
	AlignmentObject(AlignmentObject),
	/// <https://schema.org/Course>
	Course(Course),
	/// <https://schema.org/EducationalOccupationalCredential>
	EducationalOccupationalCredential(EducationalOccupationalCredential),
	/// <https://schema.org/Text>
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
	impl Serialize for ProgramPrerequisitesProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				ProgramPrerequisitesProperty::AlignmentObject(ref inner) => {
					inner.serialize(serializer)
				}
				ProgramPrerequisitesProperty::Course(ref inner) => inner.serialize(serializer),
				ProgramPrerequisitesProperty::EducationalOccupationalCredential(ref inner) => {
					inner.serialize(serializer)
				}
				ProgramPrerequisitesProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ProgramPrerequisitesProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ProgramPrerequisitesProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<AlignmentObject as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::AlignmentObject,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Course as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::Course,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<EducationalOccupationalCredential as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::EducationalOccupationalCredential,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property programPrerequisites or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property programPrerequisites";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
