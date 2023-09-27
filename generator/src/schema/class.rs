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
    serde_attributes::{serde_default, serde_derive, serde_rename, serde_skip_serializing_if},
    sparql::{SchemaQueries, SectionedSchemaQuerySolution},
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
    fn module_name() -> &'static str {
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
                    sectioned_label.label.to_case(Case::Kebab)
                )
            })
            .collect()
    }

    fn read_solutions(store: &Store) -> Vec<SectionedSchemaQuerySolution> {
        store.classes_query()
    }

    fn from_solution(store: &Store, solution: SectionedSchemaQuerySolution) -> Self {
        let mut properties: Vec<ReferencedSchema> = store
            .property_labels_of_class_query(&solution.schema.identifiable.iri)
            .into_par_iter()
            .map(ReferencedSchema::from)
            .collect();
        properties.sort_unstable();
        Class {
            iri: solution.schema.identifiable.iri,
            name: map_schema_name(solution.schema.labeled.label),
            properties,
            section: solution.sectioned.section,
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
            .map(|ReferencedSchema { label, section }| {
                let feature = Feature::Any(vec![
                    Feature::Name(format!("{}-property-schema", label.to_case(Case::Kebab))),
                    Feature::Name(section.feature_name().to_string()),
                ]);
                let feature_gate = feature.feature_gate();
                let serde_rename = serde_rename(label);
                let serde_default = serde_default();
                let serde_skip_serializing_if_empty = serde_skip_serializing_if("Vec::is_empty");
                let property = TokenStream::from_str(&format!(
                    "pub r#{}: Vec<{}Property>",
                    label.to_case(Case::Snake),
                    label.to_case(Case::UpperCamel)
                ))
                .unwrap();
                quote!(
                    #feature_gate
                    #serde_rename
                    #serde_default
                    #serde_skip_serializing_if_empty
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
