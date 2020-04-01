#[derive(Debug)]
pub struct Item {
    pub key:        String,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Agent(Agent),
    Alledge(Alledge),
    GroundSymbol(GroundSymbol),
}

#[derive(Debug)]
pub struct Alledge {
    pub(crate) thing:      AlledgeThing,
    pub(crate) categorize: Option<Category>,
}

#[derive(Debug)]
pub enum AlledgeThing {
    Variable(Variable),
    FlatText(FlatText),
    Agent(Agent),
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
