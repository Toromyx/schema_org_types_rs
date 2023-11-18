mod serde;

use std::{cell::RefCell, collections::HashSet, rc::Rc, str::FromStr};

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};

use crate::{
	doc_lines::{strings_as_doc_lines, DocLines},
	schema::{class::serde::serde_mod, map_schema_name, ReferencedSchema, Schema},
	sparql::{node_type::NodeType, SchemaQueries, SchemaQuerySolution},
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
	parents: Vec<ReferencedClassSchema>,
}

impl Class {
	fn get_all_properties_iter(&self) -> impl Iterator<Item = &ReferencedSchema> {
		let set: Rc<RefCell<HashSet<&str>>> = Rc::new(RefCell::new(HashSet::new()));
		let set_1 = set.clone();
		let set_2 = set.clone();
		self.properties
			.iter()
			.filter(move |property| set_1.borrow_mut().insert(&property.name))
			.chain(self.parents.iter().flat_map(move |parent| {
				let set = set_2.clone();
				parent
					.properties
					.iter()
					.filter(move |property| set.borrow_mut().insert(&property.name))
			}))
	}
}

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct ReferencedClassSchema {
	name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	properties: Vec<ReferencedSchema>,
}

impl Schema for Class {
	fn parent_module_name() -> &'static str {
		"classes"
	}

	fn name(&self) -> &String {
		&self.name
	}

	fn iri(&self) -> &String {
		&self.iri
	}

	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let mut properties: Vec<_> = store
			.get_properties_of_class(&solution.iri)
			.into_iter()
			.map(ReferencedSchema::from)
			.collect();
		properties.sort_unstable();
		let mut parents: Vec<_> = store
			.get_parents_of_class(&solution.iri)
			.into_iter()
			.filter(|solution| NodeType::from_iri(store, &solution.iri) == NodeType::Class)
			.map(|solution| {
				let mut parent_properties: Vec<_> = store
					.get_properties_of_class(&solution.iri)
					.into_iter()
					.map(ReferencedSchema::from)
					.collect();
				parent_properties.sort_unstable();
				ReferencedClassSchema {
					name: solution.label,
					properties: parent_properties,
				}
			})
			.collect();
		parents.sort_unstable();
		Class {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			properties,
			parents,
		}
	}
}

fn get_property_name(referenced_schema: &ReferencedSchema) -> TokenStream {
	TokenStream::from_str(&format!(
		"r#{}",
		get_property_name_unescaped(referenced_schema)
	))
	.unwrap()
}

fn get_property_name_unescaped(referenced_schema: &ReferencedSchema) -> String {
	referenced_schema.name.to_case(Case::Snake)
}

fn get_property_type(referenced_schema: &ReferencedSchema) -> TokenStream {
	TokenStream::from_str(&format!(
		"Vec<{}>",
		get_property_type_single(referenced_schema)
	))
	.unwrap()
}

fn get_property_type_single(referenced_schema: &ReferencedSchema) -> TokenStream {
	TokenStream::from_str(&format!(
		"{}Property",
		referenced_schema.name.to_case(Case::UpperCamel)
	))
	.unwrap()
}

fn get_trait_name(name: &str) -> TokenStream {
	TokenStream::from_str(&format!("{}Trait", name.to_case(Case::UpperCamel))).unwrap()
}

impl ToTokens for Class {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let doc_lines = self.doc_lines_token_stream();
		let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
		let fields = self.get_all_properties_iter().map(|referenced_schema| {
			let doc_lines = strings_as_doc_lines(&[format!("<{}>", referenced_schema.iri)]);
			let property = TokenStream::from_str(&format!(
				"pub {}: {}",
				get_property_name(referenced_schema),
				get_property_type(referenced_schema),
			))
			.unwrap();
			quote!(
				#doc_lines
				#property,
			)
		});
		let trait_doc_lines =
			strings_as_doc_lines(&[format!("This trait is for properties from <{}>.", self.iri)]);
		let trait_name = get_trait_name(&self.name);

		fn get_get_function_signature(property: &ReferencedSchema) -> TokenStream {
			let property_name = get_property_name_unescaped(property);
			let get_function_name =
				TokenStream::from_str(&format!("get_{}", property_name)).unwrap();
			let property_type_single = get_property_type_single(property);
			quote!(
				fn #get_function_name(&self) -> &[#property_type_single]
			)
		}

		fn get_take_function_signature(property: &ReferencedSchema) -> TokenStream {
			let property_name = get_property_name_unescaped(property);
			let take_function_name =
				TokenStream::from_str(&format!("take_{}", property_name)).unwrap();
			let property_type = get_property_type(property);
			quote!(
				fn #take_function_name(&mut self) -> #property_type
			)
		}

		let trait_functions = self.properties.iter().map(|referenced_schema| {
			let get_function_doc_lines = strings_as_doc_lines(&[format!(
				"Get <{}> from [`Self`] as borrowed slice.",
				referenced_schema.iri
			)]);
			let get_function_signature = get_get_function_signature(referenced_schema);
			let take_function_doc_lines = strings_as_doc_lines(&[format!(
				"Take <{}> from [`Self`] as owned vector.",
				referenced_schema.iri
			)]);
			let take_function_signature = get_take_function_signature(referenced_schema);
			quote!(
				#get_function_doc_lines
				#get_function_signature;
				#take_function_doc_lines
				#take_function_signature;
			)
		});

		fn get_trait_function_impls(properties: &[ReferencedSchema]) -> TokenStream {
			let trait_impls = properties.iter().map(|referenced_schema| {
				let get_function_signature = get_get_function_signature(referenced_schema);
				let take_function_signature = get_take_function_signature(referenced_schema);
				let property_name = get_property_name(referenced_schema);
				quote!(
					#get_function_signature {
						self.#property_name.as_slice()
					}
					#take_function_signature {
						std::mem::take(&mut self.#property_name)
					}
				)
			});
			quote!(
				#(#trait_impls)*
			)
		}

		let trait_function_impls = get_trait_function_impls(&self.properties);
		let parent_trait_impls = self.parents.iter().map(|referenced_schema| {
			let trait_name = get_trait_name(&referenced_schema.name);
			let trait_function_impls = get_trait_function_impls(&referenced_schema.properties);
			quote!(
				impl #trait_name for #name {
					#trait_function_impls
				}
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
			#trait_doc_lines
			pub trait #trait_name {
				#(#trait_functions)*
			}
			impl #trait_name for #name {
				#trait_function_impls
			}
			#(#parent_trait_impls)*
			#[cfg(feature = "serde")]
			mod serde {
				#serde_mod
			}
		));
	}
}
