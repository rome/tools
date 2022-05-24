use crate::builders::ConcatBuilder;
use crate::intersperse::{Intersperse, IntersperseFn};
use crate::{format_elements, GroupId, TextRange, TextSize};
#[cfg(target_pointer_width = "64")]
use rome_rowan::static_assert;
use rome_rowan::{
    Language, SyntaxNode, SyntaxToken, SyntaxTokenText, SyntaxTriviaPieceComments, TextLen,
};
use std::borrow::Cow;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

type Content = Box<FormatElement>;

/// Format element that doesn't represent any content.
///
/// Can be helpful if you need to return a `FormatElement` (e.g. in an else branch) but don't want
/// to show any content.
pub fn empty_element() -> FormatElement {
    FormatElement::Empty
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![token("a,"), soft_line_break(), token("b"),]);
///
/// assert_eq!(
///     "a,b",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
/// See [soft_line_break_or_space] if you want to insert a space between the elements if the enclosing
/// `Group` fits on a single line.
///
/// Soft line breaks are emitted if the enclosing `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("a long word,"),
///     soft_line_break(),
///     token("so that the group doesn't fit on a single line"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(10).unwrap(),
///     ..PrinterOptions::default()
/// };
///
/// assert_eq!(
///     "a long word,\nso that the group doesn't fit on a single line",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
#[inline]
pub const fn soft_line_break() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::Soft))
}

/// A forced line break that are always printed. A hard line break forces any enclosing `Group`
/// to be printed over multiple lines.
///
/// ## Examples
///
/// It forces a line break, even if the enclosing `Group` would otherwise fit on a single line.
/// ```
/// use rome_formatter::*;
/// use rome_formatter::prelude::PrinterOptions;
///
/// let elements = group_elements(format_elements![
///     token("a,"),
///     hard_line_break(),
///     token("b"),
///     hard_line_break()
/// ]);
///
/// assert_eq!(
///     "a,\nb\n",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub const fn hard_line_break() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::Hard))
}

/// A forced empty line. An empty line inserts enough line breaks in the output for
/// the previous and next element to be separated by an empty line.
///
/// ## Examples
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("a,"),
///     empty_line(),
///     token("b"),
///     empty_line()
/// ]);
///
/// assert_eq!(
///     "a,\n\nb\n\n",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub const fn empty_line() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::Empty))
}

/// A line break if the enclosing `Group` doesn't fit on a single line, a space otherwise.
///
/// ## Examples
///
/// The line breaks are emitted as spaces if the enclosing `Group` fits on a a single line:
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("a,"),
///     soft_line_break_or_space(),
///     token("b"),
/// ]);
///
/// assert_eq!(
///     "a, b",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
///
/// The printer breaks the lines if the enclosing `Group` doesn't fit on a single line:
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("a long word,"),
///     soft_line_break_or_space(),
///     token("so that the group doesn't fit on a single line"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(10).unwrap(),
///     ..PrinterOptions::default()
/// };
///
/// assert_eq!(
///     "a long word,\nso that the group doesn't fit on a single line",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
#[inline]
pub const fn soft_line_break_or_space() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::SoftOrSpace))
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
/// let elements = token("Hello World");
///
/// assert_eq!(
///     "Hello World",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
///
/// Printing a string literal as a literal requires that the string literal is properly escaped and
/// enclosed in quotes (depending on the target language).
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = token("\"Hello\\tWorld\"");
///
/// assert_eq!(r#""Hello\tWorld""#, Formatted::new(elements, PrinterOptions::default()).print().as_code());
/// ```
#[inline]
pub const fn token(text: &'static str) -> FormatElement {
    if text.is_empty() {
        FormatElement::Empty
    } else {
        FormatElement::Token(Token::new_static(text))
    }
}

