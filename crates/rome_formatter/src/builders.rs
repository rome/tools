use crate::prelude::*;
use crate::{write, GroupId, TextRange, TextSize};
use crate::{Buffer, VecBuffer};
use rome_rowan::{Language, SyntaxNode, SyntaxToken, SyntaxTokenText, TextLen};
use std::borrow::Cow;
use std::cell::Cell;
use std::marker::PhantomData;

/// Format element that doesn't represent any content.
///
/// Can be helpful if you need to return a `FormatElement` (e.g. in an else branch) but don't want
/// to show any content.
pub const fn empty_element() -> Empty {
    Empty
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Empty;

impl<O> Format<O> for Empty {
    fn format(&self, _: &mut Formatter<O>) -> FormatResult<()> {
        Ok(())
    }
}

/// A line break that only gets printed if the enclosing `Group` doesn't fit on a single line.
/// It's omitted if the enclosing `Group` fits on a single line.
/// A soft line break is identical to a hard line break when not enclosed inside of a `Group`.
///
/// ## Examples
///
/// Soft line breaks are omitted if the enclosing `Group` fits on a single line
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![token("a,"), soft_line_break(), token("b")])
/// ]).unwrap();
///
/// assert_eq!(
///     "a,b",
///     elements.print().as_code()
/// );
/// ```
/// See [soft_line_break_or_space] if you want to insert a space between the elements if the enclosing
/// `Group` fits on a single line.
///
/// Soft line breaks are emitted if the enclosing `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(10).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("a long word,"),
///         soft_line_break(),
///         token("so that the group doesn't fit on a single line"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a long word,\nso that the group doesn't fit on a single line",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn soft_line_break() -> Line {
    Line::new(LineMode::Soft)
}

/// A forced line break that are always printed. A hard line break forces any enclosing `Group`
/// to be printed over multiple lines.
///
/// ## Examples
///
/// It forces a line break, even if the enclosing `Group` would otherwise fit on a single line.
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("a,"),
///         hard_line_break(),
///         token("b"),
///         hard_line_break()
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a,\nb\n",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn hard_line_break() -> Line {
    Line::new(LineMode::Hard)
}

/// A forced empty line. An empty line inserts enough line breaks in the output for
/// the previous and next element to be separated by an empty line.
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(
///     SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("a,"),
///         empty_line(),
///         token("b"),
///         empty_line()
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a,\n\nb\n\n",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn empty_line() -> Line {
    Line::new(LineMode::Empty)
}

/// A line break if the enclosing `Group` doesn't fit on a single line, a space otherwise.
///
/// ## Examples
///
/// The line breaks are emitted as spaces if the enclosing `Group` fits on a a single line:
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("a,"),
///         soft_line_break_or_space(),
///         token("b"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a, b",
///     elements.print().as_code()
/// );
/// ```
///
/// The printer breaks the lines if the enclosing `Group` doesn't fit on a single line:
/// ```
/// use rome_formatter::{format_args, format, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(10).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("a long word,"),
///         soft_line_break_or_space(),
///         token("so that the group doesn't fit on a single line"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a long word,\nso that the group doesn't fit on a single line",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn soft_line_break_or_space() -> Line {
    Line::new(LineMode::SoftOrSpace)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Line {
    mode: LineMode,
}

impl Line {
    const fn new(mode: LineMode) -> Self {
        Self { mode }
    }
}

impl<O> Format<O> for Line {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::Line(self.mode));
        Ok(())
    }
}

/// Creates a token that gets written as is to the output. Make sure to properly escape the text if
/// it's user generated (e.g. a string and not a language keyword).
///
/// ## Line feeds
/// Tokens may contain line breaks but they must use the line feeds (`\n`).
/// The [crate::Printer] converts the line feed characters to the character specified in the [crate::PrinterOptions].
///
/// ## Examples
///
/// ```
/// use rome_formatter::format;
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [token("Hello World")]).unwrap();
///
/// assert_eq!(
///     "Hello World",
///     elements.print().as_code()
/// );
/// ```
///
/// Printing a string literal as a literal requires that the string literal is properly escaped and
/// enclosed in quotes (depending on the target language).
///
/// ```
/// use rome_formatter::format;
/// use rome_formatter::prelude::*;
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = format!(SimpleFormatContext::default(), [token("\"Hello\\tWorld\"")]).unwrap();
///
/// assert_eq!(r#""Hello\tWorld""#, elements.print().as_code());
/// ```
#[inline]
pub fn token(text: &'static str) -> StaticToken {
    debug_assert_no_newlines(text);

    StaticToken { text }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StaticToken {
    text: &'static str,
}

impl<O> Format<O> for StaticToken {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::Token(Token::Static { text: self.text }));
        Ok(())
    }
}

