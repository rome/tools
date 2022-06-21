use crate::{GroupId, TextSize};
#[cfg(target_pointer_width = "64")]
use rome_rowan::static_assert;
use rome_rowan::SyntaxTokenText;
use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

type Content = Box<FormatElement>;

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [crate::space_token], [crate::soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq)]
pub enum FormatElement {
    /// A space token, see [crate::space_token] for documentation.
    Space,

    /// A new line, see [crate::soft_line_break], [crate::hard_line_break], and [crate::soft_line_break_or_space] for documentation.
    Line(LineMode),

    /// Indents the content one level deeper, see [crate::indent] for documentation and examples.
    Indent(Content),

    /// Creates a logical group where its content is either consistently printed:
    /// * on a single line: Omitting `LineMode::Soft` line breaks and printing spaces for `LineMode::SoftOrSpace`
    /// * on multiple lines: Printing all line breaks
    ///
    /// See [crate::group_elements] for documentation and examples.
    Group(Group),

    /// Forces the parent group to print in expanded mode.
    ExpandParent,

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [crate::if_group_breaks] for examples.
    ConditionalGroupContent(ConditionalGroupContent),

    /// Concatenates multiple elements together. See [concat_elements] and [join_elements] for examples.
    List(List),

    /// Concatenates multiple elements together with a given separator printed in either
    /// flat or expanded mode to fill the print width. See [fill_elements].
    Fill(Box<Fill>),

    /// A token that should be printed as is, see [token] for documentation and examples.
    Token(Token),

    /// Delay the printing of its content until the next line break
    LineSuffix(Content),

    /// Prevents that line suffixes move past this boundary. Forces the printer to print any pending
    /// line suffixes, potentially by inserting a hard line break.
    LineSuffixBoundary,

    /// Special semantic element letting the printer and formatter know this is
    /// a comment content, and it should only have a limited influence on the
    /// formatting (for instance line breaks contained within will not cause
    /// the parent group to break if this element is at the start of it).
    Comment(Box<[FormatElement]>),

    /// A token that tracks tokens/nodes that are printed using [`format_verbatim`](crate::Formatter::format_verbatim) API
    Verbatim(Verbatim),

    /// A list of different variants representing the same content. The printer picks the best fitting content.
    /// Line breaks inside of a best fitting don't propagate to parent groups.
    BestFitting(BestFitting),

    /// An interned format element. Useful when the same content must be emitted multiple times to avoid
    /// deep cloning the IR when using the `best_fitting!` macro or `if_group_fits_on_line` and `if_group_breaks`.
    Interned(Interned),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum VerbatimKind {
    Unknown,
    Suppressed,
    Verbatim {
        /// the length of the formatted node
        length: TextSize,
    },
}

/// Information of the node/token formatted verbatim
#[derive(Clone, Eq, PartialEq)]
pub struct Verbatim {
    /// The reason this range is using verbatim formatting
    pub kind: VerbatimKind,
    /// The [FormatElement] version of the node/token
    pub element: Box<FormatElement>,
}

impl Verbatim {
    pub fn new_verbatim(element: FormatElement, length: TextSize) -> Self {
        Self {
            element: Box::new(element),
            kind: VerbatimKind::Verbatim { length },
        }
    }

    pub fn new_unknown(element: FormatElement) -> Self {
        Self {
            element: Box::new(element),
            kind: VerbatimKind::Unknown,
        }
    }

    pub fn new_suppressed(element: FormatElement) -> Self {
        Self {
            element: Box::new(element),
            kind: VerbatimKind::Suppressed,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self.kind, VerbatimKind::Unknown)
    }
}

impl Debug for FormatElement {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            FormatElement::Space => write!(fmt, "Space"),
            FormatElement::Line(content) => fmt.debug_tuple("Line").field(content).finish(),
            FormatElement::Indent(content) => fmt.debug_tuple("Indent").field(content).finish(),
            FormatElement::Group(content) => {
                write!(fmt, "Group")?;
                content.fmt(fmt)
            }
            FormatElement::ConditionalGroupContent(content) => content.fmt(fmt),
            FormatElement::List(content) => {
                write!(fmt, "List ")?;
                content.fmt(fmt)
            }
            FormatElement::Fill(fill) => fill.fmt(fmt),
            FormatElement::Token(content) => content.fmt(fmt),
            FormatElement::LineSuffix(content) => {
                fmt.debug_tuple("LineSuffix").field(content).finish()
            }
            FormatElement::LineSuffixBoundary => write!(fmt, "LineSuffixBoundary"),
            FormatElement::Comment(content) => fmt.debug_tuple("Comment").field(content).finish(),
            FormatElement::Verbatim(verbatim) => fmt
                .debug_tuple("Verbatim")
                .field(&verbatim.element)
                .finish(),
            FormatElement::BestFitting(best_fitting) => {
                write!(fmt, "BestFitting")?;
                best_fitting.fmt(fmt)
            }
            FormatElement::ExpandParent => write!(fmt, "ExpandParent"),
            FormatElement::Interned(inner) => inner.fmt(fmt),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
    /// See [soft_line_break_or_space] for documentation.
    SoftOrSpace,
    /// See [soft_line_break] for documentation.
    Soft,
    /// See [hard_line_break] for documentation.
    Hard,
    /// See [empty_line] for documentation.
    Empty,
}

/// A token used to gather a list of elements; see [concat_elements] and [join_elements].
#[derive(Clone, Default, Eq, PartialEq)]
pub struct List {
    content: Vec<FormatElement>,
}

impl Debug for List {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_list().entries(&self.content).finish()
    }
}

