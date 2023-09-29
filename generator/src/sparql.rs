mod as_literal;
mod as_named_node;
mod into_solutions;

use as_literal::AsLiteral;
use as_named_node::AsNamedNode;
use const_format::concatcp;
use into_solutions::IntoSolutions;
use oxigraph::{model::NamedNode, sparql::QuerySolution, store::Store};

use crate::schema_section::SchemaSection;

/// The prefixes/namespaces used in the schema.org RDF
const PREFIXES: &str = r#"
PREFIX schema: <https://schema.org/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"#;

/// A query to get all classes which are not an enumeration or data type
const CLASSES_QUERY: &str = concatcp!(
    PREFIXES,
    r#"
SELECT
    DISTINCT ?node
    ?label
    ?section
WHERE {
    ?node a rdfs:Class .
    FILTER NOT EXISTS {
        ?node rdfs:subClassOf*/rdfs:subClassOf schema:Enumeration .
    }
    FILTER NOT EXISTS {
        ?node rdfs:subClassOf*/a schema:DataType .
    }
    ?node rdfs:label ?label .
    OPTIONAL { ?node schema:isPartOf ?section . }
}
"#
);

/// A query to get all properties
const PROPERTIES_QUERY: &str = concatcp!(
    PREFIXES,
    r#"
SELECT
    DISTINCT ?node
    ?label
    ?section
WHERE {
    ?node a rdf:Property .
    ?node rdfs:label ?label .
    OPTIONAL { ?node schema:isPartOf ?section . }
}
"#
);

/// A query to get all enumerations
const ENUMERATIONS_QUERY: &str = concatcp!(
    PREFIXES,
    r#"
SELECT
    DISTINCT ?node
    ?label
    ?section
WHERE {
    ?node rdfs:subClassOf*/rdfs:subClassOf schema:Enumeration .
    ?node rdfs:label ?label .
    OPTIONAL { ?node schema:isPartOf ?section . }
}
"#
);

/// A query to get all data types
const DATA_TYPES_QUERY: &str = concatcp!(
    PREFIXES,
    r#"
SELECT
    DISTINCT ?node
    ?label
    ?section
WHERE {
    ?node rdfs:subClassOf*/a schema:DataType .
    ?node rdfs:label ?label .
    OPTIONAL { ?node schema:isPartOf ?section . }
}
"#
);

#[derive(Debug, Clone)]
pub struct IdentifiableQuerySolution {
    pub iri: String,
}

#[derive(Debug, Clone)]
pub struct LabeledQuerySolution {
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct SectionedQuerySolution {
    pub section: SchemaSection,
}

#[derive(Debug, Clone)]
pub struct SchemaQuerySolution {
    pub identifiable: IdentifiableQuerySolution,
    pub labeled: LabeledQuerySolution,
}

#[derive(Debug, Clone)]
pub struct SectionedSchemaQuerySolution {
    pub schema: SchemaQuerySolution,
    pub sectioned: SectionedQuerySolution,
}

impl From<&NamedNode> for SchemaSection {
    fn from(value: &NamedNode) -> Self {
        match value.as_str() {
            "https://attic.schema.org" => Self::Attic,
            "https://auto.schema.org" => Self::Auto,
            "https://bib.schema.org" => Self::Bib,
            "https://health-lifesci.schema.org" => Self::HealthLifesci,
            "https://meta.schema.org" => Self::Meta,
            "https://pending.schema.org" => Self::Pending,
            iri => {
                panic!("Could no determine the Schema.org section of \"{}\".", iri)
            }
        }
    }
}

impl From<&QuerySolution> for IdentifiableQuerySolution {
    fn from(value: &QuerySolution) -> Self {
        Self {
            iri: value
                .get("node")
                .expect("The ?node variable should exist within the query.")
                .as_named_node()
                .as_str()
                .to_string(),
        }
    }
}

impl From<&QuerySolution> for LabeledQuerySolution {
    fn from(value: &QuerySolution) -> Self {
        Self {
            label: value
                .get("label")
                .expect("The ?label variable should exist within the query.")
                .as_literal()
                .value()
                .to_string(),
        }
    }
}

impl From<&QuerySolution> for SectionedQuerySolution {
    fn from(value: &QuerySolution) -> Self {
        Self {
            section: value
                .get("section")
                .map(|term| term.as_named_node().into())
                .unwrap_or_default(),
        }
    }
}

impl From<&QuerySolution> for SchemaQuerySolution {
    fn from(value: &QuerySolution) -> Self {
        Self {
            identifiable: value.into(),
            labeled: value.into(),
        }
    }
}

impl From<&QuerySolution> for SectionedSchemaQuerySolution {
    fn from(value: &QuerySolution) -> Self {
        Self {
            schema: value.into(),
            sectioned: value.into(),
        }
    }
}

pub trait SchemaQueries {
    /// Query for all classes, which are not an enumeration ot data type.
    fn classes_query(&self) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all properties.
    fn properties_query(&self) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all enumerations.
    fn enumerations_query(&self) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all data types.
    fn data_types_query(&self) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all properties of a type, including the properties of types the type is a subclass of.
    fn properties_of_class_query(&self, class_iri: &str) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all value labels of a property.
    fn variants_of_property_query(&self, property_iri: &str) -> Vec<SectionedSchemaQuerySolution>;