/// Create a token from a dynamic string and a range of the input source
pub fn dynamic_token(text: &str, position: TextSize) -> DynamicToken {
    debug_assert_no_newlines(text);

    DynamicToken { text, position }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DynamicToken<'a> {
    text: &'a str,
    position: TextSize,
}

impl<O> Format<O> for DynamicToken<'_> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::Token(Token::Dynamic {
            text: self.text.to_string().into_boxed_str(),
            source_position: self.position,
        }));
        Ok(())
    }
}

pub fn syntax_token_cow_slice<'a, L: Language>(
    text: Cow<'a, str>,
    token: &'a SyntaxToken<L>,
    start: TextSize,
) -> SyntaxTokenCowSlice<'a, L> {
    debug_assert_no_newlines(&text);

    SyntaxTokenCowSlice { text, token, start }
}

pub struct SyntaxTokenCowSlice<'a, L: Language> {
    text: Cow<'a, str>,
    token: &'a SyntaxToken<L>,
    start: TextSize,
}

impl<L: Language, O> Format<O> for SyntaxTokenCowSlice<'_, L> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        match &self.text {
            Cow::Borrowed(text) => {
                let range = TextRange::at(self.start, text.text_len());
                debug_assert_eq!(
                    *text,
                    &self.token.text()[range - self.token.text_range().start()],
                    "The borrowed string doesn't match the specified token substring. Does the borrowed string belong to this token and range?"
                );

                let relative_range = range - self.token.text_range().start();
                let slice = self.token.token_text().slice(relative_range);

                f.write_element(FormatElement::Token(Token::SyntaxTokenSlice {
                    slice,
                    source_position: self.start,
                }))
            }
            Cow::Owned(text) => f.write_element(FormatElement::Token(Token::Dynamic {
                text: text.to_string().into_boxed_str(),
                source_position: self.start,
            })),
        };

        Ok(())
    }
}

pub fn syntax_token_text_slice<L: Language>(
    token: &SyntaxToken<L>,
    range: TextRange,
) -> SyntaxTokenTextSlice {
    let relative_range = range - token.text_range().start();
    let slice = token.token_text().slice(relative_range);

    debug_assert_no_newlines(&slice);

    SyntaxTokenTextSlice {
        text: slice,
        source_position: range.start(),
    }
}

pub struct SyntaxTokenTextSlice {
    text: SyntaxTokenText,
    source_position: TextSize,
}

impl<O> Format<O> for SyntaxTokenTextSlice {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::Token(Token::SyntaxTokenSlice {
            slice: self.text.clone(),
            source_position: self.source_position,
        }));

        Ok(())
    }
}

fn debug_assert_no_newlines(text: &str) {
    debug_assert!(!text.contains('\r'), "The content '{}' contains an unsupported '\\r' line terminator character but string tokens must only use line feeds '\\n' as line separator. Use '\\n' instead of '\\r' and '\\r\\n' to insert a line break in strings.", text);
}

/// Push a [FormatElement] to the end of the current line
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     token("a"),
///     line_suffix(&token("c")),
///     token("b")
/// ]).unwrap();
///
/// assert_eq!(
///     "abc",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn line_suffix<O>(inner: &dyn Format<O>) -> LineSuffix<O> {
    LineSuffix { content: inner }
}

#[derive(Copy, Clone)]
pub struct LineSuffix<'a, O> {
    content: &'a dyn Format<O>,
}

