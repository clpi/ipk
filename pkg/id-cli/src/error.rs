use std::{error, io, fmt};

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),    
    TomlSer(toml::ser::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{}", e),
            Self::TomlSer(e) => write!(f, "{}", e)
        } 
    }
}

impl error::Error for CliError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Self::Io(e) => e.source(),
            Self::TomlSer(e) => e.source()
        }
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        Self::Io(e) 
    }
}
impl From<toml::ser::Error> for CliError {
    fn from(e: toml::ser::Error) -> Self {
        Self::TomlSer(e)
    }
}
