use super::*;
/// <https://schema.org/healthcareReportingData>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum HealthcareReportingDataProperty {
	#[cfg(any(
		any(feature = "cdcpmd-record-schema", feature = "pending-schema-section"),
		doc
	))]
	CdcpmdRecord(CdcpmdRecord),
	#[cfg(any(
		any(feature = "dataset-schema", feature = "general-schema-section"),
		doc
	))]
	Dataset(Dataset),
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
	impl Serialize for HealthcareReportingDataProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "cdcpmd-record-schema", feature = "pending-schema-section"),
					doc
				))]
				HealthcareReportingDataProperty::CdcpmdRecord(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "dataset-schema", feature = "general-schema-section"),
					doc
				))]
				HealthcareReportingDataProperty::Dataset(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				HealthcareReportingDataProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for HealthcareReportingDataProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "cdcpmd-record-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CdcpmdRecord as Deserialize>::deserialize(deserializer),
				HealthcareReportingDataProperty::CdcpmdRecord,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "dataset-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Dataset as Deserialize>::deserialize(deserializer),
				HealthcareReportingDataProperty::Dataset,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				HealthcareReportingDataProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property healthcareReportingData or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property healthcareReportingData";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
