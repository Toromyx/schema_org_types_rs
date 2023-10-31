use super::*;
/// <https://schema.org/coursePrerequisites>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CoursePrerequisitesProperty {
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
	impl Serialize for CoursePrerequisitesProperty {
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
				CoursePrerequisitesProperty::AlignmentObject(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "course-schema", feature = "general-schema-section"),
					doc
				))]
				CoursePrerequisitesProperty::Course(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				CoursePrerequisitesProperty::Text(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for CoursePrerequisitesProperty {
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
				CoursePrerequisitesProperty::AlignmentObject,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "course-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Course as Deserialize>::deserialize(deserializer),
				CoursePrerequisitesProperty::Course,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				CoursePrerequisitesProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property coursePrerequisites",
			))
		}
	}
}
