mod as_literal;
mod as_named_node;
mod into_solutions;
pub mod node_type;

use as_literal::AsLiteral;
use as_named_node::AsNamedNode;
use into_solutions::IntoSolutions;
use oxigraph::{
	sparql::{QueryResults, QuerySolution},
	store::Store,
};

/// The prefixes/namespaces used in the schema.org RDF
const PREFIXES: &str = r#"
PREFIX schema: <https://schema.org/>
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
"#;

/// The schemas which are ignored in this crate.
///
/// These don't work as types in rust:
/// - <https://schema.org/True>
/// - <https://schema.org/False>
///
/// These are never used:
/// - <https://schema.org/DataType>
/// - <https://schema.org/Intangible>
/// - <https://schema.org/Series>
const IGNORED_SCHEMAS_FILTER: &str = r#"
FILTER NOT EXISTS {
	VALUES ?node {
		schema:True
		schema:False
		schema:DataType
		schema:Intangible
		schema:Series
	}
}
"#;

fn iri_from_solution(solution: &QuerySolution) -> String {
	solution
		.get("node")
		.expect("The ?node variable should exist within the query solution.")
		.as_named_node()
		.as_str()
		.to_string()
}

fn label_from_solution(solution: &QuerySolution) -> String {
	solution
		.get("label")
		.expect("The ?label variable should exist within the query solution.")
		.as_literal()
		.value()
		.to_string()
}

#[derive(Debug, Clone)]
pub struct SchemaQuerySolution {
	pub iri: String,
	pub label: String,
}

impl From<&QuerySolution> for SchemaQuerySolution {
	fn from(value: &QuerySolution) -> Self {
		Self {
			iri: iri_from_solution(value),
			label: label_from_solution(value),
		}
	}
}

pub struct EnumerationVariantSolution {
	pub iri: String,
	pub label: String,
}

impl From<&QuerySolution> for EnumerationVariantSolution {
	fn from(value: &QuerySolution) -> Self {
		Self {
			iri: iri_from_solution(value),
			label: label_from_solution(value),
		}
	}
}

struct CountSolution(u64);

impl From<QueryResults> for CountSolution {
	fn from(value: QueryResults) -> Self {
		Self(
			value
				.into_solutions()
				.pop()
				.expect("There should always be a query solution within a count query.")
				.get("count")
				.expect("The ?count variable should exist within a count query solution.")
				.as_literal()
				.value()
				.parse::<u64>()
				.expect("The ?count literal should be parsable as an unsigned integer."),
		)
	}
}

pub trait SchemaQueries {
	fn get_schemas(&self) -> Vec<SchemaQuerySolution>;

	fn is_enumeration_variant(&self, iri: &str) -> bool;

	fn is_data_type(&self, iri: &str) -> bool;

	fn is_enumeration(&self, iri: &str) -> bool;

	fn is_property(&self, iri: &str) -> bool;

	/// Get all direct properties of a class, not including parents' properties
	fn get_properties_of_class(&self, class_iri: &str) -> Vec<SchemaQuerySolution>;

	/// Get all parents of a class via rdfs:subClassOf
	fn get_parents_of_class(&self, class_iri: &str) -> Vec<SchemaQuerySolution>;

	/// Query for all value labels of a property.
	fn get_variants_of_property(&self, property_iri: &str) -> Vec<SchemaQuerySolution>;

	/// Query for all enumeration variants of a specific enumeration.
	fn get_variants_of_enumeration(&self, enumeration_iri: &str)
	-> Vec<EnumerationVariantSolution>;

	/// Query for a transformable parent data type of another data type.
	///
	/// The data type needs to be transformable to a [`crate::serde_attributes::data_type::RustType`].
	fn get_transformable_data_type_label_of_data_type(&self, data_type_iri: &str) -> String;
}

