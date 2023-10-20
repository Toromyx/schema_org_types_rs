use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
	doc_lines::DocLines,
	feature::Feature,
	schema::{map_schema_name, ReferencedSchema, Schema},
	schema_section::SchemaSection,
	serde_attributes::{
		serde_as, serde_default, serde_derive, serde_rename, serde_skip_serializing_if,
	},
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
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub section: SchemaSection,
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

	fn section(&self) -> &SchemaSection {
		&self.section
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
			section: solution.section,
		}
	}
}

impl ToTokens for Class {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let serde_derive = serde_derive();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let fields = self
			.properties
			.iter()
			.map(|ReferencedSchema { name, section, .. }| {
				let feature = Feature::Any(vec![
					Feature::Name(format!("{}-property-schema", name.to_case(Case::Kebab))),
					Feature::Name(section.feature_name().to_string()),
				]);
				let feature_gate = feature.feature_gate();
				let serde_rename = serde_rename(name);
				let serde_default = serde_default();
				let serde_skip_serializing_if_empty = serde_skip_serializing_if("Vec::is_empty");
				let serde_as = serde_as("OneOrMany<_>");
				let property = TokenStream::from_str(&format!(
					"pub r#{}: Vec<{}Property>",
					name.to_case(Case::Snake),
					name.to_case(Case::UpperCamel)
				))
				.unwrap();
				quote!(
					#feature_gate
					#serde_rename
					#serde_default
					#serde_skip_serializing_if_empty
					#serde_as
					#property
				)
			});
		tokens.append_all(quote!(
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#serde_derive
			pub struct #name {
				#(#fields),*
			}
		));
	}
}
