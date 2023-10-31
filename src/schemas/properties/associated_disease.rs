use super::*;
/// <https://schema.org/associatedDisease>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AssociatedDiseaseProperty {
	#[cfg(any(
		any(
			feature = "medical-condition-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalCondition(MedicalCondition),
	#[cfg(any(
		any(feature = "property-value-schema", feature = "general-schema-section"),
		doc
	))]
	PropertyValue(PropertyValue),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AssociatedDiseaseProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "medical-condition-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				AssociatedDiseaseProperty::MedicalCondition(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "property-value-schema", feature = "general-schema-section"),
					doc
				))]
				AssociatedDiseaseProperty::PropertyValue(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				AssociatedDiseaseProperty::Url(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AssociatedDiseaseProperty {
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
					feature = "medical-condition-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalCondition as Deserialize>::deserialize(deserializer),
				AssociatedDiseaseProperty::MedicalCondition,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "property-value-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PropertyValue as Deserialize>::deserialize(deserializer),
				AssociatedDiseaseProperty::PropertyValue,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				AssociatedDiseaseProperty::Url,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property associatedDisease",
			))
		}
	}
}
