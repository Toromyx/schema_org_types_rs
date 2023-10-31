use super::*;
/// <https://schema.org/experienceRequirements>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ExperienceRequirementsProperty {
	#[cfg(any(
		any(
			feature = "occupational-experience-requirements-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	OccupationalExperienceRequirements(OccupationalExperienceRequirements),
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
	impl Serialize for ExperienceRequirementsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "occupational-experience-requirements-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ExperienceRequirementsProperty::OccupationalExperienceRequirements(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				ExperienceRequirementsProperty::Text(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ExperienceRequirementsProperty {
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
					feature = "occupational-experience-requirements-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<OccupationalExperienceRequirements as Deserialize>::deserialize(deserializer),
				ExperienceRequirementsProperty::OccupationalExperienceRequirements,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				ExperienceRequirementsProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property experienceRequirements",
			))
		}
	}
}
