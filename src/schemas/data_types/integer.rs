use super::*;
/// <https://schema.org/Integer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Integer(pub crate::number_types::Integer);
impl std::ops::Deref for Integer {
	type Target = crate::number_types::Integer;
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
	impl Serialize for Integer {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("Integer", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for Integer {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = Integer;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Integer")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: crate::number_types::Integer =
						<crate::number_types::Integer as Deserialize>::deserialize(e)?;
					Ok(Integer(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "Integer", DataTypeVisitor)
		}
	}
}