    /// Query for all enumeration variants of a specific enumeration.
    fn variants_of_enumeration_query(&self, enumeration_iri: &str) -> Vec<SchemaQuerySolution>;

    /// Query for a transformable parent data type of another data type.
    ///
    /// The data type needs to be transformable to a [`crate::serde_attributes::data_type::RustType`].
    fn transformable_data_type_label_of_data_type_query(
        &self,
        data_type_iri: &str,
    ) -> LabeledQuerySolution;
}

impl SchemaQueries for Store {
    fn classes_query(&self) -> Vec<SectionedSchemaQuerySolution> {
        self.query(CLASSES_QUERY)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn properties_query(&self) -> Vec<SectionedSchemaQuerySolution> {
        self.query(PROPERTIES_QUERY)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn enumerations_query(&self) -> Vec<SectionedSchemaQuerySolution> {
        self.query(ENUMERATIONS_QUERY)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn data_types_query(&self) -> Vec<SectionedSchemaQuerySolution> {
        self.query(DATA_TYPES_QUERY)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn properties_of_class_query(&self, class_iri: &str) -> Vec<SectionedSchemaQuerySolution> {
        let query = format!(
            r#"
{}
SELECT DISTINCT
    ?node
    ?label
    ?section
WHERE {{
    {{
        <{}> rdfs:subClassOf* ?parent .
        ?node schema:domainIncludes ?parent .
    }}
    UNION
    {{
        ?node schema:domainIncludes <{}> .
    }}
    ?node rdfs:label ?label .
    OPTIONAL {{ ?node schema:isPartOf ?section . }}
}}
"#,
            PREFIXES, class_iri, class_iri
        );
        self.query(&query)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn variants_of_property_query(&self, property_iri: &str) -> Vec<SectionedSchemaQuerySolution> {
        let query = format!(
            r#"
{}
SELECT
    ?node
    ?label
    ?section
WHERE {{
    <{}> schema:rangeIncludes ?node .
    ?node rdfs:label ?label .
    OPTIONAL {{ ?node schema:isPartOf ?section . }}
}}
"#,
            PREFIXES, property_iri
        );
        self.query(&query)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SectionedSchemaQuerySolution::from)
            .collect()
    }

    fn variants_of_enumeration_query(&self, enumeration_iri: &str) -> Vec<SchemaQuerySolution> {
        let query = format!(
            r#"
{}
SELECT
    ?node
    ?label
WHERE {{
    ?node a <{}> .
    ?node rdfs:label ?label .
}}
"#,
            PREFIXES, enumeration_iri
        );
        self.query(&query)
            .unwrap()
            .into_solutions()
            .iter()
            .map(SchemaQuerySolution::from)
            .collect()
    }

    fn transformable_data_type_label_of_data_type_query(
        &self,
        data_type_iri: &str,
    ) -> LabeledQuerySolution {
        let query = format!(
            r#"
{}
SELECT
    ?label
WHERE {{
    <{}> rdfs:subClassOf* ?transformable .
    VALUES ?transformable {{
        schema:DateTime
        schema:Date
        schema:Time
        schema:Text
        schema:Integer
        schema:Number
        schema:Boolean
    }}
    ?transformable rdfs:label ?label .
}}
LIMIT 1
"#,
            PREFIXES, data_type_iri
        );
        let solutions = self.query(&query).unwrap().into_solutions();
        let Some(solution) = solutions.first() else {
            panic!(
                "Could not get a transformable data type for the schema \"{}\"",
                data_type_iri,
            );
        };
        solution.into()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use oxigraph::sparql::Query;

    use super::*;

    #[test]
    fn test_query_validity() {
        Query::from_str(CLASSES_QUERY).unwrap();
        Query::from_str(PROPERTIES_QUERY).unwrap();
        Query::from_str(ENUMERATIONS_QUERY).unwrap();
        Query::from_str(DATA_TYPES_QUERY).unwrap();
    }
}
