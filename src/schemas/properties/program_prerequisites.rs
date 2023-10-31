use super::*;
/// <https://schema.org/programPrerequisites>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ProgramPrerequisitesProperty {
	#[cfg(any(
		any(
			feature = "alignment-object-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	AlignmentObject(AlignmentObject),
	#[cfg(any(
		any(feature = "course-schema", feature = "general-schema-section"),
		doc
	))]
	Course(Course),
	#[cfg(any(
		any(
			feature = "educational-occupational-credential-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	EducationalOccupationalCredential(EducationalOccupationalCredential),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
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
				#[cfg(any(
					any(
						feature = "alignment-object-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProgramPrerequisitesProperty::AlignmentObject(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "course-schema", feature = "general-schema-section"),
					doc
				))]
				ProgramPrerequisitesProperty::Course(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "educational-occupational-credential-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ProgramPrerequisitesProperty::EducationalOccupationalCredential(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				ProgramPrerequisitesProperty::Text(ref inner) => inner.serialize(serializer),
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
			#[cfg(any(
				any(
					feature = "alignment-object-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AlignmentObject as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::AlignmentObject,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "course-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Course as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::Course,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "educational-occupational-credential-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<EducationalOccupationalCredential as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::EducationalOccupationalCredential,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				ProgramPrerequisitesProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property programPrerequisites",
			))
		}
	}
}
