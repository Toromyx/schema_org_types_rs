/// <https://schema.org/UKNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum UkNonprofitType {
	/// <https://schema.org/CharitableIncorporatedOrganization>
	CharitableIncorporatedOrganization,
	/// <https://schema.org/LimitedByGuaranteeCharity>
	LimitedByGuaranteeCharity,
	/// <https://schema.org/UKTrust>
	UkTrust,
	/// <https://schema.org/UnincorporatedAssociationCharity>
	UnincorporatedAssociationCharity,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for UkNonprofitType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				UkNonprofitType::CharitableIncorporatedOrganization => serializer
					.serialize_unit_variant(
						"UkNonprofitType",
						0u32,
						"CharitableIncorporatedOrganization",
					),
				UkNonprofitType::LimitedByGuaranteeCharity => serializer.serialize_unit_variant(
					"UkNonprofitType",
					1u32,
					"LimitedByGuaranteeCharity",
				),
				UkNonprofitType::UkTrust => {
					serializer.serialize_unit_variant("UkNonprofitType", 2u32, "UkTrust")
				}
				UkNonprofitType::UnincorporatedAssociationCharity => serializer
					.serialize_unit_variant(
						"UkNonprofitType",
						3u32,
						"UnincorporatedAssociationCharity",
					),
			}
		}
	}
	impl<'de> Deserialize<'de> for UkNonprofitType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CharitableIncorporatedOrganization,
				LimitedByGuaranteeCharity,
				UkTrust,
				UnincorporatedAssociationCharity,
			}
			struct FieldVisitor;
			impl<'de> de::Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("variant identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						"CharitableIncorporatedOrganization" => {
							Ok(Field::CharitableIncorporatedOrganization)
						}
						"LimitedByGuaranteeCharity" => Ok(Field::LimitedByGuaranteeCharity),
						"UkTrust" => Ok(Field::UkTrust),
						"UnincorporatedAssociationCharity" => {
							Ok(Field::UnincorporatedAssociationCharity)
						}
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"CharitableIncorporatedOrganization" => {
							Ok(Field::CharitableIncorporatedOrganization)
						}
						b"LimitedByGuaranteeCharity" => Ok(Field::LimitedByGuaranteeCharity),
						b"UkTrust" => Ok(Field::UkTrust),
						b"UnincorporatedAssociationCharity" => {
							Ok(Field::UnincorporatedAssociationCharity)
						}
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_variant(value, VARIANTS))
						}
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct EnumerationVisitor;
			impl<'de> Visitor<'de> for EnumerationVisitor {
				type Value = UkNonprofitType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema UKNonprofitType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::CharitableIncorporatedOrganization, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UkNonprofitType::CharitableIncorporatedOrganization)
						}
						(Field::LimitedByGuaranteeCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UkNonprofitType::LimitedByGuaranteeCharity)
						}
						(Field::UkTrust, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UkNonprofitType::UkTrust)
						}
						(Field::UnincorporatedAssociationCharity, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UkNonprofitType::UnincorporatedAssociationCharity)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"CharitableIncorporatedOrganization",
				"LimitedByGuaranteeCharity",
				"UkTrust",
				"UnincorporatedAssociationCharity",
			];
			deserializer.deserialize_enum("UkNonprofitType", VARIANTS, EnumerationVisitor)
		}
	}
}
