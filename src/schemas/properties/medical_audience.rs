use super::*;
/// <https://schema.org/medicalAudience>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalAudienceProperty {
	#[cfg(any(
		any(
			feature = "medical-audience-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalAudience(MedicalAudience),
	#[cfg(any(
		any(
			feature = "medical-audience-type-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalAudienceType(MedicalAudienceType),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MedicalAudienceProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "medical-audience-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MedicalAudienceProperty::MedicalAudience(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-audience-type-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				MedicalAudienceProperty::MedicalAudienceType(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for MedicalAudienceProperty {
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
					feature = "medical-audience-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalAudience as Deserialize>::deserialize(deserializer),
				MedicalAudienceProperty::MedicalAudience,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-audience-type-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalAudienceType as Deserialize>::deserialize(deserializer),
				MedicalAudienceProperty::MedicalAudienceType,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property medicalAudience",
			))
		}
	}
}
