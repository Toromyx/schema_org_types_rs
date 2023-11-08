use super::*;
/// <https://schema.org/associatedAnatomy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AssociatedAnatomyProperty {
	AnatomicalStructure(AnatomicalStructure),
	AnatomicalSystem(AnatomicalSystem),
	SuperficialAnatomy(SuperficialAnatomy),
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
	impl Serialize for AssociatedAnatomyProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				AssociatedAnatomyProperty::AnatomicalStructure(ref inner) => {
					inner.serialize(serializer)
				}
				AssociatedAnatomyProperty::AnatomicalSystem(ref inner) => {
					inner.serialize(serializer)
				}
				AssociatedAnatomyProperty::SuperficialAnatomy(ref inner) => {
					inner.serialize(serializer)
				}
				#[cfg(all(feature = "fallible", feature = "serde"))]
				AssociatedAnatomyProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AssociatedAnatomyProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<AnatomicalStructure as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::AnatomicalStructure,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<AnatomicalSystem as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::AnatomicalSystem,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<SuperficialAnatomy as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::SuperficialAnatomy,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				AssociatedAnatomyProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property associatedAnatomy or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property associatedAnatomy";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
