// https://github.com/mcarton/rust-derivative/issues/112
#![allow(clippy::incorrect_partial_ord_impl_on_ord_type)]

use indicatif::MultiProgress;
use read::read;
use write::write;

mod doc_lines;
mod feature;
mod read;
mod schema;
mod serde_attributes;
mod sparql;
mod write;

#[tokio::main]
async fn main() {
	let multi_progress = MultiProgress::new();
	let store = read().await;
	write(&store, &multi_progress);
}
