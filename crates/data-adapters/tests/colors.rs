use json::test::TestJSONType;
use mindbase_artifact::Artifact;
use mindbase_data_adapters::json::{self, JsonAdapter};
use mindbase_hypergraph::Hypergraph;

/// Parse a simple JSON file into artifacts using a simple in-memory store
#[test]
fn colors() -> Result<(), std::io::Error> {
    let v = include_str!("./colors.json");
    let graph: Hypergraph<_, Artifact<TestJSONType>> = Hypergraph::memory();

    let adapter = JsonAdapter::new(&graph, TestJSONType::typemap());

    let _json_document = adapter.load(v.as_bytes(), "colors.json")?;

    let mut out = std::io::stdout();
    graph.dump_entities(&mut out)?;

    // adapter.write(&mut out, json_document)?;

    Ok(())
}
