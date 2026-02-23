use std::fmt;
use std::path::PathBuf;

pub enum EditorResponse {
    TerminalInitializationFailed,
}

pub enum EditorWithArgsResponse{
    ParentDirDoesNotExist(PathBuf),
}

pub enum BufferResponse {
    NoFilenameSet,
}

impl fmt::Display for EditorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TerminalInitializationFailed => {
                write!(f, "Failed to initialize terminal")
            }
        }
    }
}

impl fmt::Display for EditorWithArgsResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            Self::ParentDirDoesNotExist(parent) => {
                write!(f, "Error: The parent directory '{}' does not exist.", parent.to_string_lossy())
            }
        }
    }
}

impl fmt::Display for BufferResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoFilenameSet => {
                write!(f, "No filename set. Use 'Save As' to specify a filename.")
            }
        }
    }
}
