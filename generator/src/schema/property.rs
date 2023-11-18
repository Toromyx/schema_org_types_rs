mod serde;

use std::{cmp::Ordering, str::FromStr};

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::{
	deprecated_attribute::DeprecatedAttribute,
	doc_lines::{strings_as_doc_lines, DocLines},
	feature::Feature,
	schema::{
		data_type::rust_type::RustType, map_schema_name, property::serde::serde_mod,
		ReferencedSchema, Schema,
	},
	sparql::{SchemaQueries, SchemaQuerySolution},
};

/// A schema.org property.
///
/// This struct contains all information to be quoted into a rust enum.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Property {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub variants: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub superseded_by: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
}

impl Schema for Property {
	fn parent_module_name() -> &'static str {
		"properties"
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let mut variants: Vec<ReferencedSchema> = store
			.get_variants_of_property(&solution.iri)
			.into_iter()
			.map(|solution| ReferencedSchema::from_solution(store, solution))
			.collect();

		#[derive(PartialEq, Eq, PartialOrd, Ord)]
		enum VariantType {
			Class,
			Enumeration,
			DataType,
		}

		impl VariantType {
			fn from_variant(store: &Store, variant: &ReferencedSchema) -> Self {
				if store.is_enumeration(&variant.iri) {
					return Self::Enumeration;
				}
				if store.is_data_type(&variant.iri) {
					return Self::DataType;
				}
				Self::Class
			}
		}

		variants.sort_unstable_by(|variant, other| {
			let variant_type = VariantType::from_variant(store, variant);
			let other_type = VariantType::from_variant(store, other);
			match variant_type.cmp(&other_type) {
				Ordering::Less => Ordering::Less,
				Ordering::Equal => match variant_type {
					VariantType::DataType => RustType::from(variant.name.as_str())
						.cmp(&RustType::from(other.name.as_str())),
					_ => variant.cmp(other),
				},
				Ordering::Greater => Ordering::Greater,
			}
		});
		let superseded_by = store
			.get_superseded_by(&solution.iri)
			.into_iter()
			.map(|solution| ReferencedSchema::from_solution(store, solution))
			.collect();
		Self {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			variants,
			superseded_by,
			in_attic: solution.in_attic,
		}
	}
}

impl DeprecatedAttribute for Property {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[ReferencedSchema] {
		self.superseded_by.as_slice()
	}
}

impl ToTokens for Property {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let deprecated_attribute = self.deprecated_attribute();
		let name =
			TokenStream::from_str(&format!("{}Property", self.name.to_case(Case::UpperCamel)))
				.unwrap();
		let variants = self.variants.iter().map(|referenced_schema| {
			let doc_lines = strings_as_doc_lines(&[format!("<{}>", referenced_schema.iri)]);
			let deprecated_attribute = referenced_schema.deprecated_attribute();
			let variant_name =
				TokenStream::from_str(&referenced_schema.name.to_case(Case::UpperCamel)).unwrap();
			quote!(
				#doc_lines
				#deprecated_attribute
				#variant_name(#variant_name),
			)
		});
		let serde_mod = serde_mod(self);
		let fallible_feature_gate = Feature::All(vec![
			Feature::Name("fallible".to_string()),
			Feature::Name("serde".to_string()),
		])
		.as_cfg_attribute();
		tokens.append_all(quote! (
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#deprecated_attribute
			pub enum #name {
				#(#variants)*
				#fallible_feature_gate
				SerdeFail(crate::fallible::FailValue),
			}
			#[cfg(feature = "serde")]
			mod serde {
				#serde_mod
			}
		));
	}
}
