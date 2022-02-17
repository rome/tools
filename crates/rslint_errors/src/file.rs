use rome_rowan::{Language, SyntaxElement, SyntaxNode, SyntaxToken, TextRange};
use std::{collections::HashMap, ops::Range};

/// A value which can be used as the range inside of a diagnostic.
///
/// This is essentially a hack to allow us to use SyntaxElement, SyntaxNode, etc directly
pub trait Span {
    fn as_range(&self) -> Range<usize>;

    fn as_text_range(&self) -> TextRange {
        TextRange::new(
            (self.as_range().start as u32).into(),
            (self.as_range().end as u32).into(),
        )
    }

    /// Make a new span which extends to another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^^^
    /// ```
    fn join<T: Span>(&self, other: T) -> Range<usize> {
        self.as_range().start..other.as_range().end
    }

    /// Make a new span which is between another span
    ///
    /// ```text
    /// from      to
    ///     ^^^^^^
    /// ```
    fn between<T: Span>(&self, other: T) -> Range<usize> {
        self.as_range().end..other.as_range().start
    }

    /// Make a new span which extends until another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^
    /// ```
    fn until<T: Span>(&self, other: T) -> Range<usize> {
        self.as_range().start..other.as_range().start
    }

    fn sub_start(&self, amount: usize) -> Range<usize> {
        let range = self.as_range();
        range.start - amount..range.end
    }

    fn add_start(&self, amount: usize) -> Range<usize> {
        let range = self.as_range();
        range.start + amount..range.end
    }

    fn sub_end(&self, amount: usize) -> Range<usize> {
        let range = self.as_range();
        range.start..range.end - amount
    }

    fn add_end(&self, amount: usize) -> Range<usize> {
        let range = self.as_range();
        range.start..range.end + amount
    }
}

impl<T: Span> Span for &T {
    fn as_range(&self) -> Range<usize> {
        (*self).as_range()
    }
}

impl<T: Span> Span for &mut T {
    fn as_range(&self) -> Range<usize> {
        (**self).as_range()
    }
}

impl<T: Clone> Span for Range<T>
where
    T: Into<usize>,
{
    fn as_range(&self) -> Range<usize> {
        self.start.clone().into()..self.end.clone().into()
    }
}

impl<T: Language> Span for SyntaxNode<T> {
    fn as_range(&self) -> Range<usize> {
        self.text_range().into()
    }
}

impl<T: Language> Span for SyntaxToken<T> {
    fn as_range(&self) -> Range<usize> {
        self.text_range().into()
    }
}

impl<T: Language> Span for SyntaxElement<T> {
    fn as_range(&self) -> Range<usize> {
        match self {
            SyntaxElement::Node(n) => n.text_range(),
            SyntaxElement::Token(t) => t.text_range(),
        }
        .into()
    }
}

impl Span for TextRange {
    fn as_range(&self) -> Range<usize> {
        (*self).into()
    }
}

/// An id that points into a file database.
pub type FileId = usize;

/// A range that is indexed in a specific file.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileSpan {
    pub file: FileId,
    pub range: Range<usize>,
}

impl FileSpan {
    pub fn new(file: FileId, span: impl Span) -> Self {
        let range = span.as_range();
        debug_assert!(
            range.start <= range.end,
            "slice index starts at {} but ends at {}",
            range.start,
            range.end
        );

        Self { file, range }
    }
}

/// Interface for interacting with source files
/// that are identified by a unique identifier.
pub trait Files {
    /// Returns the name of the file identified by the id.
    fn name(&self, id: FileId) -> Option<&str>;

    /// Returns the source of the file identified by the id.
    fn source(&self, id: FileId) -> Option<&str>;

    /// The index of the line at the byte index.
    ///
    /// ## Implementation
    /// This can be implemented by caching the results of [`line_starts`]
    /// and then use [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
    /// to compute the line index.
    ///
    /// ```ignore
    /// match self.line_starts.binary_search(byte_index) {
    ///     Ok(line) => line,
    ///     Err(next_line) => next_line - 1,
    /// }
    /// ```
    fn line_index(&self, file_id: FileId, byte_index: usize) -> Option<usize>;

    /// The byte range of line in the source of the file.
    fn line_range(&self, id: FileId, line_index: usize) -> Option<Range<usize>>;
}

/// A file database that contains only one file.
#[derive(Clone, Debug)]
pub struct SimpleFile {
    name: String,
    source: String,
    line_starts: Vec<usize>,
}

impl SimpleFile {
    /// Create a new file with the name and source.
    pub fn new(name: String, source: String) -> Self {
        Self {
            line_starts: line_starts(&source).collect(),
            name,
            source,
        }
    }

    /// Returns a `SimpleFile` that has no name and no source.
    pub fn empty() -> SimpleFile {
        SimpleFile::new(String::new(), String::new())
    }

    fn line_start(&self, line_index: usize) -> Option<usize> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => self.line_starts.get(line_index).cloned(),
            Ordering::Equal => Some(self.source.len()),
            Ordering::Greater => None,
        }
    }
}

impl Files for SimpleFile {
    fn name(&self, _id: FileId) -> Option<&str> {
        Some(&self.name)
    }

    fn source(&self, _id: FileId) -> Option<&str> {
        Some(&self.source)
    }

    fn line_index(&self, _file_id: FileId, byte_index: usize) -> Option<usize> {
        Some(
            self.line_starts
                .binary_search(&byte_index)
                .unwrap_or_else(|next_line| next_line - 1),
        )
    }

    fn line_range(&self, _: FileId, line_index: usize) -> Option<Range<usize>> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Some(line_start..next_line_start)
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

    fn source(&self, id: FileId) -> Option<&str> {
        self.files.get(&id)?.source(id)
    }

    fn line_index(&self, id: FileId, byte_index: usize) -> Option<usize> {
        self.files.get(&id)?.line_index(id, byte_index)
    }

    fn line_range(&self, file_id: FileId, line_index: usize) -> Option<Range<usize>> {
        self.files.get(&file_id)?.line_range(file_id, line_index)
    }
}

/// Computes the byte indicies of every line start.
pub fn line_starts(source: &str) -> impl '_ + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices(&['\n', '\r']).filter_map(|(i, _)| {
        let bytes = source.as_bytes();

        match bytes[i] {
            // Filter out the `\r` in `\r\n` to avoid counting the line break twice
            b'\r' if i + 1 < bytes.len() && bytes[i + 1] == b'\n' => None,
            _ => Some(i + 1),
        }
    }))
}

#[cfg(test)]
mod tests {
    use crate::file::line_starts;

    #[test]
    fn line_starts_with_carriage_return_line_feed() {
        let input = "a\r\nb\r\nc";
        let starts = line_starts(input).collect::<Vec<_>>();

        assert_eq!(vec![0, 3, 6], starts);
    }

    #[test]
    fn line_starts_with_carriage_return() {
        let input = "a\rb\rc";
        let starts = line_starts(input).collect::<Vec<_>>();

        assert_eq!(vec![0, 2, 4], starts);
    }

    #[test]
    fn line_starts_with_line_feed() {
        let input = "a\nb\nc";
        let starts = line_starts(input).collect::<Vec<_>>();

        assert_eq!(vec![0, 2, 4], starts);
    }
}
