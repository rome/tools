use crate::prelude::{dynamic_text, format_with};
use crate::printer::LineEnding;
use crate::{
    format, format_args, group, soft_block_indent, soft_line_break_or_space,
    soft_line_indent_or_space, space, text, write, Buffer, Format, FormatContext, FormatOptions,
    FormatResult, Formatter, GroupId, IndentStyle, LineWidth, PrinterOptions, TextSize,
    TransformSourceMap,
};
#[cfg(target_pointer_width = "64")]
use rome_rowan::static_assert;
use rome_rowan::SyntaxTokenText;
use rustc_hash::FxHashMap;
#[cfg(debug_assertions)]
use std::any::type_name;
use std::any::TypeId;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU8;
use std::ops::Deref;
use std::rc::Rc;

type Content = Box<[FormatElement]>;

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [crate::space], [crate::soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq)]
pub enum FormatElement {
    /// A space token, see [crate::space] for documentation.
    Space,

    /// A new line, see [crate::soft_line_break], [crate::hard_line_break], and [crate::soft_line_break_or_space] for documentation.
    Line(LineMode),

    /// Indents the content one level deeper, see [crate::indent] for documentation and examples.
    Indent(Content),

    /// Variant of [FormatElement::Indent] that indents content by a number of spaces. For example, `Align(2)`
    /// indents any content following a line break by an additional two spaces.
    ///
    /// Nesting (Aligns)[FormatElement::Align] has the effect that all except the most inner align are handled as (Indent)[FormatElement::Indent].
    Align(Align),

    /// Reduces the indention of the specified content either by one level or to the root, depending on the mode.
    /// Reverse operation of `Indent` and can be used to *undo* an `Align` for nested content.
    Dedent { content: Content, mode: DedentMode },

    /// Creates a logical group where its content is either consistently printed:
    /// * on a single line: Omitting `LineMode::Soft` line breaks and printing spaces for `LineMode::SoftOrSpace`
    /// * on multiple lines: Printing all line breaks
    ///
    /// See [crate::group] for documentation and examples.
    Group(Group),

    /// Forces the parent group to print in expanded mode.
    ExpandParent,

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [crate::if_group_breaks] for examples.
    ConditionalGroupContent(ConditionalGroupContent),

    /// Optimized version of [FormatElement::ConditionalGroupContent] for the case where some content
    /// should be indented if the specified group breaks.
    IndentIfGroupBreaks(IndentIfGroupBreaks),

    /// Concatenates multiple elements together. See [crate::Formatter::join_with] for examples.
    List(List),

    /// Concatenates multiple elements together with a given separator printed in either
    /// flat or expanded mode to fill the print width. Expect that the content is a list of alternating
    /// [element, separator] See [crate::Formatter::fill].
    Fill(Content),

    /// A text that should be printed as is, see [crate::text] for documentation and examples.
    Text(Text),

    /// Delay the printing of its content until the next line break
    LineSuffix(Content),

    /// Prevents that line suffixes move past this boundary. Forces the printer to print any pending
    /// line suffixes, potentially by inserting a hard line break.
    LineSuffixBoundary,

    /// A token that tracks tokens/nodes that are printed as verbatim.
    Verbatim(Verbatim),

    /// A list of different variants representing the same content. The printer picks the best fitting content.
    /// Line breaks inside of a best fitting don't propagate to parent groups.
    BestFitting(BestFitting),

    /// An interned format element. Useful when the same content must be emitted multiple times to avoid
    /// deep cloning the IR when using the `best_fitting!` macro or `if_group_fits_on_line` and `if_group_breaks`.
    Interned(Interned),

    /// Special semantic element marking the content with a label.
    /// This does not directly influence how the content will be printed.
    ///
    /// See [crate::labelled] for documentation.
    Label(Label),
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
    pub content: Box<[FormatElement]>,
}

impl Verbatim {
    pub fn new_verbatim(content: Box<[FormatElement]>, length: TextSize) -> Self {
        Self {
            content,
            kind: VerbatimKind::Verbatim { length },
        }
    }

    pub fn new_unknown(content: Box<[FormatElement]>) -> Self {
        Self {
            content,
            kind: VerbatimKind::Unknown,
        }
    }

