use super::*;
/// <https://schema.org/expressedIn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum ExpressedInProperty {
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
			feature = "anatomical-system-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	AnatomicalSystem(AnatomicalSystem),
	#[cfg(any(
		any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
		doc
	))]
	BioChemEntity(BioChemEntity),
	#[cfg(any(
		any(feature = "defined-term-schema", feature = "pending-schema-section"),
		doc
	))]
	DefinedTerm(DefinedTerm),
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
	impl Serialize for ExpressedInProperty {
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
				ExpressedInProperty::AnatomicalStructure(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "anatomical-system-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				ExpressedInProperty::AnatomicalSystem(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
					doc
				))]
				ExpressedInProperty::BioChemEntity(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "defined-term-schema", feature = "pending-schema-section"),
					doc
				))]
				ExpressedInProperty::DefinedTerm(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				ExpressedInProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for ExpressedInProperty {
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
				ExpressedInProperty::AnatomicalStructure,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "anatomical-system-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<AnatomicalSystem as Deserialize>::deserialize(deserializer),
				ExpressedInProperty::AnatomicalSystem,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<BioChemEntity as Deserialize>::deserialize(deserializer),
				ExpressedInProperty::BioChemEntity,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "defined-term-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<DefinedTerm as Deserialize>::deserialize(deserializer),
				ExpressedInProperty::DefinedTerm,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				ExpressedInProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property expressedIn or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property expressedIn";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