impl<O> Format<O> for LineSuffix<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());
        write!(buffer, [self.content])?;

        let content = buffer.into_document().into_element();
        f.write_element(FormatElement::LineSuffix(Box::new(content)));
        Ok(())
    }
}

/// Inserts a boundary for line suffixes that forces to print all pending line suffixes. Helpful
/// if a line sufix shouldn't pass a certain point.
///
/// ## Examples
///
/// Forces the line suffix "c" to be printed before the token `d`.
/// ```
/// use rome_formatter::format;
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     token("a"),
///     line_suffix(&token("c")),
///     token("b"),
///     line_suffix_boundary(),
///     token("d")
/// ]).unwrap();
///
/// assert_eq!(
///     "abc\nd",
///     elements.print().as_code()
/// );
/// ```
pub const fn line_suffix_boundary() -> LineSuffixBoundary {
    LineSuffixBoundary
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LineSuffixBoundary;

impl<O> Format<O> for LineSuffixBoundary {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::LineSuffixBoundary);
        Ok(())
    }
}

/// Mark a [FormatElement] as being a piece of trivia
///
/// This does not directly influence how this content will be printed, but some
/// parts of the formatter may chose to handle this element in a specific way
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(
///     SimpleFormatContext::default(),
///     [
///         group_elements(&format_args![
///             comment(&empty_line()),
///             token("a"),
///             soft_line_break_or_space(),
///             token("b")
///         ])
///     ]
/// ).unwrap();
///
/// assert_eq!(
///     "\na b",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub fn comment<O>(content: &dyn Format<O>) -> Comment<O> {
    Comment { content }
}

#[derive(Copy, Clone)]
pub struct Comment<'a, O> {
    content: &'a dyn Format<O>,
}

impl<O> Format<O> for Comment<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        write!(buffer, [self.content])?;
        let content = buffer.into_document().into_element();

        f.write_element(FormatElement::Comment(Box::new(content)));
        Ok(())
    }
}

/// Inserts a single space. Allows to separate different tokens.
///
/// ## Examples
///
/// ```
/// use rome_formatter::format;
/// use rome_formatter::prelude::*;
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = format!(SimpleFormatContext::default(), [token("a"), space_token(), token("b")]).unwrap();
///
/// assert_eq!("a b", elements.print().as_code());
/// ```
#[inline]
pub const fn space_token() -> Space {
    Space
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Space;

impl<O> Format<O> for Space {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::Space);
        Ok(())
    }
}

/// It adds a level of indentation to the given content
///
/// It doesn't add any line breaks at the edges of the content, meaning that
/// the line breaks have to be manually added.
///
/// This helper should be used only in rare cases, instead you should rely more on
/// [block_indent] and [soft_block_indent]
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let block = format!(SimpleFormatContext::default(), [
///     token("switch {"),
///     block_indent(&format_args![
///         token("default:"),
///         indent(&format_args![
///             // this is where we want to use a
///             hard_line_break(),
///             token("break;"),
///         ])
///     ]),
///     token("}"),
/// ]).unwrap();
///
/// assert_eq!(
///     "switch {\n\tdefault:\n\t\tbreak;\n}",
///     block.print().as_code()
/// );
/// ```
#[inline]
pub const fn indent<O>(content: &dyn Format<O>) -> Indent<O> {
    Indent { content }
}

pub struct Indent<'a, O> {
    content: &'a dyn Format<O>,
}

impl<O> Format<O> for Indent<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        write!(buffer, [self.content])?;

        if buffer.is_empty() {
            return Ok(());
        }

        let content = buffer.into_document().into_element();
        f.write_element(FormatElement::Indent(Box::new(content)));
        Ok(())
    }
}

