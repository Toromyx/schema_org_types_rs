use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote};

use crate::{schema::data_type::DataType, serde_attributes::serde_as};

pub fn serde_mod(data_type: &DataType) -> TokenStream {
	let schema_name = &data_type.name;
	let name_string = schema_name.to_case(Case::UpperCamel);
	let name = TokenStream::from_str(&name_string).unwrap();
	let expecting = format!("schema.org schema {}", schema_name);

	let inner_type = &data_type.rust_type;

	let serialize_newtype_struct_value = match data_type.rust_type.serde_as() {
		Some(serde_as_str) => {
			let serde_with = serde_as(serde_as_str);
			quote!({
				struct SerializeWith<'a>(&'a #inner_type);

				impl<'a> Serialize for SerializeWith<'a> {
					fn serialize<S>(
						&self,
						serializer: S,
					) -> Result<S::Ok, S::Error>
					where
						S: Serializer,
					{
						#serde_with::serialize(self.0, serializer)
					}

				}

				&SerializeWith(&self.0)
			})
		}
		None => quote!(&self.0),
	};

	let deserialize_type = match data_type.rust_type.serde_as() {
		Some(serde_as_str) => serde_as(serde_as_str),
		None => quote!(<#inner_type as Deserialize>),
	};

	quote!(
		use std::{fmt, fmt::Formatter};

		use ::serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

		use super::*;

		impl Serialize for #name {
			fn serialize<S>(
				&self,
				serializer: S,
			) -> Result<S::Ok, S::Error>
			where
				S: Serializer,
			{
				serializer.serialize_newtype_struct(
					#name_string,
					#serialize_newtype_struct_value,
				)
			}
		}

		impl<'de> Deserialize<'de> for #name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct DataTypeVisitor;

				impl<'de> Visitor<'de> for DataTypeVisitor {
					type Value = #name;

					fn expecting(
						&self,
						formatter: &mut Formatter,
					) -> fmt::Result {
						formatter.write_str(
							#expecting,
						)
					}

					fn visit_newtype_struct<E>(
						self,
						e: E,
					) -> Result<Self::Value, E::Error>
					where
						E: Deserializer<'de>,
					{
						let inner: #inner_type = #deserialize_type::deserialize(e)?;
						Ok(#name(inner))
					}
				}

				Deserializer::deserialize_newtype_struct(
					deserializer,
					#name_string,
					DataTypeVisitor,
				)
			}
		}
	)
}
