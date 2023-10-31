use super::*;
/// <https://schema.org/URL>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Url(pub ::url::Url);
impl std::ops::Deref for Url {
	type Target = ::url::Url;
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
	impl Serialize for Url {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("Url", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for Url {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = Url;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema URL")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: ::url::Url = <::url::Url as Deserialize>::deserialize(e)?;
					Ok(Url(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(deserializer, "Url", DataTypeVisitor)
		}
	}
}
