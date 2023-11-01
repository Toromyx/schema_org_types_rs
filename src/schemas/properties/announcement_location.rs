use super::*;
/// <https://schema.org/announcementLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AnnouncementLocationProperty {
	#[cfg(any(
		any(feature = "civic-structure-schema", feature = "general-schema-section"),
		doc
	))]
	CivicStructure(CivicStructure),
	#[cfg(any(
		any(feature = "local-business-schema", feature = "general-schema-section"),
		doc
	))]
	LocalBusiness(LocalBusiness),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AnnouncementLocationProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "civic-structure-schema", feature = "general-schema-section"),
					doc
				))]
				AnnouncementLocationProperty::CivicStructure(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "local-business-schema", feature = "general-schema-section"),
					doc
				))]
				AnnouncementLocationProperty::LocalBusiness(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AnnouncementLocationProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "civic-structure-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CivicStructure as Deserialize>::deserialize(deserializer),
				AnnouncementLocationProperty::CivicStructure,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "local-business-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<LocalBusiness as Deserialize>::deserialize(deserializer),
				AnnouncementLocationProperty::LocalBusiness,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property announcementLocation",
			))
		}
	}
}
