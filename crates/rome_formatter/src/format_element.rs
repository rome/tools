pub mod document;
pub mod tag;

use crate::format_element::tag::{LabelId, Tag};

use crate::{TagKind, TextSize};
#[cfg(target_pointer_width = "64")]
use rome_rowan::static_assert;
use rome_rowan::SyntaxTokenText;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [crate::builders::space], [crate::builders::soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq)]
pub enum FormatElement {
    /// A space token, see [crate::builders::space] for documentation.
    Space,

    /// A new line, see [crate::builders::soft_line_break], [crate::builders::hard_line_break], and [crate::builders::soft_line_break_or_space] for documentation.
    Line(LineMode),

    /// Forces the parent group to print in expanded mode.
    ExpandParent,

    /// A text that should be printed as is, see [crate::builders::text] for documentation and examples.
    Text(Text),

    /// Prevents that line suffixes move past this boundary. Forces the printer to print any pending
    /// line suffixes, potentially by inserting a hard line break.
    LineSuffixBoundary,

    /// An interned format element. Useful when the same content must be emitted multiple times to avoid
    /// deep cloning the IR when using the `best_fitting!` macro or `if_group_fits_on_line` and `if_group_breaks`.
    Interned(Interned),

    /// A list of different variants representing the same content. The printer picks the best fitting content.
    /// Line breaks inside of a best fitting don't propagate to parent groups.
    BestFitting(BestFitting),

    /// A [Tag] that marks the start/end of some content to which some special formatting is applied.
    Tag(Tag),
}

impl std::fmt::Debug for FormatElement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FormatElement::Space => write!(fmt, "Space"),
            FormatElement::Line(mode) => fmt.debug_tuple("Line").field(mode).finish(),
            FormatElement::ExpandParent => write!(fmt, "ExpandParent"),
            FormatElement::Text(text) => text.fmt(fmt),
            FormatElement::LineSuffixBoundary => write!(fmt, "LineSuffixBoundary"),
            FormatElement::BestFitting(best_fitting) => {
                fmt.debug_tuple("BestFitting").field(&best_fitting).finish()
            }
            FormatElement::Interned(interned) => {
                fmt.debug_list().entries(interned.deref()).finish()
            }
            FormatElement::Tag(tag) => fmt.debug_tuple("Tag").field(tag).finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
    /// See [crate::builders::soft_line_break_or_space] for documentation.
    SoftOrSpace,
    /// See [crate::builders::soft_line_break] for documentation.
    Soft,
    /// See [crate::builders::hard_line_break] for documentation.
    Hard,
    /// See [crate::builders::literal_line] for documentation.
    Literal,
    /// See [crate::builders::empty_line] for documentation.
    Empty,
}

impl LineMode {
    pub const fn is_hard(&self) -> bool {
        matches!(self, LineMode::Hard)
    }

