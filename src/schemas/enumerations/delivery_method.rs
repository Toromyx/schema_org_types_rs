/// <https://schema.org/DeliveryMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum DeliveryMethod {
	/// <https://schema.org/LockerDelivery>
	LockerDelivery,
	/// <https://schema.org/OnSitePickup>
	OnSitePickup,
	/// <https://schema.org/ParcelService>
	ParcelService,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for DeliveryMethod {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				DeliveryMethod::LockerDelivery => {
					serializer.serialize_unit_variant("DeliveryMethod", 0u32, "LockerDelivery")
				}
				DeliveryMethod::OnSitePickup => {
					serializer.serialize_unit_variant("DeliveryMethod", 1u32, "OnSitePickup")
				}
				DeliveryMethod::ParcelService => {
					serializer.serialize_unit_variant("DeliveryMethod", 2u32, "ParcelService")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for DeliveryMethod {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				LockerDelivery,
				OnSitePickup,
				ParcelService,
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
						"LockerDelivery" => Ok(Field::LockerDelivery),
						"OnSitePickup" => Ok(Field::OnSitePickup),
						"ParcelService" => Ok(Field::ParcelService),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"LockerDelivery" => Ok(Field::LockerDelivery),
						b"OnSitePickup" => Ok(Field::OnSitePickup),
						b"ParcelService" => Ok(Field::ParcelService),
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
				type Value = DeliveryMethod;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DeliveryMethod")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::LockerDelivery, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeliveryMethod::LockerDelivery)
						}
						(Field::OnSitePickup, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeliveryMethod::OnSitePickup)
						}
						(Field::ParcelService, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(DeliveryMethod::ParcelService)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &["LockerDelivery", "OnSitePickup", "ParcelService"];
			deserializer.deserialize_enum("DeliveryMethod", VARIANTS, EnumerationVisitor)
		}
	}
}
