/// <https://schema.org/OrderStatus>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum OrderStatus {
	/// <https://schema.org/OrderCancelled>
	OrderCancelled,
	/// <https://schema.org/OrderDelivered>
	OrderDelivered,
	/// <https://schema.org/OrderInTransit>
	OrderInTransit,
	/// <https://schema.org/OrderPaymentDue>
	OrderPaymentDue,
	/// <https://schema.org/OrderPickupAvailable>
	OrderPickupAvailable,
	/// <https://schema.org/OrderProblem>
	OrderProblem,
	/// <https://schema.org/OrderProcessing>
	OrderProcessing,
	/// <https://schema.org/OrderReturned>
	OrderReturned,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for OrderStatus {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				OrderStatus::OrderCancelled => {
					serializer.serialize_unit_variant("OrderStatus", 0u32, "OrderCancelled")
				}
				OrderStatus::OrderDelivered => {
					serializer.serialize_unit_variant("OrderStatus", 1u32, "OrderDelivered")
				}
				OrderStatus::OrderInTransit => {
					serializer.serialize_unit_variant("OrderStatus", 2u32, "OrderInTransit")
				}
				OrderStatus::OrderPaymentDue => {
					serializer.serialize_unit_variant("OrderStatus", 3u32, "OrderPaymentDue")
				}
				OrderStatus::OrderPickupAvailable => {
					serializer.serialize_unit_variant("OrderStatus", 4u32, "OrderPickupAvailable")
				}
				OrderStatus::OrderProblem => {
					serializer.serialize_unit_variant("OrderStatus", 5u32, "OrderProblem")
				}
				OrderStatus::OrderProcessing => {
					serializer.serialize_unit_variant("OrderStatus", 6u32, "OrderProcessing")
				}
				OrderStatus::OrderReturned => {
					serializer.serialize_unit_variant("OrderStatus", 7u32, "OrderReturned")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for OrderStatus {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				OrderCancelled,
				OrderDelivered,
				OrderInTransit,
				OrderPaymentDue,
				OrderPickupAvailable,
				OrderProblem,
				OrderProcessing,
				OrderReturned,
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
						"OrderCancelled" => Ok(Field::OrderCancelled),
						"OrderDelivered" => Ok(Field::OrderDelivered),
						"OrderInTransit" => Ok(Field::OrderInTransit),
						"OrderPaymentDue" => Ok(Field::OrderPaymentDue),
						"OrderPickupAvailable" => Ok(Field::OrderPickupAvailable),
						"OrderProblem" => Ok(Field::OrderProblem),
						"OrderProcessing" => Ok(Field::OrderProcessing),
						"OrderReturned" => Ok(Field::OrderReturned),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"OrderCancelled" => Ok(Field::OrderCancelled),
						b"OrderDelivered" => Ok(Field::OrderDelivered),
						b"OrderInTransit" => Ok(Field::OrderInTransit),
						b"OrderPaymentDue" => Ok(Field::OrderPaymentDue),
						b"OrderPickupAvailable" => Ok(Field::OrderPickupAvailable),
						b"OrderProblem" => Ok(Field::OrderProblem),
						b"OrderProcessing" => Ok(Field::OrderProcessing),
						b"OrderReturned" => Ok(Field::OrderReturned),
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
				type Value = OrderStatus;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema OrderStatus")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::OrderCancelled, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderCancelled)
						}
						(Field::OrderDelivered, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderDelivered)
						}
						(Field::OrderInTransit, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderInTransit)
						}
						(Field::OrderPaymentDue, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderPaymentDue)
						}
						(Field::OrderPickupAvailable, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderPickupAvailable)
						}
						(Field::OrderProblem, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderProblem)
						}
						(Field::OrderProcessing, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderProcessing)
						}
						(Field::OrderReturned, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(OrderStatus::OrderReturned)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"OrderCancelled",
				"OrderDelivered",
				"OrderInTransit",
				"OrderPaymentDue",
				"OrderPickupAvailable",
				"OrderProblem",
				"OrderProcessing",
				"OrderReturned",
			];
			deserializer.deserialize_enum("OrderStatus", VARIANTS, EnumerationVisitor)
		}
	}
}
