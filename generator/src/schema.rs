use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;

use crate::{
	deprecated_attribute::DeprecatedAttribute,
	sparql::{SchemaQueries, SchemaQuerySolution},
};

pub mod class;
pub mod data_type;
pub mod enumeration;
pub mod property;

pub trait Schema {
	/// Get the module name describing where to write this schema into.
	fn parent_module_name() -> &'static str;

	/// Get the name of this schema.
	fn name(&self) -> &String;

	/// Get the IRI of this schema.
	fn iri(&self) -> &String;

	/// Get the module name of this schema. Per default this is [`Case::Snake`] of the [`Self::name`].
	fn module_name(&self) -> String {
		self.name().to_case(Case::Snake)
	}

	/// Build the complete schema from a query solution and the RDF store.
	fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self;
}

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct ReferencedSchema {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub superseded_by: Vec<ReferencedSchema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
}

impl ReferencedSchema {
	fn from_solution(store: &Store, value: SchemaQuerySolution) -> Self {
		let superseded_by = store
			.get_superseded_by(&value.iri)
			.into_iter()
			.map(|solution| ReferencedSchema::from_solution(store, solution))
			.collect();
		Self {
			iri: value.iri,
			name: value.label,
			superseded_by,
			in_attic: value.in_attic,
		}
	}
}

impl DeprecatedAttribute for ReferencedSchema {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[ReferencedSchema] {
		self.superseded_by.as_slice()
	}
}

/// Map schema names which are incompatible with rust as identifier.
pub fn map_schema_name(name: String) -> String {
	match name.as_str() {
		"3DModel" => "Model3D".to_string(),
		_ => name,
	}
}
