use super::*;
/// <https://schema.org/homeTeam>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum HomeTeamProperty {
	#[cfg(any(
		any(feature = "person-schema", feature = "general-schema-section"),
		doc
	))]
	Person(Person),
	#[cfg(any(
		any(feature = "sports-team-schema", feature = "general-schema-section"),
		doc
	))]
	SportsTeam(SportsTeam),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for HomeTeamProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "person-schema", feature = "general-schema-section"),
					doc
				))]
				HomeTeamProperty::Person(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "sports-team-schema", feature = "general-schema-section"),
					doc
				))]
				HomeTeamProperty::SportsTeam(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for HomeTeamProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "person-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Person as Deserialize>::deserialize(deserializer),
				HomeTeamProperty::Person,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "sports-team-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<SportsTeam as Deserialize>::deserialize(deserializer),
				HomeTeamProperty::SportsTeam,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property homeTeam",
			))
		}
	}
}
