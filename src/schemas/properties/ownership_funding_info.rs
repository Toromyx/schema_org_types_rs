use super::*;
/// <https://schema.org/ownershipFundingInfo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OwnershipFundingInfoProperty {
	#[cfg(any(
		any(feature = "about-page-schema", feature = "general-schema-section"),
		doc
	))]
	AboutPage(AboutPage),
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
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
	impl Serialize for OwnershipFundingInfoProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "about-page-schema", feature = "general-schema-section"),
					doc
				))]
				OwnershipFundingInfoProperty::AboutPage(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "creative-work-schema", feature = "general-schema-section"),
					doc
				))]
				OwnershipFundingInfoProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				OwnershipFundingInfoProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				OwnershipFundingInfoProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				OwnershipFundingInfoProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for OwnershipFundingInfoProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "about-page-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AboutPage as Deserialize>::deserialize(deserializer),
				OwnershipFundingInfoProperty::AboutPage,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "creative-work-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				OwnershipFundingInfoProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				OwnershipFundingInfoProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				OwnershipFundingInfoProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				OwnershipFundingInfoProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property ownershipFundingInfo or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property ownershipFundingInfo";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
