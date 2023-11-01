use super::*;
/// <https://schema.org/PronounceableText>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct PronounceableText(pub String);
impl std::ops::Deref for PronounceableText {
	type Target = String;
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
	impl Serialize for PronounceableText {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_newtype_struct("PronounceableText", &self.0)
		}
	}
	impl<'de> Deserialize<'de> for PronounceableText {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct DataTypeVisitor;
			impl<'de> Visitor<'de> for DataTypeVisitor {
				type Value = PronounceableText;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema PronounceableText")
				}
				fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
				where
					E: Deserializer<'de>,
				{
					let inner: String = <String as Deserialize>::deserialize(e)?;
					Ok(PronounceableText(inner))
				}
			}
			Deserializer::deserialize_newtype_struct(
				deserializer,
				"PronounceableText",
				DataTypeVisitor,
			)
		}
	}
}