impl List {
    pub(crate) fn new(content: Vec<FormatElement>) -> Self {
        Self { content }
    }

    pub fn into_vec(self) -> Vec<FormatElement> {
        self.content
    }
}

impl Deref for List {
    type Target = [FormatElement];

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

/// Fill is a list of [FormatElement]s along with a separator.
///
/// The printer prints this list delimited by a separator, wrapping the list when it
/// reaches the specified `line_width`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fill {
    pub(super) list: List,
    pub(super) separator: FormatElement,
}

impl Fill {
    pub fn list(&self) -> &List {
        &self.list
    }

    pub fn separator(&self) -> &FormatElement {
        &self.separator
    }
}

/// Group is a special token that controls how the child tokens are printed.
///
/// The printer first tries to print all tokens in the group onto a single line (ignoring soft line wraps)
/// but breaks the array cross multiple lines if it would exceed the specified `line_width`, if a child token is a hard line break or if a string contains a line break.
#[derive(Clone, PartialEq, Eq)]
pub struct Group {
    pub(crate) content: Box<[FormatElement]>,
    pub(crate) id: Option<GroupId>,
}

impl Debug for Group {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(id) = &self.id {
            fmt.debug_struct("")
                .field("content", &self.content)
                .field("id", id)
                .finish()
        } else {
            fmt.debug_tuple("").field(&self.content).finish()
        }
    }
}

impl Group {
    pub fn new(content: Vec<FormatElement>) -> Self {
        Self {
            content: content.into_boxed_slice(),
            id: None,
        }
    }

    pub fn with_id(mut self, id: Option<GroupId>) -> Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrintMode {
    /// Omits any soft line breaks
    Flat,
    /// Prints soft line breaks as line breaks
    Expanded,
}

impl PrintMode {
    pub const fn is_flat(&self) -> bool {
        matches!(self, PrintMode::Flat)
    }

    pub const fn is_expanded(&self) -> bool {
        matches!(self, PrintMode::Expanded)
    }
}

/// Provides the printer with different representations for the same element so that the printer
/// can pick the best fitting variant.
///
/// Best fitting is defined as the variant that takes the most horizontal space but fits on the line.
#[derive(Clone, Eq, PartialEq)]
pub struct BestFitting {
    /// The different variants for this element.
    /// The first element is the one that takes up the most space horizontally (the most flat),
    /// The last element takes up the least space horizontally (but most horizontal space).
    variants: Box<[FormatElement]>,
}

impl BestFitting {
    /// Creates a new best fitting IR with the given variants. The method itself isn't unsafe
    /// but it is to discourage people from using it because the printer will panic if
    /// the slice doesn't contain at least the least and most expanded variants.
    ///
    /// You're looking for a way to create a `BestFitting` object, use the `best_fitting![least_expanded, most_expanded]` macro.
    ///
    /// ## Safety
    /// The slice must contain at least two variants.
    #[doc(hidden)]
    pub(crate) unsafe fn from_vec_unchecked(variants: Vec<FormatElement>) -> Self {
        debug_assert!(
            variants.len() >= 2,
            "Requires at least the least expanded and most expanded variants"
        );

        Self {
            variants: variants.into_boxed_slice(),
        }
    }

    /// Returns the most expanded variant
    pub fn most_expanded(&self) -> &FormatElement {
        self.variants.last().expect(
            "Most contain at least two elements, as guaranteed by the best fitting builder.",
        )
    }

    pub fn variants(&self) -> &[FormatElement] {
        &self.variants
    }

