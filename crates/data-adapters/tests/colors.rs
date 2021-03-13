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

    let json_document = adapter.load(v.as_bytes(), "colors.json".to_string())?;

    let mut out = std::io::stdout();
    //graph.dump_entities(&mut out)?;

    // TODO 1 - LEFT OFF HERE
    // [ ] Fix writer
    //  [X] hyperedge indexing
    //  [ ] hyperedge traversal
    //  [ ] RootElement traversal
    // [ ] Search graph for JSON documents and list
    //    [X] Index entities by weight
    //    [ ] Query entities by symbol(weight)
    // [ ] Update to use Claims
    // [ ] Update to use Fuzzy Symbols
    // [ ] Update mbcli
    // [ ] First long-lived database (trivially practical storage application)
    // [ ] deeper Application invariant expression, mindful of fuzziness

    let (filename, root_id) = adapter.get_filename_and_root(&json_document)?;

    println!("Document name: {:?}", filename);
    println!("Document root {}", root_id);

    adapter.write(&mut out, root_id)?;

    Ok(())
}