/// Push a [FormatElement] to the end of the current line
///
/// ## Examples
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = format_elements![token("a"), line_suffix(token("c")), token("b")];
///
/// assert_eq!(
///     "abc",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn line_suffix(element: impl Into<FormatElement>) -> FormatElement {
    FormatElement::LineSuffix(Box::new(element.into()))
}

/// Inserts a boundary for line suffixes that forces to print all pending line suffixes. Helpful
/// if a line sufix shouldn't pass a certain point.
///
/// ## Examples
///
/// Forces the line suffix "c" to be printed before the token `d`.
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = format_elements![token("a"), line_suffix(token("c")), token("b"), line_suffix_boundary(), token("d")];
///
/// assert_eq!(
///     "abc\nd",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
pub const fn line_suffix_boundary() -> FormatElement {
    FormatElement::LineSuffixBoundary
}

/// Mark a [FormatElement] as being a piece of trivia
///
/// This does not directly influence how this content will be printed, but some
/// parts of the formatter may chose to handle this element in a specific way
///
/// ## Examples
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     comment(empty_line()),
///     token("a"),
///     soft_line_break_or_space(),
///     token("b")
/// ]);
///
/// assert_eq!(
///     "\na b",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn comment(element: impl Into<FormatElement>) -> FormatElement {
    FormatElement::Comment(Box::new(element.into()))
}

/// Inserts a single space. Allows to separate different tokens.
///
/// ## Examples
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = format_elements![token("a"), space_token(), token("b")];
///
/// assert_eq!("a b", Formatted::new(elements, PrinterOptions::default()).print().as_code());
/// ```
#[inline]
pub const fn space_token() -> FormatElement {
    FormatElement::Space
}

/// Concatenates the content of multiple [FormatElement]s.
///
/// ## Examples
///
/// ```rust
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
/// let expr = concat_elements(vec![
///     token("a"),
///     space_token(),
///     token("+"),
///     space_token(),
///     token("b"),
/// ]);
///
/// assert_eq!(
///     "a + b",
///     Formatted::new(expr, PrinterOptions::default())
///         .print()
///         .as_code()
/// )
/// ```
pub fn concat_elements<I>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = FormatElement>,
{
    let elements = elements.into_iter();
    let mut builder = ConcatBuilder::new();

    builder.size_hint(elements.size_hint());

    for element in elements {
        builder.entry(element);
    }

    builder.finish()
}

/// Concatenates a list of [FormatElement]s with spaces and line breaks to fit
/// them on as few lines as possible. Each element introduces a conceptual group. The printer
/// first tries to print the item in flat mode but then prints it in expanded mode if it doesn't fit.
///
/// ## Examples
///
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::Formatted;
/// use std::str::from_utf8;
///
/// let a = from_utf8(&[b'a'; 30]).unwrap();
/// let b = from_utf8(&[b'b'; 30]).unwrap();
/// let c = from_utf8(&[b'c'; 30]).unwrap();
/// let d = from_utf8(&[b'd'; 30]).unwrap();
/// let expr = fill_elements(space_token(), [token(a), token(b), token(c), token(d)]);
///
/// assert_eq!(
///     format!("{a} {b}\n{c} {d}"),
///     Formatted::new(expr, PrinterOptions::default())
///         .print()
///         .as_code()
/// )
/// ```
/// ```rust
/// use rome_formatter::prelude::*;
/// use rome_formatter::Formatted;
/// use std::str::from_utf8;
///
/// let a = "<b>Important: </b>";
/// let b = "Please do not commit memory bugs such as segfaults, buffer overflows, etc. otherwise you ";
/// let c = "<em>will</em>";
/// let d = " be reprimanded";
/// let expr = fill_elements(empty_element(), [token(a), token(b), token(c), token(d)]);
///
/// assert_eq!(
///     format!("{a}\n{b}\n{c}{d}"),
///     Formatted::new(expr, PrinterOptions::default())
///         .print()
///         .as_code()
/// )
/// ```
pub fn fill_elements<TSep: Into<FormatElement>>(
    separator: TSep,
    elements: impl IntoIterator<Item = FormatElement>,
) -> FormatElement {
    let mut list: Vec<_> = elements.into_iter().collect();
    match list.len() {
        0 => empty_element(),
        1 => list.pop().unwrap(),
        _ => FormatElement::Fill(Box::new(Fill {
            list: List::new(list),
            separator: separator.into(),
        })),
    }
}

