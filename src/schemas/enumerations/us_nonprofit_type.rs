/// <https://schema.org/USNonprofitType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum UsNonprofitType {
	/// <https://schema.org/Nonprofit501a>
	Nonprofit501A,
	/// <https://schema.org/Nonprofit501c1>
	Nonprofit501C1,
	/// <https://schema.org/Nonprofit501c10>
	Nonprofit501C10,
	/// <https://schema.org/Nonprofit501c11>
	Nonprofit501C11,
	/// <https://schema.org/Nonprofit501c12>
	Nonprofit501C12,
	/// <https://schema.org/Nonprofit501c13>
	Nonprofit501C13,
	/// <https://schema.org/Nonprofit501c14>
	Nonprofit501C14,
	/// <https://schema.org/Nonprofit501c15>
	Nonprofit501C15,
	/// <https://schema.org/Nonprofit501c16>
	Nonprofit501C16,
	/// <https://schema.org/Nonprofit501c17>
	Nonprofit501C17,
	/// <https://schema.org/Nonprofit501c18>
	Nonprofit501C18,
	/// <https://schema.org/Nonprofit501c19>
	Nonprofit501C19,
	/// <https://schema.org/Nonprofit501c2>
	Nonprofit501C2,
	/// <https://schema.org/Nonprofit501c20>
	Nonprofit501C20,
	/// <https://schema.org/Nonprofit501c21>
	Nonprofit501C21,
	/// <https://schema.org/Nonprofit501c22>
	Nonprofit501C22,
	/// <https://schema.org/Nonprofit501c23>
	Nonprofit501C23,
	/// <https://schema.org/Nonprofit501c24>
	Nonprofit501C24,
	/// <https://schema.org/Nonprofit501c25>
	Nonprofit501C25,
	/// <https://schema.org/Nonprofit501c26>
	Nonprofit501C26,
	/// <https://schema.org/Nonprofit501c27>
	Nonprofit501C27,
	/// <https://schema.org/Nonprofit501c28>
	Nonprofit501C28,
	/// <https://schema.org/Nonprofit501c3>
	Nonprofit501C3,
	/// <https://schema.org/Nonprofit501c4>
	Nonprofit501C4,
	/// <https://schema.org/Nonprofit501c5>
	Nonprofit501C5,
	/// <https://schema.org/Nonprofit501c6>
	Nonprofit501C6,
	/// <https://schema.org/Nonprofit501c7>
	Nonprofit501C7,
	/// <https://schema.org/Nonprofit501c8>
	Nonprofit501C8,
	/// <https://schema.org/Nonprofit501c9>
	Nonprofit501C9,
	/// <https://schema.org/Nonprofit501d>
	Nonprofit501D,
	/// <https://schema.org/Nonprofit501e>
	Nonprofit501E,
	/// <https://schema.org/Nonprofit501f>
	Nonprofit501F,
	/// <https://schema.org/Nonprofit501k>
	Nonprofit501K,
	/// <https://schema.org/Nonprofit501n>
	Nonprofit501N,
	/// <https://schema.org/Nonprofit501q>
	Nonprofit501Q,
	/// <https://schema.org/Nonprofit527>
	Nonprofit527,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for UsNonprofitType {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				UsNonprofitType::Nonprofit501A => {
					serializer.serialize_unit_variant("UsNonprofitType", 0u32, "Nonprofit501A")
				}
				UsNonprofitType::Nonprofit501C1 => {
					serializer.serialize_unit_variant("UsNonprofitType", 1u32, "Nonprofit501C1")
				}
				UsNonprofitType::Nonprofit501C10 => {
					serializer.serialize_unit_variant("UsNonprofitType", 2u32, "Nonprofit501C10")
				}
				UsNonprofitType::Nonprofit501C11 => {
					serializer.serialize_unit_variant("UsNonprofitType", 3u32, "Nonprofit501C11")
				}
				UsNonprofitType::Nonprofit501C12 => {
					serializer.serialize_unit_variant("UsNonprofitType", 4u32, "Nonprofit501C12")
				}
				UsNonprofitType::Nonprofit501C13 => {
					serializer.serialize_unit_variant("UsNonprofitType", 5u32, "Nonprofit501C13")
				}
				UsNonprofitType::Nonprofit501C14 => {
					serializer.serialize_unit_variant("UsNonprofitType", 6u32, "Nonprofit501C14")
				}
				UsNonprofitType::Nonprofit501C15 => {
					serializer.serialize_unit_variant("UsNonprofitType", 7u32, "Nonprofit501C15")
				}
				UsNonprofitType::Nonprofit501C16 => {
					serializer.serialize_unit_variant("UsNonprofitType", 8u32, "Nonprofit501C16")
				}
				UsNonprofitType::Nonprofit501C17 => {
					serializer.serialize_unit_variant("UsNonprofitType", 9u32, "Nonprofit501C17")
				}
				UsNonprofitType::Nonprofit501C18 => {
					serializer.serialize_unit_variant("UsNonprofitType", 10u32, "Nonprofit501C18")
				}
				UsNonprofitType::Nonprofit501C19 => {
					serializer.serialize_unit_variant("UsNonprofitType", 11u32, "Nonprofit501C19")
				}
				UsNonprofitType::Nonprofit501C2 => {
					serializer.serialize_unit_variant("UsNonprofitType", 12u32, "Nonprofit501C2")
				}
				UsNonprofitType::Nonprofit501C20 => {
					serializer.serialize_unit_variant("UsNonprofitType", 13u32, "Nonprofit501C20")
				}
				UsNonprofitType::Nonprofit501C21 => {
					serializer.serialize_unit_variant("UsNonprofitType", 14u32, "Nonprofit501C21")
				}
				UsNonprofitType::Nonprofit501C22 => {
					serializer.serialize_unit_variant("UsNonprofitType", 15u32, "Nonprofit501C22")
				}
				UsNonprofitType::Nonprofit501C23 => {
					serializer.serialize_unit_variant("UsNonprofitType", 16u32, "Nonprofit501C23")
				}
				UsNonprofitType::Nonprofit501C24 => {
					serializer.serialize_unit_variant("UsNonprofitType", 17u32, "Nonprofit501C24")
				}
				UsNonprofitType::Nonprofit501C25 => {
					serializer.serialize_unit_variant("UsNonprofitType", 18u32, "Nonprofit501C25")
				}
				UsNonprofitType::Nonprofit501C26 => {
					serializer.serialize_unit_variant("UsNonprofitType", 19u32, "Nonprofit501C26")
				}
				UsNonprofitType::Nonprofit501C27 => {
					serializer.serialize_unit_variant("UsNonprofitType", 20u32, "Nonprofit501C27")
				}
				UsNonprofitType::Nonprofit501C28 => {
					serializer.serialize_unit_variant("UsNonprofitType", 21u32, "Nonprofit501C28")
				}
				UsNonprofitType::Nonprofit501C3 => {
					serializer.serialize_unit_variant("UsNonprofitType", 22u32, "Nonprofit501C3")
				}
				UsNonprofitType::Nonprofit501C4 => {
					serializer.serialize_unit_variant("UsNonprofitType", 23u32, "Nonprofit501C4")
				}
				UsNonprofitType::Nonprofit501C5 => {
					serializer.serialize_unit_variant("UsNonprofitType", 24u32, "Nonprofit501C5")
				}
				UsNonprofitType::Nonprofit501C6 => {
					serializer.serialize_unit_variant("UsNonprofitType", 25u32, "Nonprofit501C6")
				}
				UsNonprofitType::Nonprofit501C7 => {
					serializer.serialize_unit_variant("UsNonprofitType", 26u32, "Nonprofit501C7")
				}
				UsNonprofitType::Nonprofit501C8 => {
					serializer.serialize_unit_variant("UsNonprofitType", 27u32, "Nonprofit501C8")
				}
				UsNonprofitType::Nonprofit501C9 => {
					serializer.serialize_unit_variant("UsNonprofitType", 28u32, "Nonprofit501C9")
				}
				UsNonprofitType::Nonprofit501D => {
					serializer.serialize_unit_variant("UsNonprofitType", 29u32, "Nonprofit501D")
				}
				UsNonprofitType::Nonprofit501E => {
					serializer.serialize_unit_variant("UsNonprofitType", 30u32, "Nonprofit501E")
				}
				UsNonprofitType::Nonprofit501F => {
					serializer.serialize_unit_variant("UsNonprofitType", 31u32, "Nonprofit501F")
				}
				UsNonprofitType::Nonprofit501K => {
					serializer.serialize_unit_variant("UsNonprofitType", 32u32, "Nonprofit501K")
				}
				UsNonprofitType::Nonprofit501N => {
					serializer.serialize_unit_variant("UsNonprofitType", 33u32, "Nonprofit501N")
				}
				UsNonprofitType::Nonprofit501Q => {
					serializer.serialize_unit_variant("UsNonprofitType", 34u32, "Nonprofit501Q")
				}
				UsNonprofitType::Nonprofit527 => {
					serializer.serialize_unit_variant("UsNonprofitType", 35u32, "Nonprofit527")
				}
			}
		}
	}
	impl<'de> Deserialize<'de> for UsNonprofitType {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Nonprofit501A,
				Nonprofit501C1,
				Nonprofit501C10,
				Nonprofit501C11,
				Nonprofit501C12,
				Nonprofit501C13,
				Nonprofit501C14,
				Nonprofit501C15,
				Nonprofit501C16,
				Nonprofit501C17,
				Nonprofit501C18,
				Nonprofit501C19,
				Nonprofit501C2,
				Nonprofit501C20,
				Nonprofit501C21,
				Nonprofit501C22,
				Nonprofit501C23,
				Nonprofit501C24,
				Nonprofit501C25,
				Nonprofit501C26,
				Nonprofit501C27,
				Nonprofit501C28,
				Nonprofit501C3,
				Nonprofit501C4,
				Nonprofit501C5,
				Nonprofit501C6,
				Nonprofit501C7,
				Nonprofit501C8,
				Nonprofit501C9,
				Nonprofit501D,
				Nonprofit501E,
				Nonprofit501F,
				Nonprofit501K,
				Nonprofit501N,
				Nonprofit501Q,
				Nonprofit527,
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
						"Nonprofit501A" => Ok(Field::Nonprofit501A),
						"Nonprofit501C1" => Ok(Field::Nonprofit501C1),
						"Nonprofit501C10" => Ok(Field::Nonprofit501C10),
						"Nonprofit501C11" => Ok(Field::Nonprofit501C11),
						"Nonprofit501C12" => Ok(Field::Nonprofit501C12),
						"Nonprofit501C13" => Ok(Field::Nonprofit501C13),
						"Nonprofit501C14" => Ok(Field::Nonprofit501C14),
						"Nonprofit501C15" => Ok(Field::Nonprofit501C15),
						"Nonprofit501C16" => Ok(Field::Nonprofit501C16),
						"Nonprofit501C17" => Ok(Field::Nonprofit501C17),
						"Nonprofit501C18" => Ok(Field::Nonprofit501C18),
						"Nonprofit501C19" => Ok(Field::Nonprofit501C19),
						"Nonprofit501C2" => Ok(Field::Nonprofit501C2),
						"Nonprofit501C20" => Ok(Field::Nonprofit501C20),
						"Nonprofit501C21" => Ok(Field::Nonprofit501C21),
						"Nonprofit501C22" => Ok(Field::Nonprofit501C22),
						"Nonprofit501C23" => Ok(Field::Nonprofit501C23),
						"Nonprofit501C24" => Ok(Field::Nonprofit501C24),
						"Nonprofit501C25" => Ok(Field::Nonprofit501C25),
						"Nonprofit501C26" => Ok(Field::Nonprofit501C26),
						"Nonprofit501C27" => Ok(Field::Nonprofit501C27),
						"Nonprofit501C28" => Ok(Field::Nonprofit501C28),
						"Nonprofit501C3" => Ok(Field::Nonprofit501C3),
						"Nonprofit501C4" => Ok(Field::Nonprofit501C4),
						"Nonprofit501C5" => Ok(Field::Nonprofit501C5),
						"Nonprofit501C6" => Ok(Field::Nonprofit501C6),
						"Nonprofit501C7" => Ok(Field::Nonprofit501C7),
						"Nonprofit501C8" => Ok(Field::Nonprofit501C8),
						"Nonprofit501C9" => Ok(Field::Nonprofit501C9),
						"Nonprofit501D" => Ok(Field::Nonprofit501D),
						"Nonprofit501E" => Ok(Field::Nonprofit501E),
						"Nonprofit501F" => Ok(Field::Nonprofit501F),
						"Nonprofit501K" => Ok(Field::Nonprofit501K),
						"Nonprofit501N" => Ok(Field::Nonprofit501N),
						"Nonprofit501Q" => Ok(Field::Nonprofit501Q),
						"Nonprofit527" => Ok(Field::Nonprofit527),
						_ => Err(de::Error::unknown_variant(value, VARIANTS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"Nonprofit501A" => Ok(Field::Nonprofit501A),
						b"Nonprofit501C1" => Ok(Field::Nonprofit501C1),
						b"Nonprofit501C10" => Ok(Field::Nonprofit501C10),
						b"Nonprofit501C11" => Ok(Field::Nonprofit501C11),
						b"Nonprofit501C12" => Ok(Field::Nonprofit501C12),
						b"Nonprofit501C13" => Ok(Field::Nonprofit501C13),
						b"Nonprofit501C14" => Ok(Field::Nonprofit501C14),
						b"Nonprofit501C15" => Ok(Field::Nonprofit501C15),
						b"Nonprofit501C16" => Ok(Field::Nonprofit501C16),
						b"Nonprofit501C17" => Ok(Field::Nonprofit501C17),
						b"Nonprofit501C18" => Ok(Field::Nonprofit501C18),
						b"Nonprofit501C19" => Ok(Field::Nonprofit501C19),
						b"Nonprofit501C2" => Ok(Field::Nonprofit501C2),
						b"Nonprofit501C20" => Ok(Field::Nonprofit501C20),
						b"Nonprofit501C21" => Ok(Field::Nonprofit501C21),
						b"Nonprofit501C22" => Ok(Field::Nonprofit501C22),
						b"Nonprofit501C23" => Ok(Field::Nonprofit501C23),
						b"Nonprofit501C24" => Ok(Field::Nonprofit501C24),
						b"Nonprofit501C25" => Ok(Field::Nonprofit501C25),
						b"Nonprofit501C26" => Ok(Field::Nonprofit501C26),
						b"Nonprofit501C27" => Ok(Field::Nonprofit501C27),
						b"Nonprofit501C28" => Ok(Field::Nonprofit501C28),
						b"Nonprofit501C3" => Ok(Field::Nonprofit501C3),
						b"Nonprofit501C4" => Ok(Field::Nonprofit501C4),
						b"Nonprofit501C5" => Ok(Field::Nonprofit501C5),
						b"Nonprofit501C6" => Ok(Field::Nonprofit501C6),
						b"Nonprofit501C7" => Ok(Field::Nonprofit501C7),
						b"Nonprofit501C8" => Ok(Field::Nonprofit501C8),
						b"Nonprofit501C9" => Ok(Field::Nonprofit501C9),
						b"Nonprofit501D" => Ok(Field::Nonprofit501D),
						b"Nonprofit501E" => Ok(Field::Nonprofit501E),
						b"Nonprofit501F" => Ok(Field::Nonprofit501F),
						b"Nonprofit501K" => Ok(Field::Nonprofit501K),
						b"Nonprofit501N" => Ok(Field::Nonprofit501N),
						b"Nonprofit501Q" => Ok(Field::Nonprofit501Q),
						b"Nonprofit527" => Ok(Field::Nonprofit527),
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
				type Value = UsNonprofitType;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema USNonprofitType")
				}
				fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
				where
					A: de::EnumAccess<'de>,
				{
					match de::EnumAccess::variant::<Field>(data)? {
						(Field::Nonprofit501A, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501A)
						}
						(Field::Nonprofit501C1, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C1)
						}
						(Field::Nonprofit501C10, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C10)
						}
						(Field::Nonprofit501C11, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C11)
						}
						(Field::Nonprofit501C12, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C12)
						}
						(Field::Nonprofit501C13, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C13)
						}
						(Field::Nonprofit501C14, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C14)
						}
						(Field::Nonprofit501C15, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C15)
						}
						(Field::Nonprofit501C16, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C16)
						}
						(Field::Nonprofit501C17, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C17)
						}
						(Field::Nonprofit501C18, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C18)
						}
						(Field::Nonprofit501C19, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C19)
						}
						(Field::Nonprofit501C2, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C2)
						}
						(Field::Nonprofit501C20, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C20)
						}
						(Field::Nonprofit501C21, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C21)
						}
						(Field::Nonprofit501C22, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C22)
						}
						(Field::Nonprofit501C23, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C23)
						}
						(Field::Nonprofit501C24, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C24)
						}
						(Field::Nonprofit501C25, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C25)
						}
						(Field::Nonprofit501C26, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C26)
						}
						(Field::Nonprofit501C27, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C27)
						}
						(Field::Nonprofit501C28, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C28)
						}
						(Field::Nonprofit501C3, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C3)
						}
						(Field::Nonprofit501C4, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C4)
						}
						(Field::Nonprofit501C5, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C5)
						}
						(Field::Nonprofit501C6, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C6)
						}
						(Field::Nonprofit501C7, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C7)
						}
						(Field::Nonprofit501C8, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C8)
						}
						(Field::Nonprofit501C9, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501C9)
						}
						(Field::Nonprofit501D, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501D)
						}
						(Field::Nonprofit501E, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501E)
						}
						(Field::Nonprofit501F, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501F)
						}
						(Field::Nonprofit501K, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501K)
						}
						(Field::Nonprofit501N, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501N)
						}
						(Field::Nonprofit501Q, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit501Q)
						}
						(Field::Nonprofit527, variant) => {
							de::VariantAccess::unit_variant(variant)?;
							Ok(UsNonprofitType::Nonprofit527)
						}
					}
				}
			}
			const VARIANTS: &[&str] = &[
				"Nonprofit501A",
				"Nonprofit501C1",
				"Nonprofit501C10",
				"Nonprofit501C11",
				"Nonprofit501C12",
				"Nonprofit501C13",
				"Nonprofit501C14",
				"Nonprofit501C15",
				"Nonprofit501C16",
				"Nonprofit501C17",
				"Nonprofit501C18",
				"Nonprofit501C19",
				"Nonprofit501C2",
				"Nonprofit501C20",
				"Nonprofit501C21",
				"Nonprofit501C22",
				"Nonprofit501C23",
				"Nonprofit501C24",
				"Nonprofit501C25",
				"Nonprofit501C26",
				"Nonprofit501C27",
				"Nonprofit501C28",
				"Nonprofit501C3",
				"Nonprofit501C4",
				"Nonprofit501C5",
				"Nonprofit501C6",
				"Nonprofit501C7",
				"Nonprofit501C8",
				"Nonprofit501C9",
				"Nonprofit501D",
				"Nonprofit501E",
				"Nonprofit501F",
				"Nonprofit501K",
				"Nonprofit501N",
				"Nonprofit501Q",
				"Nonprofit527",
			];
			deserializer.deserialize_enum("UsNonprofitType", VARIANTS, EnumerationVisitor)
		}
	}
}
