use oxigraph::store::Store;

use crate::sparql::SchemaQueries;

#[derive(PartialEq)]
pub enum NodeType {
	EnumerationVariant,
	Property,
	DataType,
	Enumeration,
	Class,
}

impl NodeType {
	pub fn from_iri(store: &Store, iri: &str) -> Self {
		if store.is_enumeration_variant(iri) {
			return Self::EnumerationVariant;
		}
		if store.is_property(iri) {
			return Self::Property;
		}
		if store.is_data_type(iri) {
			return Self::DataType;
		}
		if store.is_enumeration(iri) {
			return Self::Enumeration;
		}
		Self::Class
	}
}
