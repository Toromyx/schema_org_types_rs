use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;

use crate::{schema_section::SchemaSection, sparql::SchemaQuerySolution};

pub mod class;
pub mod data_type;
pub mod enumeration;
pub mod property;

pub trait Schema {
    /// Get the module name describing where to write this schema into.
    fn parent_module_name() -> &'static str;

    /// Get the feature name which gates the compilation of this schema.
    fn feature_name(&self) -> String;

    /// Get the name of this schema.
    fn name(&self) -> &String;

    /// Get the IRI of this schema.
    fn iri(&self) -> &String;

    /// Get the section of this schema.
    fn section(&self) -> &SchemaSection;

    /// Get a [`Vec`] of all features that the children of this schema depend on
    fn child_feature_names(&self) -> Vec<String>;

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
    pub section: SchemaSection,
}

impl From<SchemaQuerySolution> for ReferencedSchema {
    fn from(value: SchemaQuerySolution) -> Self {
        Self {
            iri: value.iri,
            name: value.label,
            section: value.section,
        }
    }
}

/// Map schema names which are incompatible with rust as identifier.
pub fn map_schema_name(name: String) -> String {
    match name.as_str() {
        "3DModel" => "Model3D".to_string(),
        _ => name,
    }
}
