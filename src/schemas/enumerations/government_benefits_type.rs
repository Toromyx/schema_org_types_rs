/// <https://schema.org/GovernmentBenefitsType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum GovernmentBenefitsType {
	/// <https://schema.org/BasicIncome>
	BasicIncome,
	/// <https://schema.org/BusinessSupport>
	BusinessSupport,
	/// <https://schema.org/DisabilitySupport>
	DisabilitySupport,
	/// <https://schema.org/HealthCare>
	HealthCare,
	/// <https://schema.org/OneTimePayments>
	OneTimePayments,
	/// <https://schema.org/PaidLeave>
	PaidLeave,
	/// <https://schema.org/ParentalSupport>
	ParentalSupport,
	/// <https://schema.org/UnemploymentSupport>
	UnemploymentSupport,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for GovernmentBenefitsType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				GovernmentBenefitsType::BasicIncome => {
					serializer.serialize_unit_variant("GovernmentBenefitsType", 0u32, "BasicIncome")
				}
				GovernmentBenefitsType::BusinessSupport => serializer.serialize_unit_variant(
					"GovernmentBenefitsType",
					1u32,
					"BusinessSupport",
				),
				GovernmentBenefitsType::DisabilitySupport => serializer.serialize_unit_variant(
					"GovernmentBenefitsType",
					2u32,
					"DisabilitySupport",
				),
				GovernmentBenefitsType::HealthCare => {
					serializer.serialize_unit_variant("GovernmentBenefitsType", 3u32, "HealthCare")
				}
				GovernmentBenefitsType::OneTimePayments => serializer.serialize_unit_variant(
					"GovernmentBenefitsType",
					4u32,
					"OneTimePayments",
				),
				GovernmentBenefitsType::PaidLeave => {
					serializer.serialize_unit_variant("GovernmentBenefitsType", 5u32, "PaidLeave")
				}
				GovernmentBenefitsType::ParentalSupport => serializer.serialize_unit_variant(
					"GovernmentBenefitsType",
					6u32,
					"ParentalSupport",
				),
				GovernmentBenefitsType::UnemploymentSupport => serializer.serialize_unit_variant(
					"GovernmentBenefitsType",
					7u32,
					"UnemploymentSupport",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for GovernmentBenefitsType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				BasicIncome,
				BusinessSupport,
				DisabilitySupport,
				HealthCare,
				OneTimePayments,
				PaidLeave,
				ParentalSupport,
				UnemploymentSupport,
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
						"BasicIncome" => Ok(Field::BasicIncome),
						"BusinessSupport" => Ok(Field::BusinessSupport),
						"DisabilitySupport" => Ok(Field::DisabilitySupport),
						"HealthCare" => Ok(Field::HealthCare),
						"OneTimePayments" => Ok(Field::OneTimePayments),
						"PaidLeave" => Ok(Field::PaidLeave),
						"ParentalSupport" => Ok(Field::ParentalSupport),
						"UnemploymentSupport" => Ok(Field::UnemploymentSupport),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"BasicIncome" => Ok(Field::BasicIncome),
						b"BusinessSupport" => Ok(Field::BusinessSupport),
						b"DisabilitySupport" => Ok(Field::DisabilitySupport),
						b"HealthCare" => Ok(Field::HealthCare),
						b"OneTimePayments" => Ok(Field::OneTimePayments),
						b"PaidLeave" => Ok(Field::PaidLeave),
						b"ParentalSupport" => Ok(Field::ParentalSupport),
						b"UnemploymentSupport" => Ok(Field::UnemploymentSupport),
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
				type Value = GovernmentBenefitsType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema GovernmentBenefitsType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::BasicIncome, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::BasicIncome)
						}
						(Field::BusinessSupport, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::BusinessSupport)
						}
						(Field::DisabilitySupport, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::DisabilitySupport)
						}
						(Field::HealthCare, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::HealthCare)
						}
						(Field::OneTimePayments, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::OneTimePayments)
						}
						(Field::PaidLeave, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::PaidLeave)
						}
						(Field::ParentalSupport, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::ParentalSupport)
						}
						(Field::UnemploymentSupport, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(GovernmentBenefitsType::UnemploymentSupport)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"BasicIncome",
				"BusinessSupport",
				"DisabilitySupport",
				"HealthCare",
				"OneTimePayments",
				"PaidLeave",
				"ParentalSupport",
				"UnemploymentSupport",
			];
			deserializer.deserialize_enum("GovernmentBenefitsType", VARIANTS, EnumerationVisitor)
		}
	}
}
