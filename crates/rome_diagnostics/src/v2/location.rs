use std::ops::Deref;

use rome_text_edit::{TextRange, TextSize};
use serde::{Deserialize, Serialize};

/// Represents the location of a diagnostic in a file
#[derive(Debug, Clone, Copy)]
pub struct Location<'a> {
    /// The path of the file this diagnostic is associated with
    pub path: Path<&'a str>,
    /// An optional range of text within the file associated with the diagnostic
    pub span: Option<TextRange>,
    /// The optional source code of the file
    pub source_code: Option<BorrowedSourceCode<'a>>,
}

impl<'a> Location<'a> {
    /// Creates a new instance of [LocationBuilder]
    pub fn builder() -> LocationBuilder<'a> {
        LocationBuilder {
            path: None,
            span: None,
            source_code: None,
        }
    }
}

/// The implementation of [PartialEq] for [Location] only compares the `path`
/// and `span` fields
impl PartialEq for Location<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.span == other.span
    }
}

impl Eq for Location<'_> {}

/// Represents the path of a file associated with a diagnostic
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Path<P> {
    /// The diagnostic is related to the content of the command line arguments
    Argv,
    /// The diagnostic is related to the content of a memory buffer
    Memory,
    /// The diagnostic is related to a file on the filesystem
    File(FilePath<P>),
}

impl<P> Path<P> {
    /// Converts a `Path<P>` to `Path<&P::Target>`
    pub fn as_deref(&self) -> Path<&<P as Deref>::Target>
    where
        P: Deref,
    {
        match self {
            Path::Argv => Path::Argv,
            Path::Memory => Path::Memory,
            Path::File(file) => Path::File(file.as_deref()),
        }
    }
}

impl Path<&'_ str> {
    /// Converts a `Path<&str>` to `Path<String>`
    pub fn to_owned(self) -> Path<String> {
        match self {
            Path::Argv => Path::Argv,
            Path::Memory => Path::Memory,
            Path::File(file) => Path::File(file.to_owned()),
        }
    }
}

/// Represents the path of a file on the filesystem
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FilePath<P> {
    /// The path is represented as a string path
    Path(P),
    /// The path is represented as a [FileId]
    FileId(FileId),
    /// The path is represented as both a string path and a [FileId]
    PathAndId { path: P, file_id: FileId },
}

impl<P> FilePath<P> {
    /// Returns the string path of this [FilePath] if it has one
    pub fn path(self) -> Option<P> {
        match self {
            FilePath::Path(path) => Some(path),
            FilePath::FileId(_) => None,
            FilePath::PathAndId { path, .. } => Some(path),
        }
    }

    /// Returns the [FileId] of this [FilePath] if it has one
    pub fn file_id(self) -> Option<FileId> {
        match self {
            FilePath::Path(_) => None,
            FilePath::FileId(file_id) => Some(file_id),
            FilePath::PathAndId { file_id, .. } => Some(file_id),
        }
    }

    /// Converts a `FilePath<P>` to `FilePath<&P::Target>`
    pub(crate) fn as_deref(&self) -> FilePath<&<P as Deref>::Target>
    where
        P: Deref,
    {
        match self {
            FilePath::Path(path) => FilePath::Path(path),
            FilePath::FileId(file_id) => FilePath::FileId(*file_id),
            FilePath::PathAndId { path, file_id } => FilePath::PathAndId {
                path,
                file_id: *file_id,
            },
        }
    }

    /// Returns the "logical or" of `self` and `other`, trying to merge the
    /// [FilePath::Path] and [FilePath::FileId] variants into [FilePath::PathAndId]
    pub(crate) fn or(self, other: Self) -> Self
    where
        P: PartialEq,
    {
        match (self, other) {
            (FilePath::Path(path), FilePath::FileId(file_id)) => {
                FilePath::PathAndId { path, file_id }
            }

            (FilePath::FileId(file_id), FilePath::Path(path)) => {
                FilePath::PathAndId { path, file_id }
            }

            (FilePath::Path(inner_path), FilePath::PathAndId { path, file_id })
                if inner_path == path =>
            {
                FilePath::PathAndId { path, file_id }
            }

            (FilePath::FileId(inner_id), FilePath::PathAndId { path, file_id })
                if inner_id == file_id =>
            {
                FilePath::PathAndId { path, file_id }
            }

            (file, _) => file,
        }
    }
}