/// Inserts a hard line break before and after the content and increases the indention level for the content by one.
///
/// Block indents indent a block of code, such as in a function body, and therefore insert a line
/// break before and after the content.
///
/// Doesn't create an indention if the passed in content is [FormatElement.is_empty].
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let block = format![
///     SimpleFormatContext::default(),
///     [
///         token("{"),
///         block_indent(&format_args![
///             token("let a = 10;"),
///             hard_line_break(),
///             token("let c = a + 5;"),
///         ]),
///         token("}"),
///     ]
/// ].unwrap();
///
/// assert_eq!(
///     "{\n\tlet a = 10;\n\tlet c = a + 5;\n}",
///     block.print().as_code()
/// );
/// ```
#[inline]
pub fn block_indent<O>(content: &dyn Format<O>) -> BlockIndent<O> {
    BlockIndent {
        content,
        mode: IndentMode::Block,
    }
}

/// Indents the content by inserting a line break before and after the content and increasing
/// the indention level for the content by one if the enclosing group doesn't fit on a single line.
/// Doesn't change the formatting if the enclosing group fits on a single line.
///
/// ## Examples
///
/// Indents the content by one level and puts in new lines if the enclosing `Group` doesn't fit on a single line
///
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(10).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("'First string',"),
///             soft_line_break_or_space(),
///             token("'second string',"),
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[\n\t'First string',\n\t'second string',\n]",
///     elements.print().as_code()
/// );
/// ```
///
/// Doesn't change the formatting if the enclosing `Group` fits on a single line
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("5,"),
///             soft_line_break_or_space(),
///             token("10"),
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[5, 10]",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub fn soft_block_indent<O>(content: &dyn Format<O>) -> BlockIndent<O> {
    BlockIndent {
        content,
        mode: IndentMode::Soft,
    }
}

/// If the enclosing `Group` doesn't fit on a single line, inserts a line break and indent.
/// Otherwise, just inserts a space.
///
/// Line indents are used to break a single line of code, and therefore only insert a line
/// break before the content and not after the content.
///
/// ## Examples
///
/// Indents the content by one level and puts in new lines if the enclosing `Group` doesn't
/// fit on a single line. Otherwise, just inserts a space.
///
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(10).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("name"),
///         space_token(),
///         token("="),
///         soft_line_indent_or_space(&format_args![
///             token("firstName"),
///             space_token(),
///             token("+"),
///             space_token(),
///             token("lastName"),
///         ]),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "name =\n\tfirstName + lastName",
///     elements.print().as_code()
/// );
/// ```
///
/// Only adds a space if the enclosing `Group` fits on a single line
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("a"),
///         space_token(),
///         token("="),
///         soft_line_indent_or_space(&token("10")),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "a = 10",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub fn soft_line_indent_or_space<O>(content: &dyn Format<O>) -> BlockIndent<O> {
    BlockIndent {
        content,
        mode: IndentMode::SoftLineOrSpace,
    }
}

#[derive(Copy, Clone)]
pub struct BlockIndent<'a, O> {
    content: &'a dyn Format<O>,
    mode: IndentMode,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum IndentMode {
    Soft,
    Block,
    SoftLineOrSpace,
}

impl<O> Format<O> for BlockIndent<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        match self.mode {
            IndentMode::Soft => write!(buffer, [soft_line_break()])?,
            IndentMode::Block => write!(buffer, [hard_line_break()])?,
            IndentMode::SoftLineOrSpace => write!(buffer, [soft_line_break_or_space()])?,
        };

        write!(buffer, [self.content])?;

        // Don't create an indent if the content is empty
        if buffer.len() == 1 {
            return Ok(());
        }

        let content = buffer.into_document().into_element();

        f.write_element(FormatElement::Indent(Box::new(content)));

        match self.mode {
            IndentMode::Soft => write!(f, [soft_line_break()])?,
            IndentMode::Block => write!(f, [hard_line_break()])?,
            IndentMode::SoftLineOrSpace => {}
        }

        Ok(())
    }
}

