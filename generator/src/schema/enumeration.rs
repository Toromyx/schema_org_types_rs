pub mod enumeration_variant;

use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use enumeration_variant::EnumerationVariant;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
    doc_lines::DocLines,
    read::map_schema_name,
    schema::Schema,
    schema_section::SchemaSection,
    serde_attributes::serde_derive,
    sparql::{SchemaQueries, SectionedSchemaQuerySolution},
};

/// A schema.org enumeration.
///
/// This struct contains all information to be quoted into a rust enum.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Enumeration {
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub description: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub iri: String,
    pub name: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub variants: Vec<EnumerationVariant>,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub section: SchemaSection,
}

impl Schema for Enumeration {
    fn module_name() -> &'static str {
        "enumerations"
    }

    fn feature_name(&self) -> String {
        format!("{}-schema", self.name.to_case(Case::Kebab))
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn section(&self) -> &SchemaSection {
        &self.section
    }

    fn child_feature_names(&self) -> Vec<String> {
        vec![]
    }

    fn read_solutions(store: &Store) -> Vec<SectionedSchemaQuerySolution> {
        store.enumerations_query()
    }

    fn from_solution(store: &Store, solution: SectionedSchemaQuerySolution) -> Self {
        let mut variants: Vec<EnumerationVariant> = store
            .enumeration_variant_labels_of_enumeration_query(&solution.schema.identifiable.iri)
            .into_par_iter()
            .map(|solution| EnumerationVariant {
                description: solution.commented.comment,
                iri: solution.identifiable.iri,
                name: map_schema_name(solution.labeled.label),
            })
            .collect();
        variants.sort_unstable();
        Self {
            description: solution.schema.commented.comment,
            iri: solution.schema.identifiable.iri,
            name: map_schema_name(solution.schema.labeled.label),
            variants,
            section: solution.sectioned.section,
        }
    }
}

impl ToTokens for Enumeration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_lines = DocLines(vec![
            self.description.clone(),
            String::from(""),
            self.iri.clone(),
        ]);
        let serde_derive = serde_derive();
        let name = TokenStream::from_str(&self.name.to_case(Case::UpperCamel)).unwrap();
        let variants = &self.variants;
        tokens.append_all(quote!(
            #doc_lines
            #[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
            #[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
            #serde_derive
            pub enum #name {
                #(#variants),*
            }
        ));
    }
}
