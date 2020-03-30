use mindbase::Error;

fn main() -> Result<(), std::io::Error> {
    let mut query = mindbase::mbql::Query::new();

    let data = "foo: DefaultAgent";
    let cursor = std::io::Cursor::new(data);

    query.parse(cursor)?;

    Ok(())
}
