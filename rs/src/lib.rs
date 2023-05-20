pub mod hash;
pub mod keys;
pub mod rand;
pub mod size;
pub mod symmetric;
pub mod utils;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Default)]
pub enum ErrorKind {
    #[default]
    Unknown,

    Todo,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind: kind,
            message: message,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            kind: ErrorKind::Unknown,
            message: Default::default(),
        }
    }
}
