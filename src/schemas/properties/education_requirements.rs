use super::*;
/// <https://schema.org/educationRequirements>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum EducationRequirementsProperty {
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
	impl Serialize for EducationRequirementsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "educational-occupational-credential-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EducationRequirementsProperty::EducationalOccupationalCredential(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				EducationRequirementsProperty::Text(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for EducationRequirementsProperty {
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
					feature = "educational-occupational-credential-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<EducationalOccupationalCredential as Deserialize>::deserialize(deserializer),
				EducationRequirementsProperty::EducationalOccupationalCredential,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				EducationRequirementsProperty::Text,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property educationRequirements",
			))
		}
	}
}
