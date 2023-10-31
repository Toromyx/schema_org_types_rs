use super::*;
/// <https://schema.org/Float>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Float(pub crate::number_types::Number);
impl std::ops::Deref for Float {
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
	impl Serialize for Float {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("Float", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for Float {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = Float;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Float")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: crate::number_types::Number =
						<crate::number_types::Number as Deserialize>::deserialize(e)?;
					Ok(Float(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "Float", DataTypeVisitor)
		}
	}
}
