use std::{collections::BinaryHeap, io::Cursor, sync::Mutex};

use indicatif::{MultiProgress, ProgressBar};
use oxigraph::{model::GraphNameRef, store::Store};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    schema::{
        class::Class, data_type::DataType, enumeration::Enumeration, property::Property, Schema,
    },
    Schemas,
};

/// Map schema names which are incompatible with rust as identifier.
pub fn map_schema_name(name: String) -> String {
    match name.as_str() {
        "3DModel" => "Model3D".to_string(),
        _ => name,
    }
}

trait ReadSchemas {
    fn read_schemas(store: &Store, multi_progress: &MultiProgress) -> Vec<Self>
    where
        Self: Sized;
}

impl<T: Schema + Ord + Send> ReadSchemas for T {
    fn read_schemas(store: &Store, multi_progress: &MultiProgress) -> Vec<Self> {
        let schemas = Mutex::new(BinaryHeap::new());
        let solutions = Self::read_solutions(store);
        let bar = multi_progress.add(ProgressBar::new(solutions.len() as u64));
        solutions.into_par_iter().for_each(|solution| {
            schemas
                .lock()
                .unwrap()
                .push(Self::from_solution(store, solution));
            bar.inc(1);
        });
        schemas.into_inner().unwrap().into_sorted_vec()
    }
}

pub async fn read_schemas(multi_progress: &MultiProgress) -> Schemas {
    let store = Store::new().unwrap();
    let rdf = reqwest::get("https://schema.org/version/latest/schemaorg-all-https.rdf")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    store
        .bulk_loader()
        .load_graph(
            Cursor::new(rdf.as_bytes()),
            oxigraph::io::GraphFormat::RdfXml,
            GraphNameRef::DefaultGraph,
            None,
        )
        .unwrap();

    Schemas {
        classes: Class::read_schemas(&store, multi_progress),
        properties: Property::read_schemas(&store, multi_progress),
        enumerations: Enumeration::read_schemas(&store, multi_progress),
        data_types: DataType::read_schemas(&store, multi_progress),
    }
}
