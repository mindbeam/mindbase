use super::ast;
use std::fmt::Display;

impl Display for ast::Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:\t{}", self.key, self.expression))
    }
}

impl Display for ast::Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //
        match self {
            ast::Expression::Agent(a) => write!(f, "{}", a),
            ast::Expression::Alledge(a) => write!(f, "{}", a),
            ast::Expression::GroundSymbol(g) => write!(f, "{}", g),
        }
    }
}

impl Display for ast::Alledge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alledge(")?;

        match &self.thing {
            ast::AlledgeThing::Variable(v) => write!(f, "{}", v),
            ast::AlledgeThing::FlatText(s) => write!(f, "{}", s),
            ast::AlledgeThing::Agent(a) => write!(f, "{}", a),
        }?;

        if let Some(c) = &self.categorize {
            write!(f, ", {}", c)?;
        }

        write!(f, ")")
    }
}

impl Display for ast::Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{})", self.0)
    }
}

impl Display for ast::FlatText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0.replace("\"", "\\\""))
    }
}

impl Display for ast::Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Agent({})", self.0)
    }
}

impl Display for ast::Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[]")
    }
}

impl Display for ast::GroundSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GroundSymbol()")
    }
}
