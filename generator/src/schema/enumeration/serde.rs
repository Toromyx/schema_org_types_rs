use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote, ToTokens};

use crate::schema::enumeration::Enumeration;

pub fn serde_mod(enumeration: &Enumeration) -> TokenStream {
	let schema_name = &enumeration.name;
	let name_string = schema_name.to_case(Case::UpperCamel);
	let name = TokenStream::from_str(&name_string).unwrap();
	let expecting = format!("schema.org schema {}", schema_name);

	struct VariantTokenStreams {
		variant_name_string: TokenStream,
		variant_name: TokenStream,
	}

	let variant_token_streams: Vec<VariantTokenStreams> = enumeration
		.variants
		.iter()
		.map(|enumeration_variant| {
			let variant_name_string = enumeration_variant.name.to_case(Case::UpperCamel);
			let variant_name = TokenStream::from_str(&variant_name_string).unwrap();
			VariantTokenStreams {
				variant_name_string: variant_name_string.to_token_stream(),
				variant_name,
			}
		})
		.collect();

	let serialize_match_arms =
		variant_token_streams
			.iter()
			.enumerate()
			.map(|(index, token_streams)| {
				let variant_name_string = &token_streams.variant_name_string;
				let variant_name = &token_streams.variant_name;
				let index_u32 = index as u32;
				quote!(
					#name::#variant_name => serializer.serialize_unit_variant(
						#name_string,
						#index_u32,
						#variant_name_string,
					),
				)
			});

	let field_enum_variants = variant_token_streams.iter().map(|token_streams| {
		let variant_name = &token_streams.variant_name;
		quote!(#variant_name,)
	});

	let visit_str_match_arms = variant_token_streams.iter().map(|token_streams| {
		let variant_name_string = &token_streams.variant_name_string;
		let variant_name = &token_streams.variant_name;
		quote!(
			#variant_name_string => Ok(Field::#variant_name),
		)
	});

	let visit_str_body = if variant_token_streams.is_empty() {
		quote!(Err(de::Error::unknown_variant(value, VARIANTS)))
	} else {
		quote!(
			match value {
				#(#visit_str_match_arms)*
				_ => Err(de::Error::unknown_variant(value, VARIANTS)),
			}
		)
	};

	let visit_bytes_match_arms = variant_token_streams.iter().map(|token_streams| {
		let variant_name_string =
			TokenStream::from_str(&format!("b{}", token_streams.variant_name_string)).unwrap();
		let variant_name = &token_streams.variant_name;
		quote!(
			#variant_name_string => Ok(Field::#variant_name),
		)
	});

	let visit_bytes_body = if variant_token_streams.is_empty() {
		quote!(
			let value = &String::from_utf8_lossy(value);
			Err(de::Error::unknown_variant(value, VARIANTS))
		)
	} else {
		quote!(
			match value {
				#(#visit_bytes_match_arms)*
				_ => {
					let value = &String::from_utf8_lossy(value);
					Err(de::Error::unknown_variant(value, VARIANTS))
				}
			}
		)
	};

	let visit_enum_match_arms = variant_token_streams.iter().map(|token_streams| {
		let variant_name = &token_streams.variant_name;
		quote!(
			(Field::#variant_name, variant) => {
				de::VariantAccess::unit_variant(variant)?;
				Ok(#name::#variant_name)
			}
		)
	});

	let visit_enum_body = if variant_token_streams.is_empty() {
		quote!(de::EnumAccess::variant::<Field>(data).map(|(impossible, _)| match impossible {}))
	} else {
		quote!(
			match de::EnumAccess::variant::<Field>(data)? {
				#(#visit_enum_match_arms)*
			}
		)
	};

	let variant_items = variant_token_streams.iter().map(|token_streams| {
		let variant_name_string = &token_streams.variant_name_string;
		quote!(
			#variant_name_string,
		)
	});

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
				match *self {
					#(#serialize_match_arms)*
				}
			}
		}

		impl<'de> Deserialize<'de> for #name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				enum Field {
					#(#field_enum_variants)*
				}

				struct FieldVisitor;

				impl<'de> de::Visitor<'de> for FieldVisitor {

					type Value = Field;

					fn expecting(
						&self,
						formatter: &mut Formatter,
					) -> fmt::Result {
						formatter.write_str("variant identifier")
					}

					fn visit_str<E>(
						self,
						value: &str,
					) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						#visit_str_body
					}

					fn visit_bytes<E>(
						self,
						value: &[u8],
					) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						#visit_bytes_body
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

				struct EnumerationVisitor;

				impl<'de> Visitor<'de> for EnumerationVisitor {

					type Value = #name;

					fn expecting(
						&self,
						formatter: &mut Formatter,
					) -> fmt::Result {
						formatter.write_str(#expecting)
					}

					fn visit_enum<A>(
						self,
						data: A,
					) -> Result<Self::Value, A::Error>
					where
						A: de::EnumAccess<'de>,
					{
						#visit_enum_body
					}
				}

				const VARIANTS: &[&str] = &[
					#(#variant_items)*
				];

				deserializer.deserialize_enum(
					#name_string,
					VARIANTS,
					EnumerationVisitor,
				)
			}
		}
	)
}
