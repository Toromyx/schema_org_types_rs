use super::*;
/// <https://schema.org/fundedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum FundedItemProperty {
	#[cfg(any(
		any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
		doc
	))]
	BioChemEntity(BioChemEntity),
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
	Event(Event),
	#[cfg(any(
		any(
			feature = "medical-entity-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	MedicalEntity(MedicalEntity),
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
	#[cfg(any(
		any(feature = "product-schema", feature = "general-schema-section"),
		doc
	))]
	Product(Product),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for FundedItemProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
					doc
				))]
				FundedItemProperty::BioChemEntity(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "creative-work-schema", feature = "general-schema-section"),
					doc
				))]
				FundedItemProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
				FundedItemProperty::Event(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "medical-entity-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				FundedItemProperty::MedicalEntity(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "organization-schema", feature = "general-schema-section"),
					doc
				))]
				FundedItemProperty::Organization(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "person-schema", feature = "general-schema-section"),
					doc
				))]
				FundedItemProperty::Person(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "product-schema", feature = "general-schema-section"),
					doc
				))]
				FundedItemProperty::Product(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for FundedItemProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "bio-chem-entity-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<BioChemEntity as Deserialize>::deserialize(deserializer),
				FundedItemProperty::BioChemEntity,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "creative-work-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				FundedItemProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "event-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Event as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Event,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "medical-entity-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<MedicalEntity as Deserialize>::deserialize(deserializer),
				FundedItemProperty::MedicalEntity,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "organization-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Organization as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Organization,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "person-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Person as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Person,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "product-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Product,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property fundedItem",
			))
		}
	}
}
