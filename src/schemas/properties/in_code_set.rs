use super::*;
/// <https://schema.org/inCodeSet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum InCodeSetProperty {
	#[cfg(any(
		any(
			feature = "category-code-set-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	CategoryCodeSet(CategoryCodeSet),
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
	impl Serialize for InCodeSetProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "category-code-set-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InCodeSetProperty::CategoryCodeSet(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				InCodeSetProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				InCodeSetProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for InCodeSetProperty {
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
					feature = "category-code-set-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CategoryCodeSet as Deserialize>::deserialize(deserializer),
				InCodeSetProperty::CategoryCodeSet,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				InCodeSetProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				InCodeSetProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property inCodeSet or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property inCodeSet";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
