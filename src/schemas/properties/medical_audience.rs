use super::*;
/// <https://schema.org/medicalAudience>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MedicalAudienceProperty {
	/// <https://schema.org/MedicalAudience>
	MedicalAudience(MedicalAudience),
	/// <https://schema.org/MedicalAudienceType>
	MedicalAudienceType(MedicalAudienceType),
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
	impl Serialize for MedicalAudienceProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				MedicalAudienceProperty::MedicalAudience(ref inner) => inner.serialize(serializer),
				MedicalAudienceProperty::MedicalAudienceType(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(all(feature = "fallible", feature = "serde"))]
				MedicalAudienceProperty::SerdeFail(ref inner) => inner.serialize(serializer),
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
			if let Ok(ok) = Result::map(
				<MedicalAudience as Deserialize>::deserialize(deserializer),
				MedicalAudienceProperty::MedicalAudience,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<MedicalAudienceType as Deserialize>::deserialize(deserializer),
				MedicalAudienceProperty::MedicalAudienceType,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				MedicalAudienceProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property medicalAudience or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property medicalAudience";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