    pub const fn is_literal(&self) -> bool {
        matches!(self, LineMode::Literal)
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

#[derive(Clone)]
pub struct Interned(Rc<[FormatElement]>);

impl Interned {
    pub(super) fn new(content: Vec<FormatElement>) -> Self {
        Self(content.into())
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
        Rc::as_ptr(&self.0).hash(hasher);
    }
}

impl std::fmt::Debug for Interned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Interned {
    type Target = [FormatElement];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// See [crate::builders::text] for documentation
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
    /// Returns `true` if self is a [FormatElement::Tag]
    pub const fn is_tag(&self) -> bool {
        matches!(self, FormatElement::Tag(_))
    }

    /// Returns `true` if self is a [FormatElement::Tag] and [Tag::is_start] is `true`.
    pub const fn is_start_tag(&self) -> bool {
        match self {
            FormatElement::Tag(tag) => tag.is_start(),
            _ => false,
        }
    }

    /// Returns `true` if self is a [FormatElement::Tag] and [Tag::is_end] is `true`.
    pub const fn is_end_tag(&self) -> bool {
        match self {
            FormatElement::Tag(tag) => tag.is_end(),
            _ => false,
        }
    }
}

impl FormatElements for FormatElement {
    fn will_break(&self) -> bool {
        match self {
            FormatElement::ExpandParent => true,
            FormatElement::Tag(Tag::StartGroup(group)) => !group.mode().is_flat(),
            FormatElement::Line(line_mode) => matches!(
                line_mode,
                LineMode::Hard | LineMode::Empty | LineMode::Literal
            ),
            FormatElement::Interned(interned) => interned.will_break(),
            // Traverse into the most flat version because the content is guaranteed to expand when even
            // the most flat version contains some content that forces a break.
            FormatElement::BestFitting(best_fitting) => best_fitting.most_flat().will_break(),
            FormatElement::LineSuffixBoundary
            | FormatElement::Space
            | FormatElement::Tag(_)
            | FormatElement::Text(_) => false,
        }
    }

    fn has_label(&self, label_id: LabelId) -> bool {
        match self {
            FormatElement::Tag(Tag::StartLabelled(actual)) => *actual == label_id,
            FormatElement::Interned(interned) => interned.deref().has_label(label_id),
            _ => false,
        }
    }

    fn start_tag(&self, _: TagKind) -> Option<&Tag> {
        None
    }

    fn end_tag(&self, kind: TagKind) -> Option<&Tag> {
        match self {
            FormatElement::Tag(tag) if tag.kind() == kind && tag.is_end() => Some(tag),
            _ => None,
        }
    }
}

impl From<Text> for FormatElement {
    fn from(token: Text) -> Self {
        FormatElement::Text(token)
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
    variants: Box<[Box<[FormatElement]>]>,
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
    pub(crate) unsafe fn from_vec_unchecked(variants: Vec<Box<[FormatElement]>>) -> Self {
        debug_assert!(
            variants.len() >= 2,
            "Requires at least the least expanded and most expanded variants"
        );

        Self {
            variants: variants.into_boxed_slice(),
        }
    }

    /// Returns the most expanded variant
    pub fn most_expanded(&self) -> &[FormatElement] {
        self.variants.last().expect(
            "Most contain at least two elements, as guaranteed by the best fitting builder.",
        )
    }

    pub fn variants(&self) -> &[Box<[FormatElement]>] {
        &self.variants
    }

    /// Returns the least expanded variant
    pub fn most_flat(&self) -> &[FormatElement] {
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

pub trait FormatElements {
    /// Returns true if this [FormatElement] is guaranteed to break across multiple lines by the printer.
    /// This is the case if this format element recursively contains a:
    /// * [crate::builders::empty_line] or [crate::builders::hard_line_break]
    /// * A token containing '\n'
    ///
    /// Use this with caution, this is only a heuristic and the printer may print the element over multiple
    /// lines if this element is part of a group and the group doesn't fit on a single line.
    fn will_break(&self) -> bool;

    /// Returns true if the element has the given label.
    fn has_label(&self, label: LabelId) -> bool;

    /// Returns the start tag of `kind` if:
    /// * the last element is an end tag of `kind`.
    /// * there's a matching start tag in this document (may not be true if this slice is an interned element and the `start` is in the document storing the interned element).
    fn start_tag(&self, kind: TagKind) -> Option<&Tag>;

    /// Returns the end tag if:
    /// * the last element is an end tag of `kind`
    fn end_tag(&self, kind: TagKind) -> Option<&Tag>;
}

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<rome_rowan::TextRange>() == 8usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::tag::VerbatimKind>() == 8usize);

#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::Text>() == 24usize);

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::format_element::Tag>() == 16usize);

// Increasing the size of FormatElement has serious consequences on runtime performance and memory footprint.
// Is there a more efficient way to encode the data to avoid increasing its size? Can the information
// be recomputed at a later point in time?
// You reduced the size of a format element? Excellent work!

#[cfg(not(debug_assertions))]
#[cfg(target_pointer_width = "64")]
static_assert!(std::mem::size_of::<crate::FormatElement>() == 32usize);
