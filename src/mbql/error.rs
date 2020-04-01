#[derive(Debug)]
pub struct Error {
    pub position: Position,
    pub kind:     ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IOError {
        error: std::io::Error,
    },
    ParseRow {
        input: String,
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

impl std::convert::From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;
        std::io::Error::new(ErrorKind::Other, format!("{}", error))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IOError { error } => f.write_fmt(format_args!("IO Error: {}", error)),
            ErrorKind::InvalidLine { input } => f.write_fmt(format_args!("Invalid row at {}: {}", self.position.row, input)),
            ErrorKind::ParseRow { input } => f.write_fmt(format_args!("Failed to parse row {}: {}", self.position.row, input)),
            ErrorKind::InvalidCommand { command } => f.write_str("meow"),
            ErrorKind::UnknownCommand { command } => f.write_str("meow"),
            ErrorKind::CommandParse { body } => f.write_str("meow"),
        }
    }
}
