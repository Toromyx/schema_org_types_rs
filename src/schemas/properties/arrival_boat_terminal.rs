use super::*;
/// <https://schema.org/arrivalBoatTerminal>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ArrivalBoatTerminalProperty {
	#[cfg(any(
		any(feature = "boat-terminal-schema", feature = "pending-schema-section"),
		doc
	))]
	BoatTerminal(BoatTerminal),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for ArrivalBoatTerminalProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "boat-terminal-schema", feature = "pending-schema-section"),
					doc
				))]
				ArrivalBoatTerminalProperty::BoatTerminal(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ArrivalBoatTerminalProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "boat-terminal-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<BoatTerminal as Deserialize>::deserialize(deserializer),
				ArrivalBoatTerminalProperty::BoatTerminal,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property arrivalBoatTerminal",
			))
		}
	}
}
