use super::*;
/// <https://schema.org/fundedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum FundedItemProperty {
	BioChemEntity(BioChemEntity),
	CreativeWork(CreativeWork),
	Event(Event),
	MedicalEntity(MedicalEntity),
	Organization(Organization),
	Person(Person),
	Product(Product),
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
	impl Serialize for FundedItemProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				FundedItemProperty::BioChemEntity(ref inner) => inner.serialize(serializer),
				FundedItemProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				FundedItemProperty::Event(ref inner) => inner.serialize(serializer),
				FundedItemProperty::MedicalEntity(ref inner) => inner.serialize(serializer),
				FundedItemProperty::Organization(ref inner) => inner.serialize(serializer),
				FundedItemProperty::Person(ref inner) => inner.serialize(serializer),
				FundedItemProperty::Product(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				FundedItemProperty::SerdeFail(ref inner) => inner.serialize(serializer),
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
			if let Ok(ok) = Result::map(
				<BioChemEntity as Deserialize>::deserialize(deserializer),
				FundedItemProperty::BioChemEntity,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				FundedItemProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Event as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Event,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<MedicalEntity as Deserialize>::deserialize(deserializer),
				FundedItemProperty::MedicalEntity,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Organization as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Organization,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Person as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Person,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Product as Deserialize>::deserialize(deserializer),
				FundedItemProperty::Product,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				FundedItemProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property fundedItem or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property fundedItem";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