/// Creates a logical `Group` around the content that should either consistently be printed on a single line
/// or broken across multiple lines.
///
/// The printer will try to print the content of the `Group` on a single line, ignoring all soft line breaks and
/// emitting spaces for soft line breaks or spaces. The printer tracks back if it isn't successful either
/// because it encountered a hard line break, or because printing the `Group` on a single line exceeds
/// the configured line width, and thus it must print all its content on multiple lines,
/// emitting line breaks for all line break kinds.
///
/// ## Examples
///
/// `Group` that fits on a single line
///
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("1,"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[1, 2, 3]",
///     elements.print().as_code()
/// );
/// ```
///
/// The printer breaks the `Group` over multiple lines if its content doesn't fit on a single line
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(20).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("'Good morning! How are you today?',"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub const fn group_elements<O>(content: &dyn Format<O>) -> GroupElements<O> {
    GroupElements {
        content,
        options: GroupElementsOptions { group_id: None },
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct GroupElementsOptions {
    pub group_id: Option<GroupId>,
}

/// Creates a group with a specific id. Useful for cases where `if_group_breaks` and `if_group_fits_on_line`
/// shouldn't refer to the direct parent group.
pub const fn group_elements_with_options<O>(
    content: &dyn Format<O>,
    options: GroupElementsOptions,
) -> GroupElements<O> {
    GroupElements { content, options }
}

#[derive(Copy, Clone)]
pub struct GroupElements<'a, O> {
    content: &'a dyn Format<O>,
    options: GroupElementsOptions,
}

impl<O> Format<O> for GroupElements<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        write!(buffer, [self.content])?;

        let content = buffer.into_document().into_element();

        let (leading, content, trailing) = content.split_trivia();
        let group = Group::new(content).with_id(self.options.group_id);

        if !leading.is_empty() {
            f.write_element(leading);
        }
        f.write_element(FormatElement::Group(group));

        if !trailing.is_empty() {
            f.write_element(trailing);
        }

        Ok(())
    }
}

/// IR element that forces the parent group to print in expanded mode.
///
/// Has no effect if used outside of a group or element that introduce implicit groups (fill element).
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("'Good morning! How are you today?',"),
///             soft_line_break_or_space(),
///             token("2,"),
///             expand_parent(), // Forces the parent to expand
///             soft_line_break_or_space(),
///             token("3"),
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
///     elements.print().as_code()
/// );
/// ```
///
/// ## Prettier
/// Equivalent to Prettier's `break_parent` IR element
pub const fn expand_parent() -> ExpandParent {
    ExpandParent
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ExpandParent;

impl<O> Format<O> for ExpandParent {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        f.write_element(FormatElement::ExpandParent);
        Ok(())
    }
}

/// Adds a conditional content that is emitted only if it isn't inside an enclosing `Group` that
/// is printed on a single line. The element allows, for example, to insert a trailing comma after the last
/// array element only if the array doesn't fit on a single line.
///
/// The element has no special meaning if used outside of a `Group`. In that case, the content is always emitted.
///
/// If you're looking for a way to only print something if the `Group` fits on a single line see [if_group_fits_on_single_line].
///
/// ## Examples
///
/// Omits the trailing comma for the last array element if the `Group` fits on a single line
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let elements = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("1,"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///             if_group_breaks(&token(","))
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[1, 2, 3]",
///     elements.print().as_code()
/// );
/// ```
///
/// Prints the trailing comma for the last array element if the `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{format_args, format, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(20).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let elements = format!(options, [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("'A somewhat longer string to force a line break',"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///             if_group_breaks(&token(","))
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(20).unwrap(),
///     ..PrinterOptions::default()
/// };
/// assert_eq!(
///     "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3,\n]",
///     elements.print().as_code()
/// );
/// ```
#[inline]
pub fn if_group_breaks<O>(content: &dyn Format<O>) -> IfGroupBreaks<O> {
    IfGroupBreaks {
        content,
        group_id: None,
        mode: PrintMode::Expanded,
    }
}

/// Inserts some content that the printer only prints if the group with the specified `group_id`
/// is printed in multiline mode. The referred group must appear before this element in the document
/// but doesn't have to one of its ancestors.
///
/// ## Examples
///
/// Prints the trailing comma if the array group doesn't fit. The `group_id` is necessary
/// because `fill` creates an implicit group around each item and tries to print the item in flat mode.
/// The item `[4]` in this example fits on a single line but the trailing comma should still be printed
///
/// ```
/// use rome_formatter::{format, format_args, write, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(20).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let formatted = format!(options, [format_with(|f| {
///     let group_id = f.group_id("array");
///
///     write!(f, [
///         group_elements_with_options(
///             &format_args![
///                 token("["),
///                 soft_block_indent(&format_with(|f| {
///                     f.fill(&soft_line_break_or_space())
///                         .entry(&token("1,"))
///                         .entry(&token("234568789,"))
///                         .entry(&token("3456789,"))
///                         .entry(&format_args!(
///                             token("["),
///                             soft_block_indent(&token("4")),
///                             token("]"),
///                             if_group_with_id_breaks(&token(","), group_id)
///                         ))
///                         .finish()
///                 })),
///                 token("]")
///             ],
///             GroupElementsOptions { group_id: Some(group_id) }
///         )
///     ])
/// })]).unwrap();
///
/// assert_eq!(
///     "[\n\t1, 234568789,\n\t3456789, [4],\n]",
///     formatted.print().as_code()
/// );
/// ```
pub const fn if_group_with_id_breaks<O>(
    content: &dyn Format<O>,
    group_id: GroupId,
) -> IfGroupBreaks<O> {
    IfGroupBreaks {
        content,
        group_id: Some(group_id),
        mode: PrintMode::Expanded,
    }
}

/// Adds a conditional content specific for `Group`s that fit on a single line. The content isn't
/// emitted for `Group`s spanning multiple lines.
///
/// See [if_group_breaks] if you're looking for a way to print content only for groups spanning multiple lines.
///
/// ## Examples
///
/// Adds the trailing comma for the last array element if the `Group` fits on a single line
/// ```
/// use rome_formatter::{format, format_args};
/// use rome_formatter::prelude::*;
///
/// let formatted = format!(SimpleFormatContext::default(), [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("1,"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///             if_group_fits_on_single_line(&token(","))
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[1, 2, 3,]",
///     formatted.print().as_code()
/// );
/// ```
///
/// Omits the trailing comma for the last array element if the `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{format, format_args, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let options = SimpleFormatContext {
///     line_width: LineWidth::try_from(20).unwrap(),
///     ..SimpleFormatContext::default()
/// };
///
/// let formatted = format!(options, [
///     group_elements(&format_args![
///         token("["),
///         soft_block_indent(&format_args![
///             token("'A somewhat longer string to force a line break',"),
///             soft_line_break_or_space(),
///             token("2,"),
///             soft_line_break_or_space(),
///             token("3"),
///             if_group_fits_on_single_line(&token(","))
///         ]),
///         token("]"),
///     ])
/// ]).unwrap();
///
/// assert_eq!(
///     "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3\n]",
///     formatted.print().as_code()
/// );
/// ```
#[inline]
pub const fn if_group_fits_on_single_line<O>(flat_content: &dyn Format<O>) -> IfGroupBreaks<O> {
    IfGroupBreaks {
        mode: PrintMode::Flat,
        group_id: None,
        content: flat_content,
    }
}

/// Inserts some content that the printer only prints if the group with the specified `group_id`
/// is printed in flat mode.
///
#[inline]
pub const fn if_group_with_id_fits_on_line<O>(
    flat_content: &dyn Format<O>,
    id: GroupId,
) -> IfGroupBreaks<O> {
    IfGroupBreaks {
        mode: PrintMode::Flat,
        group_id: Some(id),
        content: flat_content,
    }
}

#[derive(Copy, Clone)]
pub struct IfGroupBreaks<'a, O> {
    content: &'a dyn Format<O>,
    group_id: Option<GroupId>,
    mode: PrintMode,
}

impl<O> Format<O> for IfGroupBreaks<'_, O> {
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        write!(buffer, [self.content])?;

        if buffer.is_empty() {
            return Ok(());
        }

        let content = buffer.into_document().into_element();
        f.write_element(FormatElement::ConditionalGroupContent(
            ConditionalGroupContent::new(content, self.mode).with_group_id(self.group_id),
        ));
        Ok(())
    }
}