/// Joins the elements by placing a given separator between elements.
///
/// ## Examples
///
/// Joining different tokens by separating them with a comma and a space.
///
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let separator = concat_elements(vec![token(","), space_token()]);
/// let elements = join_elements(
///     separator,
///     vec![token("1"), token("2"), token("3"), token("4")],
/// );
///
/// assert_eq!(
///     "1, 2, 3, 4",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn join_elements<TSep, I>(separator: TSep, elements: I) -> FormatElement
where
    TSep: Into<FormatElement>,
    I: IntoIterator<Item = FormatElement>,
{
    concat_elements(Intersperse::new(
        elements.into_iter().filter(|e| !e.is_empty()),
        separator.into(),
    ))
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let block = (format_elements![
///     token("switch {"),
///     block_indent(format_elements![
///         token("default:"),
///         indent(format_elements![
///             // this is where we want to use a
///             hard_line_break(),
///             token("break;"),
///         ])
///     ]),
///     token("}"),
/// ]);
///
/// assert_eq!(
///     "switch {\n\tdefault:\n\t\tbreak;\n}",
///     Formatted::new(block, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn indent<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        format_elements![Indent::new(format_elements![content])]
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let block = (format_elements![
///     token("{"),
///     block_indent(format_elements![
///         token("let a = 10;"),
///         hard_line_break(),
///         token("let c = a + 5;"),
///     ]),
///     token("}"),
/// ]);
///
/// assert_eq!(
///     "{\n\tlet a = 10;\n\tlet c = a + 5;\n}",
///     Formatted::new(block, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn block_indent<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        format_elements![
            Indent::new(format_elements![hard_line_break(), content]),
            hard_line_break(),
        ]
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
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("'First string',"),
///         soft_line_break_or_space(),
///         token("'second string',"),
///     ]),
///     token("]"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(10).unwrap(),
///     ..PrinterOptions::default()
/// };
///
/// assert_eq!(
///     "[\n\t'First string',\n\t'second string',\n]",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
///
/// Doesn't change the formatting if the enclosing `Group` fits on a single line
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("5,"),
///         soft_line_break_or_space(),
///         token("10"),
///     ]),
///     token("]"),
/// ]);
///
/// assert_eq!(
///     "[5, 10]",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn soft_block_indent<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        format_elements![
            Indent::new(format_elements![soft_line_break(), content]),
            soft_line_break(),
        ]
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
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("name"),
///     space_token(),
///     token("="),
///     soft_line_indent_or_space(format_elements![
///         token("firstName"),
///         space_token(),
///         token("+"),
///         space_token(),
///         token("lastName"),
///     ]),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(10).unwrap(),
///     ..PrinterOptions::default()
/// };
///
/// assert_eq!(
///     "name =\n\tfirstName + lastName",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
///
/// Only adds a space if the enclosing `Group` fits on a single line
/// ```
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("a"),
///     space_token(),
///     token("="),
///     soft_line_indent_or_space(format_elements![token("10")]),
/// ]);
///
/// assert_eq!(
///     "a = 10",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
#[inline]
pub fn soft_line_indent_or_space<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        format_elements![Indent::new(format_elements![
            soft_line_break_or_space(),
            content
        ])]
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("1,"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///     ]),
///     token("]"),
/// ]);
///
/// assert_eq!(
///     "[1, 2, 3]",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
///
/// The printer breaks the `Group` over multiple lines if its content doesn't fit on a single line
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("'Good morning! How are you today?',"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///     ]),
///     token("]"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(20).unwrap(),
///     ..PrinterOptions::default()
/// };
///
/// assert_eq!(
///     "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
#[inline]
pub fn group_elements<T: Into<FormatElement>>(content: T) -> FormatElement {
    group_elements_with_options(content.into(), GroupElementsOptions::default())
}

#[derive(Default, Clone, Debug)]
pub struct GroupElementsOptions {
    pub group_id: Option<GroupId>,
}

/// Creates a group with a specific id. Useful for cases where `if_group_breaks` and `if_group_fits_on_line`
/// shouldn't refer to the direct parent group.
pub fn group_elements_with_options(
    content: FormatElement,
    options: GroupElementsOptions,
) -> FormatElement {
    if content.is_empty() {
        content
    } else {
        let (leading, content, trailing) = content.split_trivia();

        let group = Group::new(content).with_id(options.group_id);

        format_elements![leading, group, trailing]
    }
}

/// IR element that forces the parent group to print in expanded mode.
///
/// Has no effect if used outside of a group or element that introduce implicit groups (fill element).
///
/// ## Examples
///
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("'Good morning! How are you today?',"),
///         soft_line_break_or_space(),
///         token("2,"),
///         expand_parent(), // Forces the parent to expand
///         soft_line_break_or_space(),
///         token("3"),
///     ]),
///     token("]"),
/// ]);
///
/// assert_eq!(
///     "[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]",
///     Formatted::new(elements, PrinterOptions::default()).print().as_code()
/// );
/// ```
///
/// ## Prettier
/// Equivalent to Prettier's `break_parent` IR element
pub const fn expand_parent() -> FormatElement {
    FormatElement::ExpandParent
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("1,"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///         if_group_breaks(token(","))
///     ]),
///     token("]"),
/// ]);
/// assert_eq!(
///     "[1, 2, 3]",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
///
/// Prints the trailing comma for the last array element if the `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("'A somewhat longer string to force a line break',"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///         if_group_breaks(token(","))
///     ]),
///     token("]"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(20).unwrap(),
///     ..PrinterOptions::default()
/// };
/// assert_eq!(
///     "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3,\n]",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
#[inline]
pub fn if_group_breaks<T: Into<FormatElement>>(content: T) -> FormatElement {
    if_group_breaks_impl(content.into(), None)
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
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let formatter = Formatter::<()>::default();
/// let group_id = formatter.group_id("array");
///
/// let elements = group_elements_with_options(format_elements![
///     token("["),
///     soft_block_indent(fill_elements(space_token(), vec![
///         format_elements![token("1,")],
///         format_elements![token("234568789,")],
///         format_elements![token("3456789,")],
///         format_elements![
///             token("["),
///             soft_block_indent(token("4")),
///             token("]"),
///             if_group_with_id_breaks(token(","), group_id)
///         ],
///     ])),
///     token("]"),
/// ], GroupElementsOptions { group_id: Some(group_id) });
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(20).unwrap(),
///     ..PrinterOptions::default()
/// };
/// assert_eq!(
///     "[\n\t1, 234568789,\n\t3456789, [4],\n]",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
pub fn if_group_with_id_breaks(content: FormatElement, group_id: GroupId) -> FormatElement {
    if_group_breaks_impl(content, Some(group_id))
}

fn if_group_breaks_impl(content: FormatElement, group_id: Option<GroupId>) -> FormatElement {
    if content.is_empty() {
        content
    } else {
        FormatElement::from(
            ConditionalGroupContent::new(content, PrintMode::Expanded).with_group_id(group_id),
        )
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
/// use rome_formatter::Formatted;
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("1,"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///         if_group_fits_on_single_line(token(","))
///     ]),
///     token("]"),
/// ]);
/// assert_eq!(
///     "[1, 2, 3,]",
///     Formatted::new(elements, PrinterOptions::default())
///         .print()
///         .as_code()
/// );
/// ```
///
/// Omits the trailing comma for the last array element if the `Group` doesn't fit on a single line
/// ```
/// use rome_formatter::{Formatted, LineWidth};
/// use rome_formatter::prelude::*;
///
/// let elements = group_elements(format_elements![
///     token("["),
///     soft_block_indent(format_elements![
///         token("'A somewhat longer string to force a line break',"),
///         soft_line_break_or_space(),
///         token("2,"),
///         soft_line_break_or_space(),
///         token("3"),
///         if_group_fits_on_single_line(token(","))
///     ]),
///     token("]"),
/// ]);
///
/// let options = PrinterOptions {
///     print_width: LineWidth::try_from(20).unwrap(),
///     ..PrinterOptions::default()
/// };
/// assert_eq!(
///     "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3\n]",
///     Formatted::new(elements, options).print().as_code()
/// );
/// ```
#[inline]
pub fn if_group_fits_on_single_line<TFlat>(flat_content: TFlat) -> FormatElement
where
    TFlat: Into<FormatElement>,
{
    if_group_fits_on_line_impl(flat_content.into(), None)
}

/// Inserts some content that the printer only prints if the group with the specified `group_id`
/// is printed in flat mode.
///
#[inline]
pub fn if_group_with_id_fits_on_line(flat_content: FormatElement, id: GroupId) -> FormatElement {
    if_group_fits_on_line_impl(flat_content, Some(id))
}

fn if_group_fits_on_line_impl(
    flat_content: FormatElement,
    group_id: Option<GroupId>,
) -> FormatElement {
    if flat_content.is_empty() {
        flat_content
    } else {
        FormatElement::from(
            ConditionalGroupContent::new(flat_content, PrintMode::Flat).with_group_id(group_id),
        )
    }
}

/// Specialized version of [join_elements] for joining SyntaxNodes separated by a space, soft
/// line break or empty line depending on the input file.
///
/// This functions inspects the input source and separates consecutive elements with either
/// a [soft_line_break_or_space] or [empty_line] depending on how many line breaks were
/// separating the elements in the original file.
#[inline]
pub fn join_elements_soft_line<I, L>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (SyntaxNode<L>, FormatElement)>,
    L: Language,
{
    join_elements_with(elements, soft_line_break_or_space)
}

/// Specialized version of [join_elements] for joining SyntaxNodes separated by one or more
/// line breaks depending on the input file.
///
/// This functions inspects the input source and separates consecutive elements with either
/// a [hard_line_break] or [empty_line] depending on how many line breaks were separating the
/// elements in the original file.
#[inline]
pub fn join_elements_hard_line<I, L>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (SyntaxNode<L>, FormatElement)>,
    L: Language,
{
    join_elements_with(elements, hard_line_break)
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

#[inline]
pub fn join_elements_with<I, L>(elements: I, separator: fn() -> FormatElement) -> FormatElement
where
    I: IntoIterator<Item = (SyntaxNode<L>, FormatElement)>,
    L: Language,
{
    concat_elements(IntersperseFn::new(
        elements.into_iter(),
        |_, next_node, next_elem| {
            if next_elem.is_empty() {
                empty_element()
            } else if get_lines_before(next_node) > 1 {
                empty_line()
            } else {
                separator()
            }
        },
    ))
}

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [crate::space_token], [crate::soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq)]
pub enum FormatElement {
    Empty,

    /// A space token, see [crate::space_token] for documentation.
    Space,

    /// A new line, see [crate::soft_line_break], [crate::hard_line_break], and [crate::soft_line_break_or_space] for documentation.
    Line(Line),

    /// Indents the content one level deeper, see [crate::indent] for documentation and examples.
    Indent(Indent),

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

    /// Concatenates multiple elements together with a given separator or line breaks to fill the print width. See [fill_elements].
    Fill(Box<Fill>),

    /// A token that should be printed as is, see [token] for documentation and examples.
    Token(Token),

    /// Delay the printing of its content until the next line break
    LineSuffix(Content),

    /// Prevents that line suffixes move past this boundary. Forces the printer to print any pending
    /// line suffixes, potentially by inserting a hard line break.
    LineSuffixBoundary,

    /// Special semantic element letting the printer and formatter know this is
    /// a trivia content, and it should only have a limited influence on the
    /// formatting (for instance line breaks contained within will not cause
    /// the parent group to break if this element is at the start of it)
    Comment(Content),

    /// A token that tracks tokens/nodes that are printed using [`format_verbatim`](crate::Formatter::format_verbatim) API
    Verbatim(Verbatim),

    /// A list of different variants representing the same content. The printer picks the best fitting content.
    /// Line breaks inside of a best fitting don't propagate to parent groups.
    BestFitting(BestFitting),
}

#[derive(Clone, Eq, PartialEq)]
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
            FormatElement::Empty => write!(fmt, "Empty"),
            FormatElement::Space => write!(fmt, "Space"),
            FormatElement::Line(content) => content.fmt(fmt),
            FormatElement::Indent(content) => content.fmt(fmt),
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
        }
    }
}

/// Inserts a new line
#[derive(Clone, Eq, PartialEq)]
pub struct Line {
    pub mode: LineMode,
}

impl Debug for Line {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Line({:?})", self.mode)
    }
}

impl Line {
    pub const fn new(mode: LineMode) -> Self {
        Self { mode }
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

/// Increases the indention by one; see [indented_with_soft_break] and [indented_with_hard_break].
#[derive(Clone, Eq, PartialEq)]
pub struct Indent {
    pub(crate) content: Content,
}

impl Debug for Indent {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_tuple("Indent").field(&self.content).finish()
    }
}

impl Indent {
    pub fn new(content: FormatElement) -> Self {
        Self {
            content: Box::new(content),
        }
    }
}

/// A token used to gather a list of elements; see [concat_elements] and [join_elements].
#[derive(Clone, Eq, PartialEq)]
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

    pub(crate) fn into_vec(self) -> Vec<FormatElement> {
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
    list: List,
    separator: FormatElement,
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
    pub(crate) content: Content,
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
    pub fn new(content: FormatElement) -> Self {
        Self {
            content: Box::new(content),
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
    pub unsafe fn from_slice_unchecked(variants: &[FormatElement]) -> Self {
        debug_assert!(
            variants.len() >= 2,
            "Requires at least the least expanded and most expanded variants"
        );

        Self {
            variants: Vec::from(variants).into_boxed_slice(),
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
    /// Create a token from a static string
    const fn new_static(text: &'static str) -> Self {
        Self::Static { text }
    }

    /// Create a token from a dynamic string and a range of the input source
    pub fn new_dynamic(text: String, position: TextSize) -> Self {
        debug_assert_no_newlines(&text);

        Self::Dynamic {
            text: text.into_boxed_str(),
            source_position: position,
        }
    }

    /// Creates a token from a [Cow] that is a sub-slice over the text of a token.
    ///
    /// The `start` is the absolute start of the token in the source text.
    ///
    /// ## Returns
    /// * [Token::Dynamic] if `text` is a [Cow::Owned] (text doesn't match syntax token text)
    /// * [Token::SyntaxTokenSlice] if `text` is borrowed. Avoids allocating a new string.
    pub fn from_syntax_token_cow_slice<L: Language>(
        text: Cow<str>,
        token: &SyntaxToken<L>,
        start: TextSize,
    ) -> Self {
        match text {
            Cow::Owned(text) => Self::new_dynamic(text, start),
            Cow::Borrowed(text) => {
                let range = TextRange::at(start, text.text_len());
                debug_assert_eq!(
                    text,
                    &token.text()[range - token.text_range().start()],
                    "The borrowed string doesn't match the specified token substring. Does the borrowed string belong to this token and range?"
                );
                Token::new_syntax_token_slice(token, range)
            }
        }
    }

    /// Creates a new [Token] with a text backed by the string of [SyntaxToken]
    pub fn new_syntax_token_slice<L: Language>(token: &SyntaxToken<L>, range: TextRange) -> Self {
        let relative_range = range - token.text_range().start();
        let slice = token.token_text().slice(relative_range);

        debug_assert_no_newlines(&slice);

        Self::SyntaxTokenSlice {
            slice,
            source_position: range.start(),
        }
    }

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

fn debug_assert_no_newlines(text: &str) {
    debug_assert!(!text.contains('\r'), "The content '{}' contains an unsupported '\\r' line terminator character but string tokens must only use line feeds '\\n' as line separator. Use '\\n' instead of '\\r' and '\\r\\n' to insert a line break in strings.", text);
}

// Token equality only compares the text content
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<L: Language> From<SyntaxToken<L>> for Token {
    fn from(token: SyntaxToken<L>) -> Self {
        Self::from(&token)
    }
}

impl<'a, L: Language> From<&'a SyntaxToken<L>> for Token {
    fn from(token: &'a SyntaxToken<L>) -> Self {
        let trimmed_range = token.text_trimmed_range();

        Self::new_syntax_token_slice(token, trimmed_range)
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

impl<L: Language> From<SyntaxTriviaPieceComments<L>> for Token {
    fn from(trivia: SyntaxTriviaPieceComments<L>) -> Self {
        let range = trivia.text_range();
        Token::from_syntax_token_cow_slice(
            normalize_newlines(trivia.text().trim(), LINE_TERMINATORS),
            &trivia.as_piece().token(),
            range.start(),
        )
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
    pub const fn is_empty(&self) -> bool {
        matches!(self, FormatElement::Empty)
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
            FormatElement::Empty => false,
            FormatElement::Space => false,
            FormatElement::Line(line) => matches!(line.mode, LineMode::Hard | LineMode::Empty),
            FormatElement::Indent(indent) => indent.content.will_break(),
            FormatElement::Group(group) => group.content.will_break(),
            FormatElement::ConditionalGroupContent(group) => group.content.will_break(),
            FormatElement::List(list) => list.content.iter().any(FormatElement::will_break),
            FormatElement::Fill(fill) => {
                fill.list.content.iter().any(FormatElement::will_break)
                    || fill.separator.will_break()
            }
            FormatElement::Token(token) => token.contains('\n'),
            FormatElement::LineSuffix(_) => false,
            FormatElement::Comment(content) => content.will_break(),
            FormatElement::Verbatim(verbatim) => verbatim.element.will_break(),
            FormatElement::BestFitting(_) => false,
            FormatElement::LineSuffixBoundary => false,
            FormatElement::ExpandParent => true,
        }
    }

    /// Splits off the leading and trailing trivias (comments) from this [FormatElement]
    ///
    /// For [FormatElement::HardGroup], the trailing trivia
    /// is automatically moved  outside of the group. The group itself is then recreated around the
    /// content itself.
    pub fn split_trivia(self) -> (FormatElement, FormatElement, FormatElement) {
        match self {
            FormatElement::List(mut list) => {
                // Find the index of the first non-comment element in the list
                let content_start = list
                    .content
                    .iter()
                    .position(|elem| !matches!(elem, FormatElement::Comment(_)));

                // List contains at least one non trivia element.
                if let Some(content_start) = content_start {
                    let (leading, mut content) = if content_start > 0 {
                        let content = list.content.split_off(content_start);
                        (FormatElement::List(list), content)
                    } else {
                        // No leading trivia
                        (empty_element(), list.content)
                    };

                    let content_end = content
                        .iter()
                        .rposition(|elem| !matches!(elem, FormatElement::Comment(_)))
                        .expect("List guaranteed to contain at least one non trivia element.");
                    let trailing_start = content_end + 1;

                    let trailing = if trailing_start < content.len() {
                        FormatElement::List(List::new(content.split_off(trailing_start)))
                    } else {
                        empty_element()
                    };

                    (leading, FormatElement::List(List::new(content)), trailing)
                } else {
                    // All leading trivia
                    (FormatElement::List(list), empty_element(), empty_element())
                }
            }
            // Non-list elements are returned directly
            _ => (empty_element(), self, empty_element()),
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
            FormatElement::Empty | FormatElement::Line(_) | FormatElement::Comment(_) => None,

            FormatElement::Indent(indent) => indent.content.last_element(),
            FormatElement::Group(group) => group.content.last_element(),

            _ => Some(self),
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

impl From<ConditionalGroupContent> for FormatElement {
    fn from(token: ConditionalGroupContent) -> Self {
        FormatElement::ConditionalGroupContent(token)
    }
}

impl From<Line> for FormatElement {
    fn from(token: Line) -> Self {
        FormatElement::Line(token)
    }
}

impl From<Indent> for FormatElement {
    fn from(token: Indent) -> Self {
        FormatElement::Indent(token)
    }
}

impl From<Option<FormatElement>> for FormatElement {
    fn from(element: Option<FormatElement>) -> Self {
        element.unwrap_or_else(empty_element)
    }
}

#[cfg(test)]
mod tests {

    use crate::format_element::{
        empty_element, join_elements, normalize_newlines, List, LINE_TERMINATORS,
    };
    use crate::{concat_elements, space_token, token, FormatElement};

    #[test]
    fn concat_elements_returns_a_list_token_containing_the_passed_in_elements() {
        let concatenated = concat_elements(vec![token("a"), space_token(), token("b")]);

        assert_eq!(
            concatenated,
            FormatElement::List(List::new(vec![token("a"), space_token(), token("b")]))
        );
    }

    #[test]
    fn concat_elements_returns_the_passed_in_element_if_the_content_is_a_list_with_a_single_element(
    ) {
        let concatenated = concat_elements(vec![token("a")]);

        assert_eq!(concatenated, token("a"));
    }

    #[test]
    fn concat_elements_the_empty_element_if_the_passed_vector_is_empty() {
        let concatenated = concat_elements(vec![]);

        assert_eq!(concatenated, empty_element());
    }

    #[test]
    fn concat_elements_flattens_sub_lists_and_skips_empty_elements() {
        let concatenated = concat_elements(vec![
            token("a"),
            space_token(),
            empty_element(),
            concat_elements(vec![token("1"), space_token(), token("2")]),
            space_token(),
            token("b"),
        ]);

        assert_eq!(
            concatenated,
            FormatElement::List(List::new(vec![
                token("a"),
                space_token(),
                token("1"),
                space_token(),
                token("2"),
                space_token(),
                token("b")
            ]))
        );
    }

    #[test]
    fn join_elements_inserts_the_separator_between_elements() {
        let joined = join_elements(space_token(), vec![token("a"), token("b"), token("c")]);

        assert_eq!(
            joined,
            concat_elements(vec![
                token("a"),
                space_token(),
                token("b"),
                space_token(),
                token("c")
            ])
        );
    }

    #[test]
    fn join_returns_the_content_element_if_the_content_contains_a_single_element() {
        let joined = join_elements(space_token(), vec![token("a")]);

        assert_eq!(joined, token("a"));
    }

    #[test]
    fn join_returns_the_empty_element_if_the_passed_vec_is_empty() {
        let joined = join_elements(space_token(), vec![]);

        assert_eq!(joined, empty_element());
    }

    #[test]
    fn join_flattens_sub_lists_and_skips_empty_elements_without_inserting_separators() {
        let joined = join_elements(
            space_token(),
            vec![
                token("a"),
                empty_element(),
                concat_elements(vec![token("1"), token("+"), token("2")]),
                token("b"),
            ],
        );

        assert_eq!(
            joined,
            FormatElement::List(List::new(vec![
                token("a"),
                space_token(),
                token("1"),
                token("+"),
                token("2"),
                space_token(),
                token("b")
            ]))
        );
    }

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
