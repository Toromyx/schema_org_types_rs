use super::*;
/// <https://schema.org/usageInfo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum UsageInfoProperty {
	CreativeWork(CreativeWork),
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
	impl Serialize for UsageInfoProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				UsageInfoProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				UsageInfoProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				UsageInfoProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for UsageInfoProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				UsageInfoProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				UsageInfoProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				UsageInfoProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property usageInfo or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property usageInfo";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