pub struct JoinBuilder<'fmt, 'joiner, 'buf, O> {
    result: super::FormatResult<()>,
    fmt: &'fmt mut Formatter<'buf, O>,
    with: Option<&'joiner dyn Format<O>>,
    has_elements: bool,
}

impl<'fmt, 'joiner, 'buf, O> JoinBuilder<'fmt, 'joiner, 'buf, O> {
    pub(crate) fn new(fmt: &'fmt mut Formatter<'buf, O>) -> Self {
        Self {
            result: Ok(()),
            fmt,
            has_elements: false,
            with: None,
        }
    }

    pub(crate) fn with(fmt: &'fmt mut Formatter<'buf, O>, with: &'joiner dyn Format<O>) -> Self {
        Self {
            result: Ok(()),
            fmt,
            has_elements: false,
            with: Some(with),
        }
    }

    pub fn entry(&mut self, entry: &dyn Format<O>) -> &mut Self {
        self.result = self.result.and_then(|_| {
            if let Some(with) = &self.with {
                if self.has_elements {
                    with.format(self.fmt)?;
                }
            }
            self.has_elements = true;

            entry.format(self.fmt)
        });

        self
    }

    pub fn entries<F, I>(&mut self, entries: I) -> &mut Self
    where
        F: Format<O>,
        I: IntoIterator<Item = F>,
    {
        for entry in entries {
            self.entry(&entry);
        }

        self
    }

