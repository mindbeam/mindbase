use crate::artifact::*;

#[derive(Debug)]
pub enum Symbol {
    Artifact,
    SymbolPair(Box<Symbol>), // (Alledge/ Analogy)

    // TODO 1 - determine if we want to flatten/variablize/pointerize the tree as we parse it
    // or if we flatten that structure at a later phase?
    SymbolVar,
    Ground,
    Symbolize,
}

#[derive(Debug)]
pub struct SymbolPair {
    pub(crate) thing:      Symbol,
    pub(crate) categorize: Symbol,
}
