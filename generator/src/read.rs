use std::io::Cursor;

use oxigraph::{model::GraphNameRef, store::Store};

pub async fn read() -> Store {
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
    store
}
