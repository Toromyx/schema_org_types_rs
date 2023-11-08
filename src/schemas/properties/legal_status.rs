use super::*;
/// <https://schema.org/legalStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LegalStatusProperty {
	#[cfg(any(
		any(
			feature = "drug-legal-status-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	DrugLegalStatus(DrugLegalStatus),
	#[cfg(any(
		any(
			feature = "medical-enumeration-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalEnumeration(MedicalEnumeration),
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
	impl Serialize for LegalStatusProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "drug-legal-status-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				LegalStatusProperty::DrugLegalStatus(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-enumeration-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				LegalStatusProperty::MedicalEnumeration(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				LegalStatusProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				LegalStatusProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for LegalStatusProperty {
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
					feature = "drug-legal-status-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DrugLegalStatus as Deserialize>::deserialize(deserializer),
				LegalStatusProperty::DrugLegalStatus,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-enumeration-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalEnumeration as Deserialize>::deserialize(deserializer),
				LegalStatusProperty::MedicalEnumeration,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				LegalStatusProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				LegalStatusProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property legalStatus or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property legalStatus";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
