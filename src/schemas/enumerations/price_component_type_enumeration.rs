/// <https://schema.org/PriceComponentTypeEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum PriceComponentTypeEnumeration {
	/// <https://schema.org/ActivationFee>
	ActivationFee,
	/// <https://schema.org/CleaningFee>
	CleaningFee,
	/// <https://schema.org/DistanceFee>
	DistanceFee,
	/// <https://schema.org/Downpayment>
	Downpayment,
	/// <https://schema.org/Installment>
	Installment,
	/// <https://schema.org/Subscription>
	Subscription,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for PriceComponentTypeEnumeration {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				PriceComponentTypeEnumeration::ActivationFee => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					0u32,
					"ActivationFee",
				),
				PriceComponentTypeEnumeration::CleaningFee => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					1u32,
					"CleaningFee",
				),
				PriceComponentTypeEnumeration::DistanceFee => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					2u32,
					"DistanceFee",
				),
				PriceComponentTypeEnumeration::Downpayment => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					3u32,
					"Downpayment",
				),
				PriceComponentTypeEnumeration::Installment => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					4u32,
					"Installment",
				),
				PriceComponentTypeEnumeration::Subscription => serializer.serialize_unit_variant(
					"PriceComponentTypeEnumeration",
					5u32,
					"Subscription",
				),
			}
		}
	}
	impl<'de> Deserialize<'de> for PriceComponentTypeEnumeration {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				ActivationFee,
				CleaningFee,
				DistanceFee,
				Downpayment,
				Installment,
				Subscription,
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
						"ActivationFee" => Ok(Field::ActivationFee),
						"CleaningFee" => Ok(Field::CleaningFee),
						"DistanceFee" => Ok(Field::DistanceFee),
						"Downpayment" => Ok(Field::Downpayment),
						"Installment" => Ok(Field::Installment),
						"Subscription" => Ok(Field::Subscription),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"ActivationFee" => Ok(Field::ActivationFee),
						b"CleaningFee" => Ok(Field::CleaningFee),
						b"DistanceFee" => Ok(Field::DistanceFee),
						b"Downpayment" => Ok(Field::Downpayment),
						b"Installment" => Ok(Field::Installment),
						b"Subscription" => Ok(Field::Subscription),
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
				type Value = PriceComponentTypeEnumeration;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PriceComponentTypeEnumeration")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::ActivationFee, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::ActivationFee)
						}
						(Field::CleaningFee, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::CleaningFee)
						}
						(Field::DistanceFee, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::DistanceFee)
						}
						(Field::Downpayment, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::Downpayment)
						}
						(Field::Installment, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::Installment)
						}
						(Field::Subscription, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(PriceComponentTypeEnumeration::Subscription)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"ActivationFee",
				"CleaningFee",
				"DistanceFee",
				"Downpayment",
				"Installment",
				"Subscription",
			];
			deserializer.deserialize_enum(
				"PriceComponentTypeEnumeration",
				VARIANTS,
				EnumerationVisitor,
			)
		}
	}
}