    pub fn new_suppressed(content: Box<[FormatElement]>) -> Self {
        Self {
            content,
            kind: VerbatimKind::Suppressed,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self.kind, VerbatimKind::Unknown)
    }
}

impl std::fmt::Display for FormatElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = format!(IrFormatContext::default(), [self])
            .expect("Formatting not to throw any FormatErrors");

        f.write_str(formatted.print().as_code())
    }
}

impl std::fmt::Debug for FormatElement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::write;
        match self {
            FormatElement::Space => write!(fmt, "Space"),
            FormatElement::Line(content) => fmt.debug_tuple("Line").field(content).finish(),
            FormatElement::Indent(content) => fmt.debug_tuple("Indent").field(content).finish(),
            FormatElement::Dedent { content, mode } => fmt
                .debug_struct("Dedent")
                .field("content", content)
                .field("mode", mode)
                .finish(),
            FormatElement::Align(Align { count, content }) => fmt
                .debug_struct("Align")
                .field("count", count)
                .field("content", content)
                .finish(),
            FormatElement::Group(content) => {
                write!(fmt, "Group")?;
                content.fmt(fmt)
            }
            FormatElement::ConditionalGroupContent(content) => content.fmt(fmt),
            FormatElement::IndentIfGroupBreaks(content) => content.fmt(fmt),
            FormatElement::List(content) => {
                write!(fmt, "List ")?;
                content.fmt(fmt)
            }
            FormatElement::Fill(fill) => fill.fmt(fmt),
            FormatElement::Text(content) => content.fmt(fmt),
            FormatElement::LineSuffix(content) => {
                fmt.debug_tuple("LineSuffix").field(content).finish()
            }
            FormatElement::LineSuffixBoundary => write!(fmt, "LineSuffixBoundary"),
            FormatElement::Verbatim(verbatim) => fmt
                .debug_tuple("Verbatim")
                .field(&verbatim.content)
                .finish(),
            FormatElement::BestFitting(best_fitting) => {
                write!(fmt, "BestFitting")?;
                best_fitting.fmt(fmt)
            }
            FormatElement::ExpandParent => write!(fmt, "ExpandParent"),
            FormatElement::Interned(inner) => inner.fmt(fmt),
            FormatElement::Label(label) => {
                write!(fmt, "Label")?;
                label.fmt(fmt)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
    /// See [crate::soft_line_break_or_space] for documentation.
    SoftOrSpace,
    /// See [crate::soft_line_break] for documentation.
    Soft,
    /// See [crate::hard_line_break] for documentation.
    Hard,
    /// See [crate::empty_line] for documentation.
    Empty,
}

impl LineMode {
    pub const fn is_hard(&self) -> bool {
        matches!(self, LineMode::Hard)
    }
}

/// A token used to gather a list of elements; see [crate::Formatter::join_with].
#[derive(Clone, Default, Eq, PartialEq)]
pub struct List {
    content: Vec<FormatElement>,
}

impl std::fmt::Debug for List {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Align {
    pub(super) content: Content,
    pub(super) count: NonZeroU8,
}

impl Align {
    pub fn count(&self) -> NonZeroU8 {
        self.count
    }

    pub fn content(&self) -> &[FormatElement] {
        &self.content
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

impl std::fmt::Debug for Group {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DedentMode {
    /// Reduces the indent by a level (if the current indent is > 0)
    Level,

    /// Reduces the indent to the root
    Root,
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

impl std::fmt::Debug for BestFitting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&*self.variants).finish()
    }
}

#[derive(Clone)]
pub struct Interned(Rc<FormatElement>);

impl Interned {
    pub(crate) fn new(element: FormatElement) -> Self {
        Self(Rc::new(element))
    }
}

impl PartialEq for Interned {
    fn eq(&self, other: &Interned) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Interned {}

impl Hash for Interned {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        hasher.write_usize(Rc::as_ptr(&self.0) as usize);
    }
}

impl std::fmt::Debug for Interned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Interned {
    type Target = FormatElement;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct LabelId {
    id: TypeId,
    #[cfg(debug_assertions)]
    label: &'static str,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for LabelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label)
    }
}

#[cfg(not(debug_assertions))]
impl std::fmt::Debug for LabelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "#{:?}", self.id)
    }
}