    pub fn finish(&mut self) -> super::FormatResult<()> {
        self.result
    }
}

pub struct JoinNodesBuilder<'fmt, 'buf, Sep, O> {
    result: super::FormatResult<()>,
    separator: Sep,
    fmt: &'fmt mut Formatter<'buf, O>,
    has_elements: bool,
}

impl<'fmt, 'buf, Sep, O> JoinNodesBuilder<'fmt, 'buf, Sep, O>
where
    Sep: Format<O>,
{
    pub(super) fn new(separator: Sep, fmt: &'fmt mut Formatter<'buf, O>) -> Self {
        Self {
            result: Ok(()),
            separator,
            fmt,
            has_elements: false,
        }
    }

    pub fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<O>) {
        self.result = self.result.and_then(|_| {
            if self.has_elements {
                if get_lines_before(node) > 1 {
                    write!(self.fmt, [empty_line()])?;
                } else {
                    write!(self.fmt, [&self.separator])?;
                }
            }

            self.has_elements = true;

            write!(self.fmt, [content])
        });
    }

    pub fn entries<L, F, I>(&mut self, entries: I) -> &mut Self
    where
        L: Language,
        F: Format<O>,
        I: IntoIterator<Item = (F, SyntaxNode<L>)>,
    {
        for (content, node) in entries {
            self.entry(&node, &content)
        }

        self
    }

    pub fn finish(&mut self) -> FormatResult<()> {
        self.result
    }
}

/// Get the number of line breaks between two consecutive SyntaxNodes in the tree
pub fn get_lines_before<L: Language>(next_node: &SyntaxNode<L>) -> usize {
    // Count the newlines in the leading trivia of the next node
    if let Some(leading_trivia) = next_node.first_leading_trivia() {
        leading_trivia
            .pieces()
            .take_while(|piece| {
                // Stop at the first comment piece, the comment printer
                // will handle newlines between the comment and the node
                !piece.is_comments()
            })
            .filter(|piece| piece.is_newline())
            .count()
    } else {
        0
    }
}

pub struct FillBuilder<'fmt, 'separator, 'buf, O> {
    result: FormatResult<()>,
    fmt: &'fmt mut Formatter<'buf, O>,
    separator: &'separator dyn Format<O>,
    items: Vec<FormatElement>,
}

impl<'a, 'separator, 'buf, O> FillBuilder<'a, 'separator, 'buf, O> {
    pub(crate) fn new(
        fmt: &'a mut Formatter<'buf, O>,
        separator: &'separator dyn Format<O>,
    ) -> Self {
        Self {
            result: Ok(()),
            fmt,
            separator,
            items: vec![],
        }
    }

    pub fn entries<F, I>(&mut self, entries: I) -> &mut Self
    where
        F: Format<O>,
        I: IntoIterator<Item = F>,
    {
        for entry in entries {
            self.entry(&entry);
        }

        self
    }

    pub fn entry(&mut self, entry: &dyn Format<O>) -> &mut Self {
        self.result = self.result.and_then(|_| {
            let mut buffer = VecBuffer::new(self.fmt.state_mut());
            write!(buffer, [entry])?;

            let item = buffer.into_document().into_element();

            if !item.is_empty() {
                self.items.push(item);
            }

            Ok(())
        });

        self
    }

