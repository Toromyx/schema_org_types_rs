use oxigraph::sparql::{QueryResults, QuerySolution};

pub trait IntoSolutions {
	fn into_solutions(self) -> Vec<QuerySolution>;
}

impl IntoSolutions for QueryResults {
	fn into_solutions(self) -> Vec<QuerySolution> {
		match self {
			QueryResults::Solutions(solutions) => solutions
				.into_iter()
				.map(|solution_result| solution_result.unwrap())
				.collect(),
			_ => {
				panic!("Expected query results to be the variant Solutions.");
			}
		}
	}
}
