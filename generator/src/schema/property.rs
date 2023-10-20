use std::{cmp::Ordering, str::FromStr};

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
	doc_lines::DocLines,
	feature::Feature,
	schema::{data_type::rust_type::RustType, map_schema_name, ReferencedSchema, Schema},
	schema_section::SchemaSection,
	serde_attributes::{serde_derive, serde_untagged},
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
	pub section: SchemaSection,
}

impl Schema for Property {
	fn parent_module_name() -> &'static str {
		"properties"
	}

	fn feature_name(&self) -> String {
		format!("{}-property-schema", self.name.to_case(Case::Kebab))
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn section(&self) -> &SchemaSection {
		&self.section
	}

	fn child_feature_names(&self) -> Vec<String> {
		self.variants
			.iter()
			.map(|sectioned_label| format!("{}-schema", sectioned_label.name.to_case(Case::Kebab)))
			.collect()
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let mut variants: Vec<ReferencedSchema> = store
			.get_variants_of_property(&solution.iri)
			.into_par_iter()
			.map(ReferencedSchema::from)
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
		Self {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			variants,
			section: solution.section,
		}
	}
}

impl ToTokens for Property {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let serde_derive = serde_derive();
		let serde_untagged = serde_untagged();
		let name =
			TokenStream::from_str(&format!("{}Property", self.name.to_case(Case::UpperCamel)))
				.unwrap();
		let variants = self
			.variants
			.iter()
			.map(|ReferencedSchema { name, section, .. }| {
				let variant_name = TokenStream::from_str(&name.to_case(Case::UpperCamel)).unwrap();
				let feature = Feature::Any(vec![
					Feature::Name(format!("{}-schema", name.to_case(Case::Kebab))),
					Feature::Name(section.feature_name().to_string()),
				]);
				let feature_gate = feature.feature_gate();
				quote!(
					#feature_gate
					#variant_name(#variant_name)
				)
			});
		tokens.append_all(quote! (
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#serde_derive
			#serde_untagged
			pub enum #name {
				#(#variants),*
			}
		));
	}
}
