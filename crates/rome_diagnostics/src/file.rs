use rome_console::codespan::SourceFile;
use rome_rowan::{Language, SyntaxElement, SyntaxNode, SyntaxToken, TextRange, TextSize};
use std::{collections::HashMap, fmt::Debug, ops::Range};

/// A value which can be used as the range inside of a diagnostic.
///
/// This is essentially a hack to allow us to use SyntaxElement, SyntaxNode, etc directly
pub trait Span {
    fn as_range(&self) -> TextRange;

    /// Make a new span which extends to another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^^^
    /// ```
    fn join<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().start(), other.as_range().end())
    }

    /// Make a new span which is between another span
    ///
    /// ```text
    /// from      to
    ///     ^^^^^^
    /// ```
    fn between<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().end(), other.as_range().start())
    }

    /// Make a new span which extends until another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^
    /// ```
    fn until<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().start(), other.as_range().start())
    }

    fn sub_start(&self, amount: TextSize) -> TextRange {
        let range = self.as_range();
        TextRange::new(range.start() - amount, range.end())
    }

    fn add_start(&self, amount: TextSize) -> TextRange {
        let range = self.as_range();
        TextRange::new(range.start() + amount, range.end())
    }

    fn sub_end(&self, amount: TextSize) -> TextRange {
        let range = self.as_range();
        TextRange::new(range.start(), range.end() - amount)
    }

    fn add_end(&self, amount: TextSize) -> TextRange {
        let range = self.as_range();
        TextRange::new(range.start(), range.end() + amount)
    }
}

impl<T: Span> Span for &T {
    fn as_range(&self) -> TextRange {
        (*self).as_range()
    }
}

impl<T: Span> Span for &mut T {
    fn as_range(&self) -> TextRange {
        (**self).as_range()
    }
}

impl<T: Copy> Span for Range<T>
where
    TextSize: TryFrom<T>,
    <TextSize as TryFrom<T>>::Error: Debug,
{
    fn as_range(&self) -> TextRange {
        TextRange::new(
            TextSize::try_from(self.start).expect("integer overflow"),
            TextSize::try_from(self.end).expect("integer overflow"),
        )
    }
}

impl<T: Language> Span for SyntaxNode<T> {
    fn as_range(&self) -> TextRange {
        self.text_range()
    }
}

impl<T: Language> Span for SyntaxToken<T> {
    fn as_range(&self) -> TextRange {
        self.text_range()
    }
}

impl<T: Language> Span for SyntaxElement<T> {
    fn as_range(&self) -> TextRange {
        match self {
            SyntaxElement::Node(n) => n.text_range(),
            SyntaxElement::Token(t) => t.text_range(),
        }
    }
}

impl Span for TextRange {
    fn as_range(&self) -> TextRange {
        *self
    }
}

/// An id that points into a file database.
pub type FileId = usize;

/// A range that is indexed in a specific file.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct FileSpan {
    pub file: FileId,
    #[cfg_attr(feature = "serde", schemars(with = "rome_rowan::TextRangeSchema"))]
    pub range: TextRange,
}

impl FileSpan {
    pub fn new(file: FileId, span: impl Span) -> Self {
        let range = span.as_range();
        Self { file, range }
    }
}

/// Interface for interacting with source files
/// that are identified by a unique identifier.
pub trait Files {
    /// Returns the name of the file identified by the id.
    fn name(&self, id: FileId) -> Option<&str>;

    /// Returns the source of the file identified by the id.
    fn source(&self, id: FileId) -> Option<SourceFile<'_>>;
}

/// A file database that contains only one file.
#[derive(Clone, Debug)]
pub struct SimpleFile {
    name: String,
    source: String,
    line_starts: Vec<TextSize>,
}

impl SimpleFile {
    /// Create a new file with the name and source.
    pub fn new(name: String, source: String) -> Self {
        Self {
            line_starts: SourceFile::line_starts(&source).collect(),
            name,
            source,
        }
    }

    /// Returns a `SimpleFile` that has no name and no source.
    pub fn empty() -> SimpleFile {
        SimpleFile::new(String::new(), String::new())
    }
}

impl Files for SimpleFile {
    fn name(&self, _id: FileId) -> Option<&str> {
        Some(&self.name)
    }

    fn source(&self, _id: FileId) -> Option<SourceFile<'_>> {
        Some(SourceFile::new(&self.source, &self.line_starts))
    }
}

/// A file database that stores multiple files.
#[derive(Clone, Debug, Default)]
pub struct SimpleFiles {
    files: HashMap<FileId, SimpleFile>,
    id: usize,
}

impl SimpleFiles {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a file to this database and returns the id for the new file.
    pub fn add(&mut self, name: String, source: String) -> FileId {
        let id = self.id;
        self.id += 1;
        self.files.insert(id, SimpleFile::new(name, source));
        id
    }

    pub fn get(&self, id: FileId) -> Option<&SimpleFile> {
        self.files.get(&id)
    }
}

impl Files for SimpleFiles {
    fn name(&self, id: FileId) -> Option<&str> {
        self.files.get(&id)?.name(id)
    }

    fn source(&self, id: FileId) -> Option<SourceFile<'_>> {
        self.files.get(&id)?.source(id)
    }
}
