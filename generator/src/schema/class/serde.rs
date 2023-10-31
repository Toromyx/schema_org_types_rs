use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote, ToTokens};

use crate::{
	schema::class::{property_feature, property_name, property_type, Class},
	serde_attributes::serde_as,
};

pub fn serde_mod(class: &Class) -> TokenStream {
	let schema_name = &class.name;
	let name_string = schema_name.to_case(Case::UpperCamel);
	let name = TokenStream::from_str(&name_string).unwrap();
	let expecting = format!("schema.org schema {}", schema_name);
	struct PropertyTokenStreams {
		feature_gate_as_attribute: TokenStream,
		feature_gate_as_macro: TokenStream,
		name: TokenStream,
		variable_name: TokenStream,
		serialized_name_string: TokenStream,
		r#type: TokenStream,
		variant_name: TokenStream,
	}
	let property_token_streams: Vec<PropertyTokenStreams> = class
		.properties
		.iter()
		.map(|referenced_schema| {
			let feature = property_feature(referenced_schema);
			PropertyTokenStreams {
				feature_gate_as_attribute: feature.as_cfg_attribute(),
				feature_gate_as_macro: feature.as_cfg_macro(),
				name: property_name(referenced_schema),
				variable_name: TokenStream::from_str(&format!(
					"{}_property",
					property_name(referenced_schema)
				))
				.unwrap(),
				serialized_name_string: referenced_schema.name.to_token_stream(),
				r#type: property_type(referenced_schema),
				variant_name: TokenStream::from_str(
					&referenced_schema.name.to_case(Case::UpperCamel),
				)
				.unwrap(),
			}
		})
		.collect();
	let serde_with = serde_as("OneOrMany<_>");
	let len_terms = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_macro;
		let name = &property_token_streams.name;
		quote!(
			if #feature_gate { !Vec::is_empty(&self.#name) as usize } else { 0 },
		)
	});
	let serialize_fields = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let name = &property_token_streams.name;
		let serialized_name_string = &property_token_streams.serialized_name_string;
		let r#type = &property_token_streams.r#type;
		quote!(
			#feature_gate
			if !Vec::is_empty(&self.#name) {
				serialize_struct.serialize_field(
					#serialized_name_string,
					{
						struct SerializeWith<'a>(&'a #r#type);
						impl<'a> Serialize for SerializeWith<'a> {
							fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
							where
								S: Serializer,
							{
								#serde_with::serialize(self.0, serializer)
							}
						}
						&SerializeWith(&self.#name)
					},
				)?;
			} else {
				serialize_struct.skip_field(#serialized_name_string)?;
			}
		)
	});
	let field_enum_variants = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let variant_name = &property_token_streams.variant_name;
		quote!(
			#feature_gate
			#variant_name,
		)
	});
	let visit_str_match_arms = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let variant_name = &property_token_streams.variant_name;
		let serialized_name_string = &property_token_streams.serialized_name_string;
		quote!(
			#feature_gate
			#serialized_name_string => Ok(Field::#variant_name),
		)
	});
	let visit_bytes_match_arms = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let variant_name = &property_token_streams.variant_name;
		let serialized_name_string = &property_token_streams.serialized_name_string;
		let serialized_name =
			TokenStream::from_str(&format!("b{}", serialized_name_string)).unwrap();
		quote!(
			#feature_gate
			#serialized_name => Ok(Field::#variant_name),
		)
	});
	let visit_map_assignments = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let variable = &property_token_streams.variable_name;
		quote!(
			#feature_gate
			let mut #variable = None;
		)
	});
	let visit_map_next_key_match_arms =
		property_token_streams.iter().map(|property_token_streams| {
			let feature_gate = &property_token_streams.feature_gate_as_attribute;
			let serialized_name_string = &property_token_streams.serialized_name_string;
			let variable = &property_token_streams.variable_name;
			let variant_name = &property_token_streams.variant_name;
			let r#type = &property_token_streams.r#type;
			quote!(
				#feature_gate
				Field::#variant_name => {
					if #variable.is_some() {
						return Err(
							<A::Error as de::Error>::duplicate_field(
								#serialized_name_string,
							),
						);
					}
					#variable = Some({
						struct DeserializeWith(#r#type);
						impl<'de> Deserialize<'de> for DeserializeWith {
							fn deserialize<D>(
								deserializer: D,
							) -> Result<Self, D::Error>
							where
								D: Deserializer<'de>,
							{
								Ok(DeserializeWith(
									#serde_with::deserialize(deserializer)?
								))
							}
						}
						match map.next_value::<DeserializeWith>() {
							Ok(deserialize_with) => deserialize_with.0,
							Err(err) => {
								return Err(err);
							}
						}
					});
				},
			)
		});
	let visit_map_field_assignments = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let name = &property_token_streams.name;
		let variable = &property_token_streams.variable_name;
		quote!(
			#feature_gate
			#name: #variable.unwrap_or_default(),
		)
	});
	let field_strings = property_token_streams.iter().map(|property_token_streams| {
		let feature_gate = &property_token_streams.feature_gate_as_attribute;
		let serialized_name_string = &property_token_streams.serialized_name_string;
		quote!(
			#feature_gate
			#serialized_name_string,
		)
	});
	quote!(
		use std::{fmt, fmt::Formatter};

		use ::serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

		use super::*;

		impl Serialize for #name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: Serializer,
			{
				let len: usize = [
					#(#len_terms)*
				]
				.iter()
				.sum();
				let mut serialize_struct = Serializer::serialize_struct(
					serializer,
					#name_string,
					len,
				)?;
				#(#serialize_fields)*
				serialize_struct.end()
			}
		}

		impl<'de> Deserialize<'de> for #name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				enum Field {
					#(#field_enum_variants)*
					Ignore,
				}

				struct FieldVisitor;

				impl<'de> Visitor<'de> for FieldVisitor {
					type Value = Field;

					fn expecting(
						&self,
						formatter: &mut Formatter,
					) -> fmt::Result {
						formatter.write_str("field identifier")
					}

					fn visit_str<E>(
						self,
						value: &str,
					) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						match value {
							#(#visit_str_match_arms)*
							_ => Ok(Field::Ignore),
						}
					}

					fn visit_bytes<E>(
						self,
						value: &[u8],
					) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						match value {
							#(#visit_bytes_match_arms)*
							_ => Ok(Field::Ignore),
						}
					}
				}

				impl<'de> Deserialize<'de> for Field {
					fn deserialize<D>(
						deserializer: D,
					) -> Result<Self, D::Error>
					where
						D: Deserializer<'de>,
					{
						deserializer.deserialize_identifier(FieldVisitor)
					}
				}

				struct ClassVisitor;

				impl<'de> Visitor<'de> for ClassVisitor {
					type Value = #name;

					fn expecting(
						&self,
						formatter: &mut Formatter,
					) -> fmt::Result {
						formatter.write_str(#expecting)
					}

					fn visit_map<A>(
						self,
						mut map: A,
					) -> Result<Self::Value, A::Error>
					where
						A: de::MapAccess<'de>,
					{
						#(#visit_map_assignments)*
						while let Some(key) = map.next_key::<Field>()? {
							match key {
								#(#visit_map_next_key_match_arms)*
								_ => {
									let _ = map.next_value::<de::IgnoredAny>()?;
								}
							}
						}
						Ok(#name {
							#(#visit_map_field_assignments)*
						})
					}
				}

				const FIELDS: &[&str] = &[
					#(#field_strings)*
				];
				deserializer.deserialize_struct(
					#name_string,
					FIELDS,
					ClassVisitor,
				)
			}
		}
	)
}
