#[derive(Debug)]
pub struct Error {
    pub position: Position,
    pub kind: ErrorKind,
}

use crate::{ast, Position};

#[derive(Debug)]
pub enum ErrorKind {
    IOError {
        error: std::io::Error,
    },
    ParseRow {
        input: String,
        pest_err: pest::error::Error<super::parse::Rule>,
    },
    InvalidLine {
        input: String,
    },
    InvalidCommand {
        command: String,
    },
    UnknownCommand {
        command: String,
    },
    CommandParse {
        body: String,
        // ron:         ron::de::Error,
    },
    ArtifactVarNotFound {
        var: String,
    },
    SymbolVarNotFound {
        var: String,
    },
    SymbolVarBindingFailed {
        bound_to: std::rc::Rc<ast::SymbolStatement>,
    },

    // TODO 2 - Move this to MBError
    GSymNotFound,
    // MBError(Box<MBError>),
}

// impl std::convert::From<MBError> for MBQLError {
//     fn from(error: MBError) -> Self {
//         MBQLError {
//             position: Position::none(),
//             kind: ErrorKind::MBError(Box::new(error)),
//         }
//     }
// }

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{}", error))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IOError { error } => f.write_fmt(format_args!("IO Error: {}", error)),
            ErrorKind::InvalidLine { input } => f.write_fmt(format_args!("Invalid row at {}: {}", self.position.row, input)),
            ErrorKind::ParseRow { pest_err, .. } => {
                // TODO - fix line numbers
                f.write_fmt(format_args!("Failed to parse row {}: {}", self.position.row, pest_err))
            }
            ErrorKind::InvalidCommand { .. } => f.write_str("meow"),
            ErrorKind::UnknownCommand { .. } => f.write_str("meow"),
            ErrorKind::CommandParse { .. } => f.write_str("meow"),
            // ErrorKind::MBError(e) => write!(f, "{:?}", e),
            ErrorKind::ArtifactVarNotFound { var } => {
                write!(f, "Artifact Variable `{}` not found at row {}", var, self.position.row)
            }
            ErrorKind::SymbolVarNotFound { var } => {
                write!(f, "Symbol Variable `{}` not found at row {}", var, self.position.row)
            }
            ErrorKind::GSymNotFound => write!(f, "Ground Symbol not found at row {}", self.position.row),

            ErrorKind::SymbolVarBindingFailed { bound_to } => write!(
                f,
                "Symbol binding failed at row {}. Already bound to row {}",
                self.position.row,
                bound_to.position().row
            ),
        }
    }
}
