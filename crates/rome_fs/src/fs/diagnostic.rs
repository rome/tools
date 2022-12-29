use rome_diagnostics::{console, Advices, Diagnostic, LogCategory, Visit};
use std::io;

#[derive(Debug, Diagnostic)]
#[diagnostic(severity = Warning, category = "internalError/fs")]
pub(crate) struct FileSystemDiagnostic {
    #[location(resource)]
    pub(crate) path: String,
    #[message]
    #[description]
    #[advice]
    pub(crate) error_kind: ErrorKind,
}

#[derive(Clone, Debug)]
pub(crate) enum ErrorKind {
    /// Unknown file type
    UnknownFileType,
    /// Dereferenced (broken) symbolic link
    DereferencedSymlink(String),
    /// Symbolic link cycle or symbolic link infinite expansion
    InfiniteSymlinkExpansion(String),
}

impl console::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut console::fmt::Formatter) -> io::Result<()> {
        match self {
            ErrorKind::UnknownFileType => fmt.write_str("Unknown file type"),
            ErrorKind::DereferencedSymlink(_) => fmt.write_str("Dereferenced symlink"),
            ErrorKind::InfiniteSymlinkExpansion(_) => fmt.write_str("Infinite symlink expansion"),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnknownFileType => write!(fmt, "Unknown file type"),
            ErrorKind::DereferencedSymlink(_) => write!(fmt, "Dereferenced symlink"),
            ErrorKind::InfiniteSymlinkExpansion(_) => write!(fmt, "Infinite symlink expansion"),
        }
    }
}

impl Advices for ErrorKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
            ErrorKind::UnknownFileType => visitor.record_log(
                LogCategory::Info,
                &"Rome encountered a file system entry that's neither a file, directory or symbolic link",
            ),
            ErrorKind::DereferencedSymlink(path) => visitor.record_log(
                LogCategory::Info,
                &format!("Rome encountered a file system entry that is a broken symbolic link: {}", path),
            ),
            ErrorKind::InfiniteSymlinkExpansion(path) => visitor.record_log(
                LogCategory::Error,
                &format!("Rome encountered a file system entry that leads to an infinite symbolic link expansion, causing an infinite cycle: {}", path),
            ),
        }
    }
}
