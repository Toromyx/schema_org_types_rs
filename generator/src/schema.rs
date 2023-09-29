use derivative::Derivative;
use oxigraph::store::Store;

use crate::{schema_section::SchemaSection, sparql::SectionedSchemaQuerySolution};

pub mod class;
pub mod data_type;
pub mod enumeration;
pub mod property;

pub trait Schema {
    /// Get the module name describing where to write this schema into.
    fn module_name() -> &'static str;

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

    /// Read query solutions of this schema from the RDF store.
    fn read_solutions(store: &Store) -> Vec<SectionedSchemaQuerySolution>;

    /// Build the complete schema from a query solution and the RDF store.
    fn from_solution(store: &Store, solution: SectionedSchemaQuerySolution) -> Self;
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

impl From<SectionedSchemaQuerySolution> for ReferencedSchema {
    fn from(value: SectionedSchemaQuerySolution) -> Self {
        Self {
            iri: value.schema.identifiable.iri,
            name: value.schema.labeled.label,
            section: value.sectioned.section,
        }
    }
}
