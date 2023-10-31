use super::*;
/// <https://schema.org/memberOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum MemberOfProperty {
	#[cfg(any(
		any(feature = "organization-schema", feature = "general-schema-section"),
		doc
	))]
	Organization(Organization),
	#[cfg(any(
		any(
			feature = "program-membership-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	ProgramMembership(ProgramMembership),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for MemberOfProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "organization-schema", feature = "general-schema-section"),
					doc
				))]
				MemberOfProperty::Organization(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "program-membership-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MemberOfProperty::ProgramMembership(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for MemberOfProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "organization-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Organization as Deserialize>::deserialize(deserializer),
				MemberOfProperty::Organization,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "program-membership-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<ProgramMembership as Deserialize>::deserialize(deserializer),
				MemberOfProperty::ProgramMembership,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property memberOf",
			))
		}
	}
}
