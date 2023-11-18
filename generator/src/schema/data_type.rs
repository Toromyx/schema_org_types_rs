pub mod rust_type;
mod serde;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rust_type::RustType;

use crate::{
	deprecated_attribute::DeprecatedAttribute,
	doc_lines::DocLines,
	schema::{data_type::serde::serde_mod, map_schema_name, ReferencedSchema, Schema},
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
	pub superseded_by: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
}

impl Schema for DataType {
	fn parent_module_name() -> &'static str {
		"data_types"
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let transformable_type =
			store.get_transformable_data_type_label_of_data_type(&solution.iri);
		let superseded_by = store
			.get_superseded_by(&solution.iri)
			.into_iter()
			.map(|solution| ReferencedSchema::from_solution(store, solution))
			.collect();
		Self {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			rust_type: RustType::from(transformable_type.as_str()),
			superseded_by,
			in_attic: solution.in_attic,
		}
	}
}

impl DeprecatedAttribute for DataType {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[ReferencedSchema] {
		self.superseded_by.as_slice()
	}
}

impl ToTokens for DataType {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let deprecated_attribute = self.deprecated_attribute();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let rust_type = &self.rust_type;
		let serde_mod = serde_mod(self);
		tokens.append_all(quote!(
			use super::*;
			#doc_lines
			#[cfg_attr(feature = "derive-debug", derive(Debug))]
			#[cfg_attr(feature = "derive-clone", derive(Clone))]
			#deprecated_attribute
			pub struct #name(pub #rust_type);

			impl std::ops::Deref for #name {
				type Target = #rust_type;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			#[cfg(feature = "serde")]
			mod serde {
				#serde_mod
			}
		));
	}
}