    pub fn finish(&mut self) -> super::FormatResult<()> {
        self.result.map(|_| {
            let mut items = std::mem::take(&mut self.items);

            match items.len() {
                0 => (),
                1 => {
                    self.fmt.write_element(items.pop().unwrap());
                }
                _ => {
                    self.fmt
                        .write_element(FormatElement::Fill(List::new(items)));
                }
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrailingSeparator {
    Allowed,
    Disallowed,
    Mandatory,
}

impl TrailingSeparator {
    pub fn is_allowed(&self) -> bool {
        matches!(self, TrailingSeparator::Allowed)
    }
    pub fn is_mandatory(&self) -> bool {
        matches!(self, TrailingSeparator::Mandatory)
    }
}

impl Default for TrailingSeparator {
    fn default() -> Self {
        TrailingSeparator::Allowed
    }
}

#[derive(Copy, Clone)]
pub struct FormatWith<O, T>
where
    T: Fn(&mut Formatter<O>) -> FormatResult<()>,
{
    closure: T,
    options: PhantomData<O>,
}

impl<O, T> Format<O> for FormatWith<O, T>
where
    T: Fn(&mut Formatter<O>) -> FormatResult<()>,
{
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        (self.closure)(f)
    }
}

pub const fn format_with<O, T>(closure: T) -> FormatWith<O, T>
where
    T: Fn(&mut Formatter<O>) -> FormatResult<()>,
{
    FormatWith {
        closure,
        options: PhantomData,
    }
}

pub const fn format_once<T, O>(closure: T) -> FormatOnce<T, O>
where
    T: FnOnce(&mut Formatter<O>) -> FormatResult<()>,
{
    FormatOnce {
        closure: Cell::new(Some(closure)),
        options: PhantomData,
    }
}

pub struct FormatOnce<T, O> {
    closure: Cell<Option<T>>,
    options: PhantomData<O>,
}

impl<T, O> Format<O> for FormatOnce<T, O>
where
    T: FnOnce(&mut Formatter<O>) -> FormatResult<()>,
{
    fn format(&self, f: &mut Formatter<O>) -> FormatResult<()> {
        let closure = self.closure.take().expect("Tried to format once at least twice. This is not allowed. You may want to use format_with or .memoized instead");

        (closure)(f)
    }
}

#[derive(Default)]
pub struct ConcatBuilder {
    elements: Vec<FormatElement>,
    size_hint: Option<usize>,
}

impl ConcatBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            elements: vec![],
            size_hint: None,
        }
    }

    #[inline]
    pub fn entry(&mut self, element: FormatElement) {
        if element.is_empty() {
            return;
        }

        if self.elements.is_empty() && self.size_hint.is_some() {
            // SAFETY: Guaranteed by the `is_some` check above
            let size_hint = self.size_hint.unwrap();

            match element {
                FormatElement::List(list) => {
                    self.elements = list.into_vec();
                    self.elements.reserve(size_hint - 1);
                }
                item => {
                    self.elements.reserve(size_hint);
                    self.elements.push(item);
                }
            }
        } else {
            match element {
                FormatElement::List(list) => self.elements.extend(list.into_vec()),
                item => self.elements.push(item),
            }
        }
    }

    #[inline]
    pub fn size_hint(&mut self, hint: (usize, Option<usize>)) {
        let (lower_bound, upper_bound) = hint;

        if let Some(upper_bound) = upper_bound {
            debug_assert!(lower_bound <= upper_bound, "Expected lower bound {lower_bound} to be less than or equal to upper bound {upper_bound}");
            self.size_hint = Some(upper_bound);
        } else {
            self.size_hint = Some(lower_bound);
        }
    }

    #[inline]
    pub fn finish(mut self) -> FormatElement {
        if self.elements.len() == 1 {
            // Safety: Guaranteed to succeed by the length check above
            self.elements.pop().unwrap()
        } else {
            FormatElement::List(List::new(self.elements))
        }
    }
}
