pub mod ast;
pub mod error;
pub mod parse;
pub mod query;
pub mod search;

pub use query::Query;

#[derive(Debug, Clone)]
pub struct Position {
    pub row: usize,
    // pub col:  usize,
}
impl Position {
    pub fn none() -> Self {
        Self { row: 0 }
    }

    pub fn row(row: usize) -> Self {
        Self { row }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