    /// Returns the least expanded variant
    pub fn most_flat(&self) -> &FormatElement {
        self.variants.first().expect(
            "Most contain at least two elements, as guaranteed by the best fitting builder.",
        )
    }
}

impl Debug for BestFitting {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&*self.variants).finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Interned(Rc<FormatElement>);

impl Interned {
    pub(crate) fn try_unwrap(this: Interned) -> Result<FormatElement, Interned> {
        Rc::try_unwrap(this.0).map_err(Interned)
    }
}

impl Debug for Interned {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Interned {
    type Target = FormatElement;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConditionalGroupContent {
    pub(crate) content: Content,

    /// In what mode the content should be printed.
    /// * Flat -> Omitted if the enclosing group is a multiline group, printed for groups fitting on a single line
    /// * Multiline -> Omitted if the enclosing group fits on a single line, printed if the group breaks over multiple lines.
    pub(crate) mode: PrintMode,

    /// The id of the group for which it should check if it breaks or not. The group must appear in the document
    /// before the conditional group (but doesn't have to be in the ancestor chain).
    pub(crate) group_id: Option<GroupId>,
}

impl ConditionalGroupContent {
    pub fn new(content: FormatElement, mode: PrintMode) -> Self {
        Self {
            content: Box::new(content),
            mode,
            group_id: None,
        }
    }

    pub fn with_group_id(mut self, id: Option<GroupId>) -> Self {
        self.group_id = id;
        self
    }
}

/// See [token] for documentation
#[derive(Eq, Clone)]
pub enum Token {
    /// Token constructed by the formatter from a static string
    Static { text: &'static str },
    /// Token constructed from the input source as a dynamics
    /// string and a range of the input source
    Dynamic {
        // There's no need for the text to be mutable, using `Box<str>` safes 8 bytes over `String`.
        text: Box<str>,
        // The position of the dynamic token in the unformatted source code
        source_position: TextSize,
    },
    /// A token for a text that is taken as is from the source code (input text and formatted representation are identical).
    /// Implementing by taking a slice from a `SyntaxToken` to avoid allocating a new string.
    SyntaxTokenSlice {
        /// The start position of the token in the unformatted source code
        source_position: TextSize,
        /// The token text
        slice: SyntaxTokenText,
    },
}

impl Debug for Token {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // This does not use debug_tuple so the tokens are
        // written on a single line even when pretty-printing
        match self {
            Token::Static { text } => write!(fmt, "StaticToken({:?})", text),
            Token::Dynamic { text, .. } => write!(fmt, "DynamicToken({:?})", text),
            Token::SyntaxTokenSlice {
                slice: token_text, ..
            } => {
                write!(fmt, "SyntaxTokenSlice({:?})", token_text)
            }
        }
    }
}

impl Token {
    /// Get the range of the input source covered by this token,
    /// or None if the token was synthesized by the formatter
    pub fn source_position(&self) -> Option<&TextSize> {
        match self {
            Token::Static { .. } => None,
            Token::Dynamic {
                source_position, ..
            } => Some(source_position),
            Token::SyntaxTokenSlice {
                source_position, ..
            } => Some(source_position),
        }
    }
}

// Token equality only compares the text content
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

const LINE_SEPARATOR: char = '\u{2028}';
const PARAGRAPH_SEPARATOR: char = '\u{2029}';
pub const LINE_TERMINATORS: [char; 3] = ['\r', LINE_SEPARATOR, PARAGRAPH_SEPARATOR];

/// Replace the line terminators matching the provided list with "\n"
/// since its the only line break type supported by the printer
pub fn normalize_newlines<const N: usize>(text: &str, terminators: [char; N]) -> Cow<str> {
    let mut result = String::new();
    let mut last_end = 0;

    for (start, part) in text.match_indices(terminators) {
        result.push_str(&text[last_end..start]);
        result.push('\n');

        last_end = start + part.len();
        // If the current character is \r and the
        // next is \n, skip over the entire sequence
        if part == "\r" && text[last_end..].starts_with('\n') {
            last_end += 1;
        }
    }

    // If the result is empty no line terminators were matched,
    // return the entire input text without allocating a new String
    if result.is_empty() {
        Cow::Borrowed(text)
    } else {
        result.push_str(&text[last_end..text.len()]);
        Cow::Owned(result)
    }
}

impl Deref for Token {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Token::Static { text } => text,
            Token::Dynamic { text, .. } => text,
            Token::SyntaxTokenSlice {
                slice: token_text, ..
            } => token_text.deref(),
        }
    }
}

impl FormatElement {
    /// Returns true if the element contains no content.
    pub fn is_empty(&self) -> bool {
        match self {
            FormatElement::List(list) => list.is_empty(),
            _ => false,
        }
    }

