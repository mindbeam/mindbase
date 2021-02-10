use std::fmt::Display;

use json::JSONType;
use mindbase_artifact::{body::DataNode, body::SubGraph, test::TestWeight, Artifact, ArtifactId, NodeType};
use mindbase_data_adapters::json;
use mindbase_hypergraph::{
    entity::{directed, vertex},
    traits::Weight,
    EntityId, Hypergraph,
};

use std::sync::{Arc, Mutex};

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let v = include_str!("./colors.json");

    let mut graph: Hypergraph<_, TestWeight<JSONType>> = Hypergraph::memory();

    let adapter = json::JsonAdapter::new(&graph);

    let json_document = adapter.load(v)?;

    let mut out = std::io::stdout();
    graph.dump_entities(&mut out)?;

    // adapter.write(&mut out, json_document)?;

    Ok(())
}
