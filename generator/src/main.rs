use std::{collections::BinaryHeap, sync::Mutex};

use indicatif::{MultiProgress, ProgressBar};
use rayon::prelude::*;
use read::read_schemas;
use schema::{class::Class, data_type::DataType, enumeration::Enumeration, property::Property};
use schema_feature::SchemaFeature;
use write::{write_features, write_schemas};

mod doc_lines;
mod feature;
mod read;
mod schema;
mod schema_feature;
mod schema_section;
mod serde_attributes;
mod sparql;
mod write;

#[tokio::main]
async fn main() {
    let multi_progress = MultiProgress::new();
    let schemas = read_schemas(&multi_progress).await;
    write_features(&schemas, &multi_progress);
    write_schemas(&schemas, &multi_progress);
}

#[derive(Debug, Clone)]
pub struct Schemas {
    pub classes: Vec<Class>,
    pub properties: Vec<Property>,
    pub enumerations: Vec<Enumeration>,
    pub data_types: Vec<DataType>,
}

impl Schemas {
    pub fn to_features(&self, multi_progress: &MultiProgress) -> Vec<SchemaFeature> {
        let heap = Mutex::new(BinaryHeap::new());
        let bar = multi_progress.add(ProgressBar::new(
            (self.classes.len()
                + self.properties.len()
                + self.enumerations.len()
                + self.data_types.len()) as u64,
        ));
        self.classes.par_iter().for_each(|schema| {
            heap.lock().unwrap().push(schema.into());
            bar.inc(1);
        });
        self.properties.par_iter().for_each(|schema| {
            heap.lock().unwrap().push(schema.into());
            bar.inc(1);
        });
        self.enumerations.par_iter().for_each(|schema| {
            heap.lock().unwrap().push(schema.into());
            bar.inc(1);
        });
        self.data_types.par_iter().for_each(|schema| {
            heap.lock().unwrap().push(schema.into());
            bar.inc(1);
        });
        heap.into_inner().unwrap().into_sorted_vec()
    }
}