    /// Returns true if this [FormatElement] is guaranteed to break across multiple lines by the printer.
    /// This is the case if this format element recursively contains a:
    /// * [empty_line] or [hard_line_break]
    /// * A token containing '\n'
    ///
    /// Use this with caution, this is only a heuristic and the printer may print the element over multiple
    /// lines if this element is part of a group and the group doesn't fit on a single line.
    pub fn will_break(&self) -> bool {
        match self {
            FormatElement::Space => false,
            FormatElement::Line(line_mode) => matches!(line_mode, LineMode::Hard | LineMode::Empty),
            FormatElement::Indent(content) => content.will_break(),
            FormatElement::Group(Group { content, .. }) | FormatElement::Comment(content) => {
                content.iter().any(FormatElement::will_break)
            }
            FormatElement::ConditionalGroupContent(group) => group.content.will_break(),
            FormatElement::List(list) => list.content.iter().any(FormatElement::will_break),
            FormatElement::Fill(fill) => fill.list.content.iter().any(FormatElement::will_break),
            FormatElement::Token(token) => token.contains('\n'),
            FormatElement::LineSuffix(_) => false,
            FormatElement::Verbatim(verbatim) => verbatim.element.will_break(),
            FormatElement::BestFitting(_) => false,
            FormatElement::LineSuffixBoundary => false,
            FormatElement::ExpandParent => true,
            FormatElement::Interned(inner) => inner.0.will_break(),
        }
    }

    /// Utility function to get the "last element" of a [FormatElement], recursing
    /// into lists and groups for find the last element that's not an empty element,
    /// a line break or a comment
    pub fn last_element(&self) -> Option<&FormatElement> {
        match self {
            FormatElement::Fill(fill) => fill
                .list
                .iter()
                .rev()
                .find_map(|element| element.last_element()),
            FormatElement::List(list) => {
                list.iter().rev().find_map(|element| element.last_element())
            }
            FormatElement::Line(_) | FormatElement::Comment(_) => None,

            FormatElement::Indent(indent) => indent.last_element(),
            FormatElement::Group(group) => group
                .content
                .iter()
                .rev()
                .find_map(FormatElement::last_element),

            _ => Some(self),
        }
    }

    /// Interns a format element.
    ///
    /// Returns `self` for an empty list AND an already interned elements.
    pub fn intern(self) -> FormatElement {
        match self {
            FormatElement::List(list) if list.is_empty() => list.into(),
            element @ FormatElement::Interned(_) => element,
            element => FormatElement::Interned(Interned(Rc::new(element))),
        }
    }
}

impl From<Token> for FormatElement {
    fn from(token: Token) -> Self {
        FormatElement::Token(token)
    }
}

impl From<Group> for FormatElement {
    fn from(group: Group) -> Self {
        FormatElement::Group(group)
    }
}

impl From<List> for FormatElement {
    fn from(token: List) -> Self {
        FormatElement::List(token)
    }
}

impl FromIterator<FormatElement> for FormatElement {
    fn from_iter<T: IntoIterator<Item = FormatElement>>(iter: T) -> Self {
        let iter = iter.into_iter();

        let mut list = Vec::with_capacity(iter.size_hint().0);

        for element in iter {
            match element {
                FormatElement::List(append) => {
                    list.extend(append.content);
                }
                element => list.push(element),
            }
        }

        FormatElement::from(List::new(list))
    }
}

impl From<ConditionalGroupContent> for FormatElement {
    fn from(token: ConditionalGroupContent) -> Self {
        FormatElement::ConditionalGroupContent(token)
    }
}

#[cfg(test)]
mod tests {

    use crate::format_element::{normalize_newlines, LINE_TERMINATORS};

    #[test]
    fn test_normalize_newlines() {
        assert_eq!(normalize_newlines("a\nb", LINE_TERMINATORS), "a\nb");
        assert_eq!(normalize_newlines("a\rb", LINE_TERMINATORS), "a\nb");
        assert_eq!(normalize_newlines("a\r\nb", LINE_TERMINATORS), "a\nb");
        assert_eq!(normalize_newlines("a\u{2028}b", LINE_TERMINATORS), "a\nb");
        assert_eq!(normalize_newlines("a\u{2029}b", LINE_TERMINATORS), "a\nb");
    }
}

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<rome_rowan::TextRange>() == 8usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::VerbatimKind>() == 8usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::Verbatim>() == 16usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::Token>() == 24usize);

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::ConditionalGroupContent>() == 16usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::List>() == 24usize);

// Increasing the size of FormatElement has serious consequences on runtime performance and memory footprint.
// Is there a more efficient way to encode the data to avoid increasing its size? Can the information
// be recomputed at a later point in time?
// You reduced the size of a format element? Excellent work!

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::FormatElement>() == 32usize);
