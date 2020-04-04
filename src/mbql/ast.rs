pub mod artifact;
pub mod symbol;

use super::ast;

#[derive(Debug)]
pub struct ArtifactVar {
    var: String,
}

#[derive(Debug)]
pub struct ArtifactStatement {
    pub var:      ArtifactVar,
    pub artifact: ast::artifact::Artifact,
}

#[derive(Debug)]
pub struct SymbolVar {
    var: String,
}
pub struct SymbolStatement {
    pub var:    Option<SymbolVar>,
    pub symbol: ast::symbol::Symbol,
}

#[derive(Debug)]
pub struct Variable(pub(crate) String);

#[derive(Debug)]
pub struct FlatText(pub(crate) String);

#[derive(Debug)]
pub struct Category {}

#[derive(Debug)]
pub struct Agent(pub(crate) String);

#[derive(Debug)]
pub struct GroundSymbol;