impl LabelId {
    pub fn of<T: ?Sized + 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            #[cfg(debug_assertions)]
            label: type_name::<T>(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Label {
    pub(crate) content: Box<[FormatElement]>,
    label_id: LabelId,
}

impl Label {
    pub fn new(label_id: LabelId, content: Vec<FormatElement>) -> Self {
        Self {
            content: content.into_boxed_slice(),
            label_id,
        }
    }

    pub fn label_id(&self) -> LabelId {
        self.label_id
    }
}

impl std::fmt::Debug for Label {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("")
            .field("label_id", &self.label_id)
            .field("content", &self.content)
            .finish()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndentIfGroupBreaks {
    pub(crate) content: Content,

    pub(crate) group_id: GroupId,
}

impl IndentIfGroupBreaks {
    pub fn new(content: Content, group_id: GroupId) -> Self {
        Self { content, group_id }
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
    pub fn new(content: Box<[FormatElement]>, mode: PrintMode) -> Self {
        Self {
            content,
            mode,
            group_id: None,
        }
    }

    pub fn with_group_id(mut self, id: Option<GroupId>) -> Self {
        self.group_id = id;
        self
    }
}

/// See [crate::text] for documentation
#[derive(Eq, Clone)]
pub enum Text {
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
    SyntaxTokenTextSlice {
        /// The start position of the token in the unformatted source code
        source_position: TextSize,
        /// The token text
        slice: SyntaxTokenText,
    },
}

impl std::fmt::Debug for Text {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::write;

        // This does not use debug_tuple so the tokens are
        // written on a single line even when pretty-printing
        match self {
            Text::Static { text } => write!(fmt, "StaticText({:?})", text),
            Text::Dynamic { text, .. } => write!(fmt, "DynamicText({:?})", text),
            Text::SyntaxTokenTextSlice {
                slice: token_text, ..
            } => {
                write!(fmt, "SyntaxTokenTextSlice({:?})", token_text)
            }
        }
    }
}

impl Text {
    /// Get the range of the input source covered by this token,
    /// or None if the token was synthesized by the formatter
    pub fn source_position(&self) -> Option<&TextSize> {
        match self {
            Text::Static { .. } => None,
            Text::Dynamic {
                source_position, ..
            } => Some(source_position),
            Text::SyntaxTokenTextSlice {
                source_position, ..
            } => Some(source_position),
        }
    }
}

// Token equality only compares the text content
impl PartialEq for Text {
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

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Text::Static { text } => text,
            Text::Dynamic { text, .. } => text,
            Text::SyntaxTokenTextSlice {
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
    /// * [crate::empty_line] or [crate::hard_line_break]
    /// * A token containing '\n'
    ///
    /// Use this with caution, this is only a heuristic and the printer may print the element over multiple
    /// lines if this element is part of a group and the group doesn't fit on a single line.
    pub fn will_break(&self) -> bool {
        match self {
            FormatElement::Space => false,
            FormatElement::Line(line_mode) => matches!(line_mode, LineMode::Hard | LineMode::Empty),
            FormatElement::Group(Group { content, .. })
            | FormatElement::ConditionalGroupContent(ConditionalGroupContent { content, .. })
            | FormatElement::IndentIfGroupBreaks(IndentIfGroupBreaks { content, .. })
            | FormatElement::Fill(content)
            | FormatElement::Verbatim(Verbatim { content, .. })
            | FormatElement::Label(Label { content, .. })
            | FormatElement::Indent(content)
            | FormatElement::Dedent { content, .. }
            | FormatElement::Align(Align { content, .. }) => {
                content.iter().any(FormatElement::will_break)
            }
            FormatElement::List(list) => list.content.iter().any(FormatElement::will_break),
            FormatElement::Text(token) => token.contains('\n'),
            FormatElement::LineSuffix(_) => false,
            FormatElement::BestFitting(_) => false,
            FormatElement::LineSuffixBoundary => false,
            FormatElement::ExpandParent => true,
            FormatElement::Interned(inner) => inner.0.will_break(),
        }
    }

    /// Returns true if the element has the given label.
    pub fn has_label(&self, label_id: LabelId) -> bool {
        match self {
            FormatElement::Label(label) => label.label_id == label_id,
            FormatElement::Interned(interned) => interned.deref().has_label(label_id),
            _ => false,
        }
    }

    /// Utility function to get the "last element" of a [FormatElement], recursing
    /// into lists and groups to find the last element that's not
    /// a line break
    pub fn last_element(&self) -> Option<&FormatElement> {
        match self {
            FormatElement::List(list) => {
                list.iter().rev().find_map(|element| element.last_element())
            }
            FormatElement::Line(_) => None,

            FormatElement::Group(Group { content, .. }) | FormatElement::Indent(content) => {
                content.iter().rev().find_map(FormatElement::last_element)
            }
            FormatElement::Interned(Interned(inner)) => inner.last_element(),

            _ => Some(self),
        }
    }
}

impl From<Text> for FormatElement {
    fn from(token: Text) -> Self {
        FormatElement::Text(token)
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

#[derive(Clone, Default, Debug)]
struct IrFormatContext {
    /// The interned elements that have been printed to this point
    printed_interned_elements: FxHashMap<Interned, usize>,
}

impl FormatContext for IrFormatContext {
    type Options = IrFormatOptions;

    fn options(&self) -> &Self::Options {
        &IrFormatOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

#[derive(Debug, Clone, Default)]
struct IrFormatOptions;

impl FormatOptions for IrFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::Space(2)
    }

    fn line_width(&self) -> LineWidth {
        LineWidth(80)
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            tab_width: 2,
            print_width: self.line_width().into(),
            line_ending: LineEnding::LineFeed,
            indent_style: IndentStyle::Space(2),
        }
    }
}

impl Format<IrFormatContext> for FormatElement {
    fn fmt(&self, f: &mut crate::Formatter<IrFormatContext>) -> FormatResult<()> {
        match self {
            FormatElement::Space => {
                write!(f, [text(" ")])
            }
            FormatElement::Line(mode) => match mode {
                LineMode::SoftOrSpace => {
                    write!(f, [text("soft_line_break_or_space")])
                }
                LineMode::Soft => {
                    write!(f, [text("soft_line_break")])
                }
                LineMode::Hard => {
                    write!(f, [text("hard_line_break")])
                }
                LineMode::Empty => {
                    write!(f, [text("empty_line")])
                }
            },
            FormatElement::ExpandParent => {
                write!(f, [text("expand_parent")])
            }
            text @ FormatElement::Text(_) => f.write_element(text.clone()),
            FormatElement::LineSuffixBoundary => {
                write!(f, [text("line_suffix_boundary")])
            }
            FormatElement::Indent(content) => {
                write!(f, [text("indent("), content.as_ref(), text(")")])
            }
            FormatElement::Dedent { content, mode } => {
                let label = match mode {
                    DedentMode::Level => "dedent",
                    DedentMode::Root => "dedentRoot",
                };

                write!(f, [text(label), text("("), content.as_ref(), text(")")])
            }
            FormatElement::Align(Align { content, count }) => {
                write!(
                    f,
                    [
                        text("align("),
                        dynamic_text(&count.to_string(), TextSize::default()),
                        text(","),
                        space(),
                        content.as_ref(),
                        text(")")
                    ]
                )
            }
            FormatElement::List(list) => {
                write!(f, [list.as_ref()])
            }
            FormatElement::LineSuffix(line_suffix) => {
                write!(f, [text("line_suffix("), line_suffix.as_ref(), text(")")])
            }
            FormatElement::Verbatim(verbatim) => {
                write!(f, [text("verbatim("), verbatim.content.as_ref(), text(")")])
            }
            FormatElement::Group(group_element) => {
                write!(f, [text("group(")])?;

                if let Some(id) = group_element.id {
                    write!(
                        f,
                        [group(&soft_block_indent(&format_args![
                            group_element.content.as_ref(),
                            text(","),
                            soft_line_break_or_space(),
                            text("{"),
                            group(&format_args![soft_line_indent_or_space(&format_args![
                                text("id:"),
                                space(),
                                dynamic_text(&std::format!("{id:?}"), TextSize::default()),
                                soft_line_break_or_space()
                            ])]),
                            text("}")
                        ]))]
                    )?;
                } else {
                    write!(f, [group_element.content.as_ref()])?;
                }

                write!(f, [text(")")])
            }
            FormatElement::IndentIfGroupBreaks(content) => {
                write!(
                    f,
                    [
                        text("indent_if_group_breaks("),
                        group(&soft_block_indent(&format_args![
                            content.content.as_ref(),
                            text(","),
                            soft_line_break_or_space(),
                            text("{"),
                            group(&format_args![soft_line_indent_or_space(&format_args![
                                text("group-id:"),
                                space(),
                                dynamic_text(
                                    &std::format!("{:?}", content.group_id),
                                    TextSize::default()
                                ),
                                soft_line_break_or_space()
                            ])]),
                            text("}")
                        ]))
                    ]
                )
            }
            FormatElement::ConditionalGroupContent(content) => {
                match content.mode {
                    PrintMode::Flat => {
                        write!(f, [text("if_group_fits_on_line")])?;
                    }
                    PrintMode::Expanded => {
                        write!(f, [text("if_group_breaks")])?;
                    }
                }

                write!(f, [text("(")])?;

                if let Some(id) = content.group_id {
                    write!(
                        f,
                        [group(&soft_block_indent(&format_args![
                            content.content.as_ref(),
                            text(","),
                            soft_line_break_or_space(),
                            text("{"),
                            group(&format_args![soft_line_indent_or_space(&format_args![
                                text("id:"),
                                space(),
                                dynamic_text(&std::format!("{id:?}"), TextSize::default()),
                                soft_line_break_or_space()
                            ])]),
                            text("}")
                        ]))]
                    )?;
                } else {
                    write!(f, [content.content.as_ref()])?;
                }

                write!(f, [text(")")])
            }
            FormatElement::Label(labelled) => {
                let label_id = labelled.label_id;
                write!(
                    f,
                    [
                        text("label(\""),
                        dynamic_text(&std::format!("{label_id:?}"), TextSize::default()),
                        text("\","),
                        space(),
                        labelled.content.as_ref(),
                        text(")")
                    ]
                )
            }
            FormatElement::Fill(content) => {
                write!(f, [text("fill("), content.as_ref(), text(")")])
            }

            FormatElement::BestFitting(best_fitting) => {
                write!(
                    f,
                    [text("best_fitting("), best_fitting.variants(), text(")")]
                )
            }
            FormatElement::Interned(interned) => {
                let interned_elements = &mut f.context_mut().printed_interned_elements;

                let (index, inserted) = match interned_elements.get(interned) {
                    None => {
                        let index = interned_elements.len();
                        interned_elements.insert(interned.clone(), index);
                        (index, true)
                    }
                    Some(index) => (*index, false),
                };

                if inserted {
                    write!(
                        f,
                        [
                            dynamic_text(&std::format!("<interned {index}>"), TextSize::default()),
                            space(),
                        ]
                    )?;

                    match interned.0.as_ref() {
                        element @ FormatElement::Text(_) | element @ FormatElement::Space => {
                            write!(f, [text("\""), element, text("\"")])
                        }
                        element => element.fmt(f),
                    }
                } else {
                    write!(
                        f,
                        [dynamic_text(
                            &std::format!("<ref interned *{index}>"),
                            TextSize::default()
                        )]
                    )
                }
            }
        }
    }
}

impl<'a> Format<IrFormatContext> for &'a [FormatElement] {
    fn fmt(&self, f: &mut Formatter<IrFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [
                text("["),
                group(&soft_block_indent(&format_with(|f| {
                    let mut joiner = f.join_with(soft_line_break_or_space());
                    let len = self.len();

                    for (index, element) in self.iter().enumerate() {
                        joiner.entry(&format_with(|f| {
                            let print_as_str =
                                matches!(element, FormatElement::Text(_) | FormatElement::Space);

                            if print_as_str {
                                write!(f, [text("\""), &element, text("\"")])?;
                            } else {
                                write!(f, [group(&element)])?;
                            }

                            if index < len - 1 {
                                write!(f, [text(",")])?;
                            }

                            Ok(())
                        }));
                    }

                    joiner.finish()
                }))),
                text("]")
            ]
        )
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
static_assert!(std::mem::size_of::<crate::format_element::Verbatim>() == 24usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::Text>() == 24usize);

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::ConditionalGroupContent>() == 24usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::List>() == 24usize);

// Increasing the size of FormatElement has serious consequences on runtime performance and memory footprint.
// Is there a more efficient way to encode the data to avoid increasing its size? Can the information
// be recomputed at a later point in time?
// You reduced the size of a format element? Excellent work!

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::FormatElement>() == 32usize);