impl FilePath<&'_ str> {
    /// Converts a `FilePath<P>` to `FilePath<&P::Target>`
    pub fn to_owned(self) -> FilePath<String> {
        match self {
            FilePath::Path(path) => FilePath::Path(path.to_string()),
            FilePath::FileId(file_id) => FilePath::FileId(file_id),
            FilePath::PathAndId { path, file_id } => FilePath::PathAndId {
                path: path.to_string(),
                file_id,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
/// An id that points into a file database.
pub struct FileId(usize);

impl FileId {
    pub const fn zero() -> Self {
        Self(0)
    }
}

impl From<usize> for FileId {
    fn from(input: usize) -> Self {
        Self(input)
    }
}

impl From<FileId> for usize {
    fn from(input: FileId) -> Self {
        input.0
    }
}

type OwnedSourceCode = SourceCode<String, Vec<TextSize>>;
type BorrowedSourceCode<'a> = SourceCode<&'a str, &'a [TextSize]>;

/// Represents the source code of a file
#[derive(Debug, Clone, Copy)]
pub struct SourceCode<T, L> {
    /// The text content of the file
    pub text: T,
    /// An optional "line index" for the file, a list of byte offsets for the
    /// start of each line in the file
    pub line_starts: Option<L>,
}

impl<T, L> SourceCode<T, L> {
    /// Converts a `SourceCode<T, L>` to `SourceCode<&T::Target, &L::Target>`
    pub(crate) fn as_deref(&self) -> SourceCode<&<T as Deref>::Target, &<L as Deref>::Target>
    where
        T: Deref,
        L: Deref,
    {
        SourceCode {
            text: &self.text,
            line_starts: self.line_starts.as_deref(),
        }
    }
}

impl BorrowedSourceCode<'_> {
    /// Converts a `SourceCode<&str, &[TextSize]>` to `SourceCode<String, Vec<TextSize>>`
    pub(crate) fn to_owned(self) -> OwnedSourceCode {
        SourceCode {
            text: self.text.to_owned(),
            line_starts: self.line_starts.map(ToOwned::to_owned),
        }
    }
}

/// Builder type for the [Location] struct
pub struct LocationBuilder<'a> {
    path: Option<Path<&'a str>>,
    span: Option<TextRange>,
    source_code: Option<BorrowedSourceCode<'a>>,
}

impl<'a> LocationBuilder<'a> {
    pub fn path<P: AsPath>(mut self, path: &'a P) -> Self {
        self.path = path.as_path();
        self
    }

    pub fn span<S: AsSpan>(mut self, span: &'a S) -> Self {
        self.span = span.as_span();
        self
    }

    pub fn source_code<S: AsSourceCode>(mut self, source_code: &'a S) -> Self {
        self.source_code = source_code.as_source_code();
        self
    }

    pub fn build(self) -> Option<Location<'a>> {
        let path = self.path?;
        Some(Location {
            path,
            span: self.span,
            source_code: self.source_code,
        })
    }
}

/// Utility trait for types that can be converted to a [Path]
pub trait AsPath {
    fn as_path(&self) -> Option<Path<&'_ str>>;
}

impl<T: AsPath> AsPath for Option<T> {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        self.as_ref().and_then(T::as_path)
    }
}

impl<T: AsPath + ?Sized> AsPath for &'_ T {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        T::as_path(*self)
    }
}

impl<T: Deref<Target = str>> AsPath for Path<T> {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        Some(self.as_deref())
    }
}

impl<T: Deref<Target = str>> AsPath for FilePath<T> {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        Some(Path::File(self.as_deref()))
    }
}

// Returns the equivalent of A || B, primarily intended for writing the path of
// a diagnostic as `(String, FileId)`
impl<A: AsPath, B: AsPath> AsPath for (A, B) {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        match (self.0.as_path(), self.1.as_path()) {
            (Some(Path::File(a)), Some(Path::File(b))) => Some(Path::File(a.or(b))),
            (Some(a), Some(_b)) => Some(a),
            (Some(path), None) | (None, Some(path)) => Some(path),
            (None, None) => None,
        }
    }
}

impl AsPath for String {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        Some(Path::File(FilePath::Path(self)))
    }
}

impl AsPath for str {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        Some(Path::File(FilePath::Path(self)))
    }
}

impl AsPath for FileId {
    fn as_path(&self) -> Option<Path<&'_ str>> {
        Some(Path::File(FilePath::FileId(*self)))
    }
}

/// Utility trait for types that can be converted into `Option<TextRange>`
pub trait AsSpan {
    fn as_span(&self) -> Option<TextRange>;
}

impl<T: AsSpan> AsSpan for Option<T> {
    fn as_span(&self) -> Option<TextRange> {
        self.as_ref().and_then(T::as_span)
    }
}

impl<T: AsSpan + ?Sized> AsSpan for &'_ T {
    fn as_span(&self) -> Option<TextRange> {
        T::as_span(*self)
    }
}

impl AsSpan for TextRange {
    fn as_span(&self) -> Option<TextRange> {
        Some(*self)
    }
}

/// Utility trait for types that can be converted into [SourceCode]
pub trait AsSourceCode {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>>;
}

impl<T: AsSourceCode> AsSourceCode for Option<T> {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        self.as_ref().and_then(T::as_source_code)
    }
}

impl<T: AsSourceCode + ?Sized> AsSourceCode for &'_ T {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        T::as_source_code(*self)
    }
}

impl AsSourceCode for BorrowedSourceCode<'_> {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        Some(*self)
    }
}

impl AsSourceCode for OwnedSourceCode {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        Some(SourceCode {
            text: self.text.as_str(),
            line_starts: self.line_starts.as_deref(),
        })
    }
}

impl AsSourceCode for str {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        Some(SourceCode {
            text: self,
            line_starts: None,
        })
    }
}

impl AsSourceCode for String {
    fn as_source_code(&self) -> Option<BorrowedSourceCode<'_>> {
        Some(SourceCode {
            text: self,
            line_starts: None,
        })
    }
}
