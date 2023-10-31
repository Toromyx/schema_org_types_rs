use super::*;
/// <https://schema.org/Number>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Number(pub crate::number_types::Number);
impl std::ops::Deref for Number {
	type Target = crate::number_types::Number;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Number {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("Number", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for Number {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = Number;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Number")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: crate::number_types::Number =
						<crate::number_types::Number as Deserialize>::deserialize(e)?;
					Ok(Number(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "Number", DataTypeVisitor)
		}
	}
}
