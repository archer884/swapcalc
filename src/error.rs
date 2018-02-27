use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

type Message = Cow<'static, str>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: Message,
}

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::Error),
    Data(ParseError),
}

impl Error {
    pub fn io<T: Into<Message>>(error: io::Error, message: T) -> Self {
        Self {
            kind: ErrorKind::Io(error),
            message: message.into(),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::ErrorKind::*;

        match self.kind {
            Data(ref error) => Some(error),
            Io(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        write!(f, "{}", self.message)?;
        if let Some(cause) = self.cause() {
            write!(f, ": {}", cause)?;
        }

        Ok(())
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self {
            kind: ErrorKind::Data(error),
            message: Message::from("Failed to parse"),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingColumn,
    Failure(Box<error::Error>, String),
}

impl ParseError {
    pub fn failure<E, S>(error: E, context: S) -> Self
    where
        E: error::Error + 'static,
        S: Into<String>,
    {
        ParseError::Failure(Box::new(error), context.into())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;

        match *self {
            MissingColumn => write!(f, "Column missing from data"),
            Failure(ref error, ref context) => write!(f, "{} in context:\n  {:?}", error, context),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        use ParseError::*;

        match *self {
            MissingColumn => "Column missing from data",
            Failure(..) => "Failed to parse data",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use ParseError::*;

        match *self {
            MissingColumn => None,
            Failure(ref error, _) => Some(error.as_ref()),
        }
    }
}
