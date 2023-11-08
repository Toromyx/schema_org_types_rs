use super::*;
/// <https://schema.org/signOrSymptom>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SignOrSymptomProperty {
	#[cfg(any(
		any(
			feature = "medical-sign-or-symptom-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalSignOrSymptom(MedicalSignOrSymptom),
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
	impl Serialize for SignOrSymptomProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "medical-sign-or-symptom-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SignOrSymptomProperty::MedicalSignOrSymptom(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				SignOrSymptomProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SignOrSymptomProperty {
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
					feature = "medical-sign-or-symptom-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalSignOrSymptom as Deserialize>::deserialize(deserializer),
				SignOrSymptomProperty::MedicalSignOrSymptom,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				SignOrSymptomProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property signOrSymptom or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property signOrSymptom";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
