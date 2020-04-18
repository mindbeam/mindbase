#[derive(Debug)]
pub struct MBQLError {
    pub position: Position,
    pub kind:     MBQLErrorKind,
}

use crate::{
    error::MBError,
    mbql::Position,
};

#[derive(Debug)]
pub enum MBQLErrorKind {
    IOError {
        error: std::io::Error,
    },
    ParseRow {
        input:    String,
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

    // TODO 2 - Move this to MBError
    GSymNotFound,

    MBError(Box<MBError>),
}

impl std::convert::From<MBError> for MBQLError {
    fn from(error: MBError) -> Self {
        MBQLError { position: Position::none(),
                    kind:     MBQLErrorKind::MBError(Box::new(error)), }
    }
}
impl std::convert::From<MBQLError> for std::io::Error {
    fn from(error: MBQLError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{}", error))
    }
}

impl std::fmt::Display for MBQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            MBQLErrorKind::IOError { error } => f.write_fmt(format_args!("IO Error: {}", error)),
            MBQLErrorKind::InvalidLine { input } => f.write_fmt(format_args!("Invalid row at {}: {}", self.position.row, input)),
            MBQLErrorKind::ParseRow { pest_err, .. } => {
                // TODO - fix line numbers
                f.write_fmt(format_args!("Failed to parse row {}: {}", self.position.row, pest_err))
            },
            MBQLErrorKind::InvalidCommand { .. } => f.write_str("meow"),
            MBQLErrorKind::UnknownCommand { .. } => f.write_str("meow"),
            MBQLErrorKind::CommandParse { .. } => f.write_str("meow"),
            MBQLErrorKind::MBError(e) => write!(f, "{:?}", e),
            MBQLErrorKind::ArtifactVarNotFound { var } => {
                write!(f, "Artifact Variable `{}` not found at row {}", var, self.position.row)
            },
            MBQLErrorKind::SymbolVarNotFound { var } => {
                write!(f, "Symbol Variable `{}` not found at row {}", var, self.position.row)
            },
            MBQLErrorKind::GSymNotFound => write!(f, "Ground Symbol not found at row {}", self.position.row),
        }
    }
}
