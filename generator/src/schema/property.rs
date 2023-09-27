use std::str::FromStr;

use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote, ToTokens, TokenStreamExt};
use rayon::prelude::*;

use crate::{
    doc_lines::DocLines,
    feature::Feature,
    read::map_schema_name,
    schema::{ReferencedSchema, Schema},
    schema_section::SchemaSection,
    serde_attributes::{serde_derive, serde_untagged},
    sparql::{SchemaQueries, SectionedSchemaQuerySolution},
};

/// A schema.org property.
///
/// This struct contains all information to be quoted into a rust enum.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Property {
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub iri: String,
    pub name: String,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub values: Vec<ReferencedSchema>,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub section: SchemaSection,
}

impl Schema for Property {
    fn module_name() -> &'static str {
        "properties"
    }

    fn feature_name(&self) -> String {
        format!("{}-property-schema", self.name.to_case(Case::Kebab))
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
        self.values
            .iter()
            .map(|sectioned_label| format!("{}-schema", sectioned_label.label.to_case(Case::Kebab)))
            .collect()
    }

    fn read_solutions(store: &Store) -> Vec<SectionedSchemaQuerySolution> {
        store.properties_query()
    }

    fn from_solution(store: &Store, solution: SectionedSchemaQuerySolution) -> Self {
        let mut values: Vec<ReferencedSchema> = store
            .property_value_labels_of_property_query(&solution.schema.identifiable.iri)
            .into_par_iter()
            .map(ReferencedSchema::from)
            .collect();
        values.sort_unstable();
        Self {
            iri: solution.schema.identifiable.iri,
            name: map_schema_name(solution.schema.labeled.label),
            values,
            section: solution.sectioned.section,
        }
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_lines = self.doc_lines_token_stream();
        let serde_derive = serde_derive();
        let serde_untagged = serde_untagged();
        let name =
            TokenStream::from_str(&format!("{}Property", self.name.to_case(Case::UpperCamel)))
                .unwrap();
        let variants = self
            .values
            .iter()
            .map(|ReferencedSchema { label, section }| {
                let value_upper_camel =
                    TokenStream::from_str(&label.to_case(Case::UpperCamel)).unwrap();
                let feature = Feature::Any(vec![
                    Feature::Name(format!("{}-schema", label.to_case(Case::Kebab))),
                    Feature::Name(section.feature_name().to_string()),
                ]);
                let feature_gate = feature.feature_gate();
                quote!(
                    #feature_gate
                    #value_upper_camel(#value_upper_camel)
                )
            });
        tokens.append_all(quote! (
            use super::*;
            #doc_lines
            #[cfg_attr(feature = "derive-debug", derive(Debug))]
            #[cfg_attr(feature = "derive-clone", derive(Clone))]
            #serde_derive
            #serde_untagged
            pub enum #name {
                #(#variants),*
            }
        ));
    }
}
