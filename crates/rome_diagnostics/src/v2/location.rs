use rome_text_size::{TextRange, TextSize};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Range;
use std::{borrow::Borrow, ops::Deref};

/// Represents the location of a diagnostic in a resource.
#[derive(Debug, Clone, Copy)]
pub struct Location<'a> {
    /// The resource this diagnostic is associated with.
    pub resource: Resource<&'a str>,
    /// An optional range of text within the resource associated with the
    /// diagnostic.
    pub span: Option<TextRange>,
    /// The optional source code of the resource.
    pub source_code: Option<BorrowedSourceCode<'a>>,
}

impl<'a> Location<'a> {
    /// Creates a new instance of [LocationBuilder].
    pub fn builder() -> LocationBuilder<'a> {
        LocationBuilder {
            resource: None,
            span: None,
            source_code: None,
        }
    }
}

/// The implementation of [PartialEq] for [Location] only compares the `path`
/// and `span` fields
impl PartialEq for Location<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource && self.span == other.span
    }
}

impl Eq for Location<'_> {}

/// Represents the resource a diagnostic is associated with.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Resource<P> {
    /// The diagnostic is related to the content of the command line arguments.
    Argv,
    /// The diagnostic is related to the content of a memory buffer.
    Memory,
    /// The diagnostic is related to a file on the filesystem.
    File(FilePath<P>),
}

impl<P> Resource<P> {
    /// Returns a `FilePath<&P::Target>` if `self` points to a `Path`, or
    /// `None` otherwise.
    pub fn as_file(&self) -> Option<FilePath<&<P as Deref>::Target>>
    where
        P: Deref,
    {
        if let Resource::File(file) = self {
            Some(file.as_deref())
        } else {
            None
        }
    }

    /// Converts a `Path<P>` to `Path<&P::Target>`.
    pub fn as_deref(&self) -> Resource<&<P as Deref>::Target>
    where
        P: Deref,
    {
        match self {
            Resource::Argv => Resource::Argv,
            Resource::Memory => Resource::Memory,
            Resource::File(file) => Resource::File(file.as_deref()),
        }
    }
}

impl Resource<&'_ str> {
    /// Converts a `Path<&str>` to `Path<String>`.
    pub fn to_owned(self) -> Resource<String> {
        match self {
            Resource::Argv => Resource::Argv,
            Resource::Memory => Resource::Memory,
            Resource::File(file) => Resource::File(file.to_owned()),
        }
    }
}

/// Represents the path of a file on the filesystem.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FilePath<P> {
    /// The path is represented as a string path.
    Path(P),
    /// The path is represented as a [FileId].
    FileId(FileId),
    /// The path is represented as both a string path and a [FileId].
    PathAndId { path: P, file_id: FileId },
}

impl<P> FilePath<P> {
    /// Returns the string path of this [FilePath] if it has one.
    pub fn path(self) -> Option<P> {
        match self {
            FilePath::Path(path) => Some(path),
            FilePath::FileId(_) => None,
            FilePath::PathAndId { path, .. } => Some(path),
        }
    }

    /// Returns the [FileId] of this [FilePath] if it has one.
    pub fn file_id(self) -> Option<FileId> {
        match self {
            FilePath::Path(_) => None,
            FilePath::FileId(file_id) => Some(file_id),
            FilePath::PathAndId { file_id, .. } => Some(file_id),
        }
    }