impl SchemaQueries for Store {
	fn get_schemas(&self) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
WHERE {{
	{}
	?node rdfs:label ?label .
}}
"#,
			PREFIXES, IGNORED_SCHEMAS_FILTER
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}

	fn is_enumeration_variant(&self, iri: &str) -> bool {
		let query = format!(
			r#"
{}
SELECT
	(COUNT(*) AS ?count)
WHERE {{
	<{}> a ?enumeration .
	?enumeration rdfs:subClassOf*/rdfs:subClassOf schema:Enumeration .
}}
"#,
			PREFIXES, iri
		);
		let count_solution: CountSolution = self.query(&query).unwrap().into();
		count_solution.0 > 0
	}

	fn is_data_type(&self, iri: &str) -> bool {
		let query = format!(
			r#"
{}
SELECT
	(COUNT(*) AS ?count)
WHERE {{
	<{}> rdfs:subClassOf*/a schema:DataType .
}}
"#,
			PREFIXES, iri
		);
		let count_solution: CountSolution = self.query(&query).unwrap().into();
		count_solution.0 > 0
	}

	fn is_enumeration(&self, iri: &str) -> bool {
		let query = format!(
			r#"
{}
SELECT
	(COUNT(*) AS ?count)
WHERE {{
	<{}> rdfs:subClassOf*/rdfs:subClassOf schema:Enumeration .
	FILTER NOT EXISTS {{
		?property schema:domainIncludes <{}> .
	}}
}}
"#,
			PREFIXES, iri, iri
		);
		let count_solution: CountSolution = self.query(&query).unwrap().into();
		count_solution.0 > 0
	}

	fn is_property(&self, iri: &str) -> bool {
		let query = format!(
			r#"
{}
SELECT
	(COUNT(*) AS ?count)
WHERE {{
	<{}> a rdf:Property .
}}
"#,
			PREFIXES, iri
		);
		let count_solution: CountSolution = self.query(&query).unwrap().into();
		count_solution.0 > 0
	}

	fn get_properties_of_class(&self, class_iri: &str) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
WHERE {{
	{}
	?node schema:domainIncludes <{}> .
	?node rdfs:label ?label .
}}
"#,
			PREFIXES, IGNORED_SCHEMAS_FILTER, class_iri
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}

	fn get_parents_of_class(&self, class_iri: &str) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT DISTINCT
	?node
	?label
WHERE {{
	{}
	<{}> rdfs:subClassOf*/rdfs:subClassOf ?node .
	?node rdfs:label ?label .
}}
"#,
			PREFIXES, IGNORED_SCHEMAS_FILTER, class_iri
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}

	fn get_variants_of_property(&self, property_iri: &str) -> Vec<SchemaQuerySolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
WHERE {{
	{}
	<{}> schema:rangeIncludes ?node .
	?node rdfs:label ?label .
}}
"#,
			PREFIXES, IGNORED_SCHEMAS_FILTER, property_iri
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(SchemaQuerySolution::from)
			.collect()
	}

	fn get_variants_of_enumeration(
		&self,
		enumeration_iri: &str,
	) -> Vec<EnumerationVariantSolution> {
		let query = format!(
			r#"
{}
SELECT
	?node
	?label
WHERE {{
	{}
	?node a <{}> .
	?node rdfs:label ?label .
}}
"#,
			PREFIXES, IGNORED_SCHEMAS_FILTER, enumeration_iri
		);
		self.query(&query)
			.unwrap()
			.into_solutions()
			.iter()
			.map(EnumerationVariantSolution::from)
			.collect()
	}

	fn get_transformable_data_type_label_of_data_type(&self, data_type_iri: &str) -> String {
		let query = format!(
			r#"
{}
SELECT
	?label
WHERE {{
	<{}> rdfs:subClassOf* ?transformable .
	VALUES ?transformable {{
		schema:URL
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
		let solution = solutions.first().unwrap_or_else(|| {
			panic!(
				"Could not get a transformable data type for the schema \"{}\"",
				data_type_iri
			);
		});
		label_from_solution(solution)
	}
}
