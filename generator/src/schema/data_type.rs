pub mod rust_type;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rust_type::RustType;

use crate::{
	doc_lines::DocLines,
	schema::{map_schema_name, Schema},
	schema_section::SchemaSection,
	serde_attributes::serde_derive,
	sparql::{SchemaQueries, SchemaQuerySolution},
};

/// A Schema.org data type: <https://schema.org/DataType>
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct DataType {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub rust_type: RustType,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub section: SchemaSection,
}

impl Schema for DataType {
	fn parent_module_name() -> &'static str {
		"data_types"
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
		vec![]
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let transformable_type =
			store.get_transformable_data_type_label_of_data_type(&solution.iri);
		Self {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			rust_type: RustType::from(transformable_type.as_str()),
			section: solution.section,
		}
	}
}

impl ToTokens for DataType {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let rust_type = &self.rust_type;
		let field_attribute = rust_type.serde_attributes();
		let serde_derive = serde_derive();
		tokens.append_all(quote!(
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#serde_derive
			pub struct #name(#field_attribute pub #rust_type);

			impl std::ops::Deref for #name {
				type Target = #rust_type;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}
		));
	}
}
