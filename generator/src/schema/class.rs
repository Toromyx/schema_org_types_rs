mod serde;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
	doc_lines::DocLines,
	schema::{class::serde::serde_mod, map_schema_name, ReferencedSchema, Schema},
	sparql::{SchemaQueries, SchemaQuerySolution},
};

/// A schema.org class: <https://schema.org/Class>
///
/// This struct contains all information to be quoted into a rust struct.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub properties: Vec<ReferencedSchema>,
}

impl Schema for Class {
	fn parent_module_name() -> &'static str {
		"classes"
	}

	fn feature_name(&self) -> String {
		format!("{}-schema", self.name.to_case(Case::Kebab))
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn child_feature_names(&self) -> Vec<String> {
		self.properties
			.iter()
			.map(|sectioned_label| {
				format!(
					"{}-property-schema",
					sectioned_label.name.to_case(Case::Kebab)
				)
			})
			.collect()
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let mut properties: Vec<ReferencedSchema> = store
			.get_properties_of_class(&solution.iri)
			.into_par_iter()
			.map(ReferencedSchema::from)
			.collect();
		properties.sort_unstable();
		Class {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			properties,
		}
	}
}

fn property_name(referenced_schema: &ReferencedSchema) -> TokenStream {
	TokenStream::from_str(&format!(
		"r#{}",
		referenced_schema.name.to_case(Case::Snake)
	))
	.unwrap()
}

fn property_type(referenced_schema: &ReferencedSchema) -> TokenStream {
	TokenStream::from_str(&format!(
		"Vec<{}Property>",
		referenced_schema.name.to_case(Case::UpperCamel)
	))
	.unwrap()
}

impl ToTokens for Class {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let fields = self.properties.iter().map(|referenced_schema| {
			let property = TokenStream::from_str(&format!(
				"pub {}: {}",
				property_name(referenced_schema),
				property_type(referenced_schema),
			))
			.unwrap();
			quote!(
				#property,
			)
		});
		let serde_mod = serde_mod(self);
		tokens.append_all(quote!(
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			pub struct #name {
				#(#fields)*
			}
			#[cfg(feature = "serde")]
			mod serde {
				#serde_mod
			}
		));
	}
}
