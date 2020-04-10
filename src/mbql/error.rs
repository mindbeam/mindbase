#[derive(Debug)]
pub struct MBQLError {
    pub position: Position,
    pub kind:     ErrorKind,
}

use crate::error::MBError;

#[derive(Debug)]
pub enum ErrorKind {
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
    MBError(Box<MBError>),
}
#[derive(Debug)]
pub struct Position {
    pub row: usize,
    // pub col:  usize,
}
impl Position {
    pub fn none() -> Self {
        Self { row: 0 }
    }
}

impl std::convert::From<MBError> for MBQLError {
    fn from(error: MBError) -> Self {
        MBQLError { position: Position::none(),
                    kind:     ErrorKind::MBError(Box::new(error)), }
    }
}
impl std::convert::From<MBQLError> for std::io::Error {
    fn from(error: MBQLError) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{}", error))
    }
}

impl std::fmt::Display for MBQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IOError { error } => f.write_fmt(format_args!("IO Error: {}", error)),
            ErrorKind::InvalidLine { input } => f.write_fmt(format_args!("Invalid row at {}: {}", self.position.row, input)),
            ErrorKind::ParseRow { input, pest_err } => {
                // TODO - fix line numbers
                f.write_fmt(format_args!("Failed to parse row {}: {}", self.position.row, pest_err))
            },
            ErrorKind::InvalidCommand { command } => f.write_str("meow"),
            ErrorKind::UnknownCommand { command } => f.write_str("meow"),
            ErrorKind::CommandParse { body } => f.write_str("meow"),
            ErrorKind::MBError(e) => write!(f, "{:?}", e),
        }
    }
}
