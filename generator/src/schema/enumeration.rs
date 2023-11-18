pub mod enumeration_variant;
mod serde;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use enumeration_variant::EnumerationVariant;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::{
	deprecated_attribute::DeprecatedAttribute,
	doc_lines::DocLines,
	schema::{enumeration::serde::serde_mod, map_schema_name, ReferencedSchema, Schema},
	sparql::{SchemaQueries, SchemaQuerySolution},
};

/// A schema.org enumeration.
///
/// This struct contains all information to be quoted into a rust enum.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub variants: Vec<EnumerationVariant>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub superseded_by: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
}

impl Schema for Enumeration {
	fn parent_module_name() -> &'static str {
		"enumerations"
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let mut variants: Vec<EnumerationVariant> = store
			.get_variants_of_enumeration(&solution.iri)
			.into_iter()
			.map(|solution| {
				let superseded_by = store
					.get_superseded_by(&solution.iri)
					.into_iter()
					.map(|solution| ReferencedSchema::from_solution(store, solution))
					.collect();
				EnumerationVariant {
					iri: solution.iri,
					name: map_schema_name(solution.label),
					superseded_by,
					in_attic: solution.in_attic,
				}
			})
			.collect();
		variants.sort_unstable();
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

impl DeprecatedAttribute for Enumeration {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[ReferencedSchema] {
		self.superseded_by.as_slice()
	}
}

impl ToTokens for Enumeration {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let deprecated_attribute = self.deprecated_attribute();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let variants = &self.variants;
		let serde_mod = serde_mod(self);
		tokens.append_all(quote!(
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#deprecated_attribute
			pub enum #name {
				#(#variants)*
			}
			#[cfg(feature = "serde")]
			mod serde {
				#serde_mod
			}
		));
	}
}
