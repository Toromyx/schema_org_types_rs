use super::*;
/// <https://schema.org/diseaseSpreadStatistics>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DiseaseSpreadStatisticsProperty {
	#[cfg(any(
		any(feature = "dataset-schema", feature = "general-schema-section"),
		doc
	))]
	Dataset(Dataset),
	#[cfg(any(
		any(feature = "observation-schema", feature = "pending-schema-section"),
		doc
	))]
	Observation(Observation),
	#[cfg(any(
		any(feature = "web-content-schema", feature = "pending-schema-section"),
		doc
	))]
	WebContent(WebContent),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
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
	impl Serialize for DiseaseSpreadStatisticsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "dataset-schema", feature = "general-schema-section"),
					doc
				))]
				DiseaseSpreadStatisticsProperty::Dataset(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "observation-schema", feature = "pending-schema-section"),
					doc
				))]
				DiseaseSpreadStatisticsProperty::Observation(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "web-content-schema", feature = "pending-schema-section"),
					doc
				))]
				DiseaseSpreadStatisticsProperty::WebContent(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				DiseaseSpreadStatisticsProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				DiseaseSpreadStatisticsProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for DiseaseSpreadStatisticsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "dataset-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Dataset as Deserialize>::deserialize(deserializer),
				DiseaseSpreadStatisticsProperty::Dataset,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "observation-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Observation as Deserialize>::deserialize(deserializer),
				DiseaseSpreadStatisticsProperty::Observation,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "web-content-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<WebContent as Deserialize>::deserialize(deserializer),
				DiseaseSpreadStatisticsProperty::WebContent,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				DiseaseSpreadStatisticsProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				DiseaseSpreadStatisticsProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property diseaseSpreadStatistics or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property diseaseSpreadStatistics";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
