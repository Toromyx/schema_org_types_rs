use super::*;
/// <https://schema.org/grantee>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GranteeProperty {
	#[cfg(any(
		any(feature = "audience-schema", feature = "general-schema-section"),
		doc
	))]
	Audience(Audience),
	#[cfg(any(
		any(feature = "contact-point-schema", feature = "general-schema-section"),
		doc
	))]
	ContactPoint(ContactPoint),
	#[cfg(any(
		any(feature = "organization-schema", feature = "general-schema-section"),
		doc
	))]
	Organization(Organization),
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
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
	impl Serialize for GranteeProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "audience-schema", feature = "general-schema-section"),
					doc
				))]
				GranteeProperty::Audience(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "contact-point-schema", feature = "general-schema-section"),
					doc
				))]
				GranteeProperty::ContactPoint(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "organization-schema", feature = "general-schema-section"),
					doc
				))]
				GranteeProperty::Organization(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "person-schema", feature = "general-schema-section"),
					doc
				))]
				GranteeProperty::Person(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				GranteeProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for GranteeProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "audience-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Audience as Deserialize>::deserialize(deserializer),
				GranteeProperty::Audience,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "contact-point-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ContactPoint as Deserialize>::deserialize(deserializer),
				GranteeProperty::ContactPoint,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "organization-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Organization as Deserialize>::deserialize(deserializer),
				GranteeProperty::Organization,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "person-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Person as Deserialize>::deserialize(deserializer),
				GranteeProperty::Person,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				GranteeProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property grantee or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property grantee";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
