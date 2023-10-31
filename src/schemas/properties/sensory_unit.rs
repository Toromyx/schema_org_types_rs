use super::*;
/// <https://schema.org/sensoryUnit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum SensoryUnitProperty {
	#[cfg(any(
		any(
			feature = "anatomical-structure-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalStructure(AnatomicalStructure),
	#[cfg(any(
		any(
			feature = "superficial-anatomy-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	SuperficialAnatomy(SuperficialAnatomy),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for SensoryUnitProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "anatomical-structure-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SensoryUnitProperty::AnatomicalStructure(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "superficial-anatomy-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				SensoryUnitProperty::SuperficialAnatomy(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for SensoryUnitProperty {
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
					feature = "anatomical-structure-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AnatomicalStructure as Deserialize>::deserialize(deserializer),
				SensoryUnitProperty::AnatomicalStructure,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "superficial-anatomy-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<SuperficialAnatomy as Deserialize>::deserialize(deserializer),
				SensoryUnitProperty::SuperficialAnatomy,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property sensoryUnit",
			))
		}
	}
}