    /// Converts a `FilePath<P>` to `FilePath<&P::Target>`.
    pub fn as_deref(&self) -> FilePath<&<P as Deref>::Target>
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
    /// [FilePath::Path] and [FilePath::FileId] variants into [FilePath::PathAndId].
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
    /// Converts a `FilePath<P>` to `FilePath<&P::Target>`.
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

type OwnedSourceCode = SourceCode<String, LineIndexBuf>;
pub(crate) type BorrowedSourceCode<'a> = SourceCode<&'a str, &'a LineIndex>;

/// Represents the source code of a file.
#[derive(Debug, Clone, Copy)]
pub struct SourceCode<T, L> {
    /// The text content of the file.
    pub text: T,
    /// An optional "line index" for the file, a list of byte offsets for the
    /// start of each line in the file.
    pub line_starts: Option<L>,
}

impl<T, L> SourceCode<T, L> {
    /// Converts a `SourceCode<T, L>` to `SourceCode<&T::Target, &L::Target>`.
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
    /// Converts a `SourceCode<&str, &LineIndex>` to `SourceCode<String, LineIndexBuf>`.
    pub(crate) fn to_owned(self) -> OwnedSourceCode {
        SourceCode {
            text: self.text.to_owned(),
            line_starts: self.line_starts.map(ToOwned::to_owned),
        }
    }
}

#[derive(Debug)]
pub struct LineIndex([TextSize]);

impl LineIndex {
    pub fn new(slice: &'_ [TextSize]) -> &'_ Self {
        // SAFETY: Transmuting `&[TextSize]` to `&LineIndex` is safe since
        // `LineIndex` is a `repr(transparent)` struct containing a `[TextSize]`
        // and thus has the same memory layout
        unsafe { std::mem::transmute(slice) }
    }
}

impl Deref for LineIndex {
    type Target = [TextSize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToOwned for LineIndex {
    type Owned = LineIndexBuf;

    fn to_owned(&self) -> Self::Owned {
        LineIndexBuf(self.0.to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct LineIndexBuf(Vec<TextSize>);

impl LineIndexBuf {
    pub fn from_source_text(source: &str) -> Self {
        Self(
            std::iter::once(0)
                .chain(source.match_indices(&['\n', '\r']).filter_map(|(i, _)| {
                    let bytes = source.as_bytes();

                    match bytes[i] {
                        // Filter out the `\r` in `\r\n` to avoid counting the line break twice
                        b'\r' if i + 1 < bytes.len() && bytes[i + 1] == b'\n' => None,
                        _ => Some(i + 1),
                    }
                }))
                .map(|i| TextSize::try_from(i).expect("integer overflow"))
                .collect(),
        )
    }
}

impl Deref for LineIndexBuf {
    type Target = LineIndex;

    fn deref(&self) -> &Self::Target {
        LineIndex::new(self.0.as_slice())
    }
}

impl Borrow<LineIndex> for LineIndexBuf {
    fn borrow(&self) -> &LineIndex {
        self
    }
}

/// Builder type for the [Location] struct
pub struct LocationBuilder<'a> {
    resource: Option<Resource<&'a str>>,
    span: Option<TextRange>,
    source_code: Option<BorrowedSourceCode<'a>>,
}

impl<'a> LocationBuilder<'a> {
    pub fn resource<P: AsResource>(mut self, resource: &'a P) -> Self {
        self.resource = resource.as_resource();
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
        Some(Location {
            resource: self.resource?,
            span: self.span,
            source_code: self.source_code,
        })
    }
}

/// Utility trait for types that can be converted to a [Resource]
pub trait AsResource {
    fn as_resource(&self) -> Option<Resource<&'_ str>>;
}

impl<T: AsResource> AsResource for Option<T> {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        self.as_ref().and_then(T::as_resource)
    }
}

impl<T: AsResource + ?Sized> AsResource for &'_ T {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        T::as_resource(*self)
    }
}

impl<T: Deref<Target = str>> AsResource for Resource<T> {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        Some(self.as_deref())
    }
}

impl<T: Deref<Target = str>> AsResource for FilePath<T> {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        Some(Resource::File(self.as_deref()))
    }
}

// Returns the equivalent of A || B, primarily intended for writing the path of
// a diagnostic as `(String, FileId)`
impl<A: AsResource, B: AsResource> AsResource for (A, B) {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        match (self.0.as_resource(), self.1.as_resource()) {
            (Some(Resource::File(a)), Some(Resource::File(b))) => Some(Resource::File(a.or(b))),
            (Some(a), Some(_b)) => Some(a),
            (Some(path), None) | (None, Some(path)) => Some(path),
            (None, None) => None,
        }
    }
}

impl AsResource for String {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        Some(Resource::File(FilePath::Path(self)))
    }
}

impl AsResource for str {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        Some(Resource::File(FilePath::Path(self)))
    }
}

impl AsResource for FileId {
    fn as_resource(&self) -> Option<Resource<&'_ str>> {
        Some(Resource::File(FilePath::FileId(*self)))
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

impl<T: Copy> AsSpan for Range<T>
where
    TextSize: TryFrom<T>,
    <TextSize as TryFrom<T>>::Error: Debug,
{
    fn as_span(&self) -> Option<TextRange> {
        Some(TextRange::new(
            TextSize::try_from(self.start).expect("integer overflow"),
            TextSize::try_from(self.end).expect("integer overflow"),
        ))
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

#[cfg(test)]
mod tests {
    use rome_text_size::TextSize;

    use super::LineIndexBuf;

    #[test]
    fn line_starts_with_carriage_return_line_feed() {
        let input = "a\r\nb\r\nc";
        let LineIndexBuf(starts) = LineIndexBuf::from_source_text(input);

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(3u32),
                TextSize::from(6u32)
            ],
            starts
        );
    }

    #[test]
    fn line_starts_with_carriage_return() {
        let input = "a\rb\rc";
        let LineIndexBuf(starts) = LineIndexBuf::from_source_text(input);

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(2u32),
                TextSize::from(4u32)
            ],
            starts
        );
    }

    #[test]
    fn line_starts_with_line_feed() {
        let input = "a\nb\nc";
        let LineIndexBuf(starts) = LineIndexBuf::from_source_text(input);

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(2u32),
                TextSize::from(4u32)
            ],
            starts
        );
    }
}
