use std::{error, io, fmt, path::PathBuf};

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Io(io::Error), 
    Toml(TomlError),
    Pkg(PkgError),
}
#[derive(Debug)]
pub enum PkgError {
    AlreadyExists(PathBuf),
}
#[derive(Debug)]
pub enum TomlError {
    Ser(toml::ser::Error),
    De(toml::de::Error),
}
impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ser(e) => write!(f, "{}", e),
            Self::De(e) => write!(f, "{}", e),
        }
        
    }
}
impl fmt::Display for PkgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists(d) => write!(f, "\x1b[31;1mERROR: Directory {} already contains package", &d.to_str().unwrap()),
        }
        
    }
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pkg(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::Toml(e) => e.fmt(f)
        } 
    }
}

impl error::Error for CliError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Pkg(e) => e.source(),
            Self::Io(e) => e.source(),
            Self::Toml(e) => e.source()
        }
    }
}
impl error::Error for PkgError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::AlreadyExists(d) => None,
        }
    }
}
impl error::Error for TomlError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Ser(e) => e.source(),
            Self::De(e) => e.source()
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
        Self::Toml(TomlError::Ser(e))
    }
}
impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(TomlError::De(e))
    }
}
impl From<TomlError> for CliError {
    fn from(e: TomlError) -> Self {
        Self::Toml(e)
    }
}
impl From<PkgError> for CliError {
    fn from(e: PkgError) -> Self {
        Self::Pkg(e) 
    }
}
