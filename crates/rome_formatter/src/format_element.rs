use rome_rowan::api::SyntaxTriviaPieceComments;
use rome_rowan::{Language, SyntaxToken, TextRange, TextSize};
use rslint_parser::{AstNode, SyntaxNode};

use crate::format_elements;
use crate::intersperse::{Intersperse, IntersperseFn};
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

/// A line break that only gets printed if the enclosing [Group] doesn't fit on a single line.
/// It's omitted if the enclosing [Group] fits on a single line.
/// A soft line break is identical to a hard line break when not enclosed inside of a [Group].
///
/// ## Examples
///
/// Soft line breaks are omitted if the enclosing [Group] fits on a single line
///
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break, FormatOptions};
///
/// let elements = group_elements(format_elements![
///   token("a,"),
///   soft_line_break(),
///   token("b"),
/// ]);
///
/// assert_eq!("a,b", format_element(&elements, FormatOptions::default()).as_code());
/// ```
/// See [soft_line_break_or_space] if you want to insert a space between the elements if the enclosing
/// [Group] fits on a single line.
///
/// Soft line breaks are emitted if the enclosing [Group] doesn't fit on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break, FormatOptions};
///
/// let elements = group_elements(format_elements![
///   token("a long word,"),
///   soft_line_break(),
///   token("so that the group doesn't fit on a single line"),
/// ]);
///
/// let options = FormatOptions {
///  line_width: 10,
///  ..FormatOptions::default()
/// };
///
/// assert_eq!("a long word,\nso that the group doesn't fit on a single line", format_element(&elements, options).as_code());
/// ```
#[inline]
pub const fn soft_line_break() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::Soft))
}

/// A forced line break that are always printed. A hard line break forces any enclosing [Group]
/// to be printed over multiple lines.
///
/// ## Examples
///
/// It forces a line break, even if the enclosing [Group] would otherwise fit on a single line.
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, FormatOptions, hard_line_break};
///
/// let elements = group_elements(format_elements![
///   token("a,"),
///   hard_line_break(),
///   token("b"),
///   hard_line_break()
/// ]);
///
/// assert_eq!("a,\nb\n", format_element(&elements, FormatOptions::default()).as_code());
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
/// use rome_formatter::{group_elements, format_element, format_elements, token, FormatOptions, empty_line};
///
/// let elements = group_elements(format_elements![
///   token("a,"),
///   empty_line(),
///   token("b"),
///   empty_line()
/// ]);
///
/// assert_eq!("a,\n\nb\n\n", format_element(&elements, FormatOptions::default()).as_code());
/// ```
#[inline]
pub const fn empty_line() -> FormatElement {
    FormatElement::Line(Line::new(LineMode::Empty))
}

/// A line break if the enclosing [Group] doesn't fit on a single line, a space otherwise.
///
/// ## Examples
///
/// The line breaks are emitted as spaces if the enclosing [Group] fits on a a single line:
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions};
///
/// let elements = group_elements(format_elements![
///   token("a,"),
///   soft_line_break_or_space(),
///   token("b"),
/// ]);
///
/// assert_eq!("a, b", format_element(&elements, FormatOptions::default()).as_code());
/// ```
///
/// The printer breaks the lines if the enclosing [Group] doesn't fit on a single line:
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions};
///
/// let elements = group_elements(format_elements![
///   token("a long word,"),
///   soft_line_break_or_space(),
///   token("so that the group doesn't fit on a single line"),
/// ]);
///
/// let options = FormatOptions {
///  line_width: 10,
///  ..FormatOptions::default()
/// };
///
/// assert_eq!("a long word,\nso that the group doesn't fit on a single line", format_element(&elements, options).as_code());
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
/// The [Printer] converts the line feed characters to the character specified in the [PrinterOptions].
///
/// ## Examples
///
/// ```
/// use rome_formatter::{token, format_element, FormatOptions};
/// let elements = token("Hello World");
///
/// assert_eq!("Hello World", format_element(&elements, FormatOptions::default()).as_code());
/// ```
///
/// Printing a string literal as a literal requires that the string literal is properly escaped and
/// enclosed in quotes (depending on the target language).
///
/// ```
/// use rome_formatter::{FormatOptions, token, format_element};
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = token("\"Hello\\tWorld\"");
///
/// assert_eq!(r#""Hello\tWorld""#, format_element(&elements, FormatOptions::default()).as_code());
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
/// use rome_formatter::{FormatOptions, token, format_element, line_suffix, format_elements};
///
/// let elements = format_elements![token("a"), line_suffix(token("c")), token("b")];
///
/// assert_eq!("abc", format_element(&elements, FormatOptions::default()).as_code());
/// ```
#[inline]
pub fn line_suffix(element: impl Into<FormatElement>) -> FormatElement {
    FormatElement::LineSuffix(Box::new(element.into()))
}

/// Inserts a single space. Allows to separate different tokens.
///
/// ## Examples
///
/// ```
/// use rome_formatter::{FormatOptions, token, format_element, space_token, format_elements};
///
/// // the tab must be encoded as \\t to not literally print a tab character ("Hello{tab}World" vs "Hello\tWorld")
/// let elements = format_elements![token("a"), space_token(), token("b")];
///
/// assert_eq!("a b", format_element(&elements, FormatOptions::default()).as_code());
/// ```
#[inline]
pub const fn space_token() -> FormatElement {
    FormatElement::Space
}

/// Concatenates the content of multiple [FormatToken]s.
///
/// ## Examples
///
/// ```rust
/// use rome_formatter::{concat_elements, FormatElement, space_token, token, format_element, FormatOptions};
/// let expr = concat_elements(vec![token("a"), space_token(), token("+"), space_token(), token("b")]);
///
/// assert_eq!("a + b", format_element(&expr, FormatOptions::default()).as_code())
/// ```
pub fn concat_elements<I>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = FormatElement>,
{
    let elements = elements.into_iter();

    let mut concatenated: Vec<FormatElement> = if let (_, Some(upper_bound)) = elements.size_hint()
    {
        Vec::with_capacity(upper_bound)
    } else {
        vec![]
    };

    for element in elements {
        match element {
            FormatElement::List(list) => concatenated.extend(list.content),
            FormatElement::Empty => (),
            _ => concatenated.push(element),
        }
    }

    if concatenated.is_empty() {
        empty_element()
    } else if concatenated.len() == 1 {
        concatenated.pop().unwrap()
    } else {
        FormatElement::from(List::new(concatenated))
    }
}

/// Concatenates a list of [FormatElement]s with spaces and line breaks to fit
/// them on as few lines as possible
///
/// ## Examples
///
/// ```rust
/// use std::str::from_utf8;
/// use rome_formatter::{fill_elements, FormatElement, space_token, token, format_element, FormatOptions};
/// let a = from_utf8(&[b'a'; 30]).unwrap();
/// let b = from_utf8(&[b'b'; 30]).unwrap();
/// let c = from_utf8(&[b'c'; 30]).unwrap();
/// let d = from_utf8(&[b'd'; 30]).unwrap();
/// let expr = fill_elements([token(a), token(b), token(c), token(d)]);
///
/// assert_eq!(format!("{a} {b}\n{c} {d}"), format_element(&expr, FormatOptions::default()).into_code())
/// ```
pub fn fill_elements(elements: impl IntoIterator<Item = FormatElement>) -> FormatElement {
    let mut list: Vec<_> = elements.into_iter().collect();
    match list.len() {
        0 => empty_element(),
        1 => list.pop().unwrap(),
        _ => FormatElement::Fill(List::new(list)),
    }
}

/// Joins the elements by placing a given separator between elements.
///
/// ## Examples
///
/// Joining different tokens by separating them with a comma and a space.
///
/// ```
/// use rome_formatter::{concat_elements, FormatOptions, join_elements, space_token, token, format_element};
///
/// let separator = concat_elements(vec![token(","), space_token()]);
/// let elements = join_elements(separator, vec![token("1"), token("2"), token("3"), token("4")]);
///
/// assert_eq!("1, 2, 3, 4", format_element(&elements, FormatOptions::default()).as_code());
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

/// Specialized version of [join_elements] for joining SyntaxNodes separated by a space, soft
/// line break or empty line depending on the input file.
///
/// This functions inspects the input source and separates consecutive elements with either
/// a [soft_line_break_or_space] or [empty_line] depending on how many line breaks were
/// separating the elements in the original file.
#[inline]
pub fn join_elements_soft_line<I, N>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
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
pub fn join_elements_hard_line<I, N>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
{
    join_elements_with(elements, hard_line_break)
}

#[inline]
pub fn join_elements_with<I, N>(elements: I, separator: fn() -> FormatElement) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
{
    /// Get the number of line breaks between two consecutive SyntaxNodes in the tree
    fn get_lines_between_nodes(prev_node: &SyntaxNode, next_node: &SyntaxNode) -> usize {
        // Ensure the two nodes are actually siblings on debug
        debug_assert_eq!(prev_node.next_sibling().as_ref(), Some(next_node));
        debug_assert_eq!(next_node.prev_sibling().as_ref(), Some(prev_node));

        // Count the lines separating the two statements,
        // starting with the trailing trivia of the previous node
        let mut line_count = prev_node
            .last_trailing_trivia()
            .and_then(|prev_token| {
                // Newline pieces can only come last in trailing trivias, skip to it directly
                prev_token.pieces().next_back()?.as_newline()
            })
            .is_some() as usize;

        // Then add the newlines in the leading trivia of the next node
        if let Some(leading_trivia) = next_node.first_leading_trivia() {
            for piece in leading_trivia.pieces() {
                if piece.as_newline().is_some() {
                    line_count += 1;
                } else if piece.as_comments().is_some() {
                    // Stop at the first comment piece, the comment printer
                    // will handle newlines between the comment and the node
                    break;
                }
            }
        }

        line_count
    }

    concat_elements(IntersperseFn::new(
        elements.into_iter(),
        |prev_node, next_node, next_elem| {
            if next_elem.is_empty() {
                empty_element()
            } else if get_lines_between_nodes(prev_node.syntax(), next_node.syntax()) > 1 {
                empty_line()
            } else {
                separator()
            }
        },
    ))
}

/// It adds a level of indentation to the given content
///
/// It doesn't add any line breaks at the edges of the content, meaning that
/// the line breaks have to be manually added.
///
/// This helper should be used only in rare cases, instead you should rely more on
/// [indent_block] and [soft_block_indent]
///
/// ## Examples
///
/// ```
/// use rome_formatter::{format_elements, format_element, FormatOptions, hard_line_break, block_indent, token, indent};
/// let block = (format_elements![
///   token("switch {"),
///   block_indent(format_elements![
///     token("default:"),
///     indent(format_elements![ // this is where we want to use a
///        hard_line_break(),
///        token("break;"),
///     ])
///   ]),
///   token("}"),
/// ]);
///
/// assert_eq!(
///   "switch {\n\tdefault:\n\t\tbreak;\n}",
///   format_element(&block, FormatOptions::default()).as_code()
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
/// use rome_formatter::{format_element, format_elements, token, FormatOptions, hard_line_break, block_indent};
///
/// let block = (format_elements![
///   token("{"),
///   block_indent(format_elements![
///     token("let a = 10;"),
///     hard_line_break(),
///     token("let c = a + 5;"),
///   ]),
///   token("}"),
/// ]);
///
/// assert_eq!(
///   "{\n\tlet a = 10;\n\tlet c = a + 5;\n}",
///   format_element(&block, FormatOptions::default()).as_code()
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
/// Indents the content by one level and puts in new lines if the enclosing [Group] doesn't fit on a single line
///
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("'First string',"),
///     soft_line_break_or_space(),
///     token("'second string',"),
///   ]),
///   token("]"),
/// ]);
///
/// let options = FormatOptions {
///  line_width: 10,
///  ..FormatOptions::default()
/// };
///
/// assert_eq!("[\n\t'First string',\n\t'second string',\n]", format_element(&elements, options).as_code());
/// ```
///
/// Doesn't change the formatting if the enclosing [Group] fits on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("5,"),
///     soft_line_break_or_space(),
///     token("10"),
///   ]),
///   token("]"),
/// ]);
///
/// assert_eq!(
///   "[5, 10]",
///   format_element(&elements, FormatOptions::default()).as_code()
/// );
/// ```
///
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

/// If the enclosing [Group] doesn't fit on a single line, inserts a line break and indent.
/// Otherwise, just inserts a space.
///
/// Line indents are used to break a single line of code, and therefore only insert a line
/// break before the content and not after the content.
///
/// ## Examples
///
/// Indents the content by one level and puts in new lines if the enclosing [Group] doesn't
/// fit on a single line. Otherwise, just inserts a space.
///
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_indent_or_space, FormatOptions, soft_block_indent, space_token};
///
/// let elements = group_elements(format_elements![
///   token("name"),
///   space_token(),
///   token("="),
///   soft_line_indent_or_space(format_elements![
///     token("firstName"),
///     space_token(),
///     token("+"),
///     space_token(),
///     token("lastName"),
///   ]),
/// ]);
///
/// let options = FormatOptions {
///  line_width: 10,
///  ..FormatOptions::default()
/// };
///
/// assert_eq!("name =\n\tfirstName + lastName", format_element(&elements, options).as_code());
/// ```
///
/// Only adds a space if the enclosing [Group] fits on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_indent_or_space, FormatOptions, soft_block_indent, space_token};
///
/// let elements = group_elements(format_elements![
///   token("a"),
///   space_token(),
///   token("="),
///   soft_line_indent_or_space(format_elements![
///      token("10")
///   ]),
/// ]);
///
/// assert_eq!(
///   "a = 10",
///   format_element(&elements, FormatOptions::default()).as_code()
/// );
/// ```
///
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

/// Creates a logical [Group] around the content that should either consistently be printed on a single line
/// or broken across multiple lines.
///
/// The printer will try to print the content of the [Group] on a single line, ignoring all soft line breaks and
/// emitting spaces for soft line breaks or spaces. The printer tracks back if it isn't successful either
/// because it encountered a hard line break, or because printing the [Group] on a single line exceeds
/// the configured line width, and thus it must print all its content on multiple lines,
/// emitting line breaks for all line break kinds.
///
/// ## Examples
///
/// [Group] that fits on a single line
///
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("1,"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///   ]),
///   token("]"),
/// ]);
///
/// assert_eq!("[1, 2, 3]", format_element(&elements, FormatOptions::default()).as_code());
/// ```
///
/// The printer breaks the [Group] over multiple lines if its content doesn't fit on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("'Good morning! How are you today?',"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///   ]),
///   token("]"),
/// ]);
///
/// let options = FormatOptions {
///   line_width: 20,
///   ..FormatOptions::default()
/// };
///
/// assert_eq!("[\n\t'Good morning! How are you today?',\n\t2,\n\t3\n]", format_element(&elements, options).as_code());
/// ```
#[inline]
pub fn group_elements<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        FormatElement::from(Group::new(content))
    }
}

/// Adds a conditional content that is emitted only if it isn't inside an enclosing [Group] that
/// is printed on a single line. The element allows, for example, to insert a trailing comma after the last
/// array element only if the array doesn't fit on a single line.
///
/// The element has no special meaning if used outside of a [Group]. In that case, the content is always emitted.
///
/// If you're looking for a way to only print something if the [Group] fits on a single line see [if_group_fits_on_single_line].
///
/// ## Examples
///
/// Omits the trailing comma for the last array element if the [Group] fits on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent, if_group_breaks};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("1,"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///     if_group_breaks(token(","))
///   ]),
///   token("]"),
/// ]);
/// assert_eq!("[1, 2, 3]", format_element(&elements, FormatOptions::default()).as_code());
/// ```
///
/// Prints the trailing comma for the last array element if the [Group] doesn't fit on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent, if_group_breaks};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("'A somewhat longer string to force a line break',"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///     if_group_breaks(token(","))
///   ]),
///   token("]"),
/// ]);
///
/// let options = FormatOptions { line_width: 20, ..FormatOptions::default() };
/// assert_eq!(
///   "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3,\n]",
///   format_element(&elements, options).as_code()
/// );
/// ```
#[inline]
pub fn if_group_breaks<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        FormatElement::from(ConditionalGroupContent::new(
            content,
            GroupPrintMode::Multiline,
        ))
    }
}

/// Adds a conditional content specific for [Group]s that fit on a single line. The content isn't
/// emitted for [Group]s spanning multiple lines.
///
/// See [if_group_breaks] if you're looking for a way to print content only for groups spanning multiple lines.
///
/// ## Examples
///
/// Adds the trailing comma for the last array element if the [Group] fits on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent, if_group_fits_on_single_line};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("1,"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///     if_group_fits_on_single_line(token(","))
///   ]),
///   token("]"),
/// ]);
/// assert_eq!("[1, 2, 3,]", format_element(&elements, FormatOptions::default()).as_code());
/// ```
///
/// Omits the trailing comma for the last array element if the [Group] doesn't fit on a single line
/// ```
/// use rome_formatter::{group_elements, format_element, format_elements, token, soft_line_break_or_space, FormatOptions, soft_block_indent, if_group_fits_on_single_line};
///
/// let elements = group_elements(format_elements![
///   token("["),
///   soft_block_indent(format_elements![
///     token("'A somewhat longer string to force a line break',"),
///     soft_line_break_or_space(),
///     token("2,"),
///     soft_line_break_or_space(),
///     token("3"),
///     if_group_fits_on_single_line(token(","))
///   ]),
///   token("]"),
/// ]);
///
/// let options = FormatOptions { line_width: 20, ..FormatOptions::default() };
/// assert_eq!(
///   "[\n\t'A somewhat longer string to force a line break',\n\t2,\n\t3\n]",
///   format_element(&elements, options).as_code()
/// );
/// ```
#[inline]
pub fn if_group_fits_on_single_line<TFlat>(flat_content: TFlat) -> FormatElement
where
    TFlat: Into<FormatElement>,
{
    let flat_content = flat_content.into();

    if flat_content.is_empty() {
        flat_content
    } else {
        FormatElement::from(ConditionalGroupContent::new(
            flat_content,
            GroupPrintMode::Flat,
        ))
    }
}

/// Language agnostic IR for formatting source code.
///
/// Use the helper functions like [space], [soft_line_break] etc. defined in this file to create elements.
#[derive(Clone, Eq, PartialEq)]
pub enum FormatElement {
    Empty,

    /// A space token, see [space] for documentation.
    Space,

    /// A new line, see [soft_line_break], [hard_line_break], and [soft_line_break_or_space] for documentation.
    Line(Line),

    /// Indents the content one level deeper, see [indent] for documentation and examples.
    Indent(Indent),

    /// Creates a logical group where its content is either consistently printed:
    /// * on a single line: Omitting [LineMode::Soft] line breaks and printing spaces for [LineMode::SoftOrSpace]
    /// * on multiple lines: Printing all line breaks
    ///
    /// See [group] for documentation and examples.
    Group(Group),

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [if_group_breaks] for examples.
    ConditionalGroupContent(ConditionalGroupContent),

    /// Concatenates multiple elements together. See [concat_elements] and [join_elements] for examples.
    List(List),

    /// Concatenates multiple elements together with spaces or line breaks to fill the print width. See [fill_elements].
    Fill(List),

    /// A token that should be printed as is, see [token] for documentation and examples.
    Token(Token),

    LineSuffix(Content),
}

impl Debug for FormatElement {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            FormatElement::Empty => write!(fmt, "Empty"),
            FormatElement::Space => write!(fmt, "Space"),
            FormatElement::Line(content) => content.fmt(fmt),
            FormatElement::Indent(content) => content.fmt(fmt),
            FormatElement::Group(content) => content.fmt(fmt),
            FormatElement::ConditionalGroupContent(content) => content.fmt(fmt),
            FormatElement::List(content) => {
                write!(fmt, "List ")?;
                content.fmt(fmt)
            }
            FormatElement::Token(content) => content.fmt(fmt),
            FormatElement::LineSuffix(content) => {
                fmt.debug_tuple("LineSuffix").field(content).finish()
            }
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
    fn new(content: Vec<FormatElement>) -> Self {
        Self { content }
    }

    fn trim_start(&self) -> Self {
        let mut content: Vec<_> = self
            .iter()
            .skip_while(|e| match e {
                FormatElement::Empty => true,
                FormatElement::Space => true,
                FormatElement::Line(_) => true,
                FormatElement::Indent(_) => true,
                FormatElement::Token(t) => {
                    let s = t.trim_start();
                    s.is_empty()
                }
                _ => false,
            })
            .map(Clone::clone)
            .collect();

        if let Some(FormatElement::Token(s)) = content.get_mut(0) {
            *s = s.trim_start();
        }

        Self::new(content)
    }

    fn trim_end(&self) -> Self {
        let idx_first_non_empty = self.iter().rev().position(|e| match e {
            FormatElement::Empty => false,
            FormatElement::Space => false,
            FormatElement::Line(_) => false,
            FormatElement::Indent(_) => false,
            FormatElement::Token(t) => {
                let s = t.trim_end();
                !s.is_empty()
            }
            _ => true,
        });

        match idx_first_non_empty {
            Some(idx_first_non_empty) => {
                let idx_first_non_empty = self.len() - idx_first_non_empty;
                let mut content: Vec<_> = self
                    .iter()
                    .take(idx_first_non_empty)
                    .map(Clone::clone)
                    .collect();

                if let Some(FormatElement::Token(s)) = content.last_mut() {
                    *s = s.trim_end();
                }

                Self::new(content)
            }
            None => Self::new(vec![]),
        }
    }
}

impl Deref for List {
    type Target = Vec<FormatElement>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

/// Group is a special token that controls how the child tokens are printed.
///
/// The printer first tries to print all tokens in the group onto a single line (ignoring soft line wraps)
/// but breaks the array cross multiple lines if it would exceed the specified `line_width`, if a child token is a hard line break or if a string contains a line break.
#[derive(Clone, PartialEq, Eq)]
pub struct Group {
    pub(crate) content: Content,
}

impl Debug for Group {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_tuple("Group").field(&self.content).finish()
    }
}

impl Group {
    pub fn new(content: FormatElement) -> Self {
        Self {
            content: Box::new(content),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GroupPrintMode {
    Flat,
    Multiline,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConditionalGroupContent {
    pub(crate) content: Content,

    /// In what mode the content should be printed.
    /// * Flat -> Omitted if the enclosing group is a multiline group, printed for groups fitting on a single line
    /// * Multiline -> Omitted if the enclosing group fits on a single line, printed if the group breaks over multiple lines.
    pub(crate) mode: GroupPrintMode,
}

impl ConditionalGroupContent {
    pub fn new(content: FormatElement, mode: GroupPrintMode) -> Self {
        Self {
            content: Box::new(content),
            mode,
        }
    }
}

/// See [token] for documentation
#[derive(Eq, Clone)]
pub enum Token {
    /// Token constructed by the formatter from a static string
    Static { text: &'static str },
    /// Token constructed from the input source as a dynamics
    /// string and a range of the input source
    Dynamic { text: String, source: TextRange },
}

impl Debug for Token {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // This does not use debug_tuple so the tokens are
        // written on a single line even when pretty-printing
        match self {
            Token::Static { text } => write!(fmt, "Token({:?})", text),
            Token::Dynamic { text, source } => write!(fmt, "Token({:?}, {:?})", text, source),
        }
    }
}

impl Token {
    /// Create a token from a static string
    const fn new_static(text: &'static str) -> Self {
        Self::Static { text }
    }

    /// Create a token from a dynamic string and a range of the input source
    pub(crate) fn new_dynamic(text: String, source: TextRange) -> Self {
        debug_assert!(!text.contains('\r'), "The content '{}' contains an unsupported '\\r' line terminator character but string tokens must only use line feeds '\\n' as line separator. Use '\\n' instead of '\\r' and '\\r\\n' to insert a line break in strings.", text);
        Self::Dynamic { text, source }
    }

    /// Get the range of the input source covered by this token,
    /// or None if the token was synthesized by the formatter
    pub(crate) fn source(&self) -> Option<&TextRange> {
        match self {
            Token::Static { .. } => None,
            Token::Dynamic { source, .. } => Some(source),
        }
    }

    fn trim_start(&self) -> Self {
        match self {
            Token::Static { text } => Self::Static {
                text: text.trim_start(),
            },
            Token::Dynamic { text, source } => {
                let prev_len = TextSize::from(text.len() as u32);
                let text = text.trim_start();
                let next_len = TextSize::from(text.len() as u32);
                let diff = prev_len - next_len;
                Self::Dynamic {
                    text: text.into(),
                    source: TextRange::new(source.start() + diff, source.end()),
                }
            }
        }
    }

    fn trim_end(&self) -> Self {
        match self {
            Token::Static { text } => Self::Static {
                text: text.trim_end(),
            },
            Token::Dynamic { text, source } => {
                let prev_len = TextSize::from(text.len() as u32);
                let text = text.trim_end();
                let next_len = TextSize::from(text.len() as u32);
                let diff = prev_len - next_len;
                Self::Dynamic {
                    text: text.into(),
                    source: TextRange::new(source.start(), source.end() - diff),
                }
            }
        }
    }
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
        Self::new_dynamic(token.text_trimmed().into(), token.text_trimmed_range())
    }
}

const LINE_SEPARATOR: char = '\u{2028}';
const PARAGRAPH_SEPARATOR: char = '\u{2029}';
pub(crate) const LINE_TERMINATORS: [char; 3] = ['\r', LINE_SEPARATOR, PARAGRAPH_SEPARATOR];

/// Replace the line terminators matching the provided list with "\n"
/// since its the only line break type supported by the printer
pub(crate) fn normalize_newlines<const N: usize>(text: &str, terminators: [char; N]) -> Cow<str> {
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
        Self::new_dynamic(
            normalize_newlines(trivia.text().trim(), LINE_TERMINATORS).into_owned(),
            trivia.text_range(),
        )
    }
}

impl Deref for Token {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Token::Static { text } => text,
            Token::Dynamic { text, .. } => text,
        }
    }
}

impl FormatElement {
    /// Returns true if the element contains no content.
    pub fn is_empty(&self) -> bool {
        self == &FormatElement::Empty
    }

    /// Remove all spaces, line breaks, indents from the start of
    /// the [FormatElement].
    /// Including "whitespace" characters of the [FormatElement::Token] variant.
    pub fn trim_start(&self) -> FormatElement {
        match self {
            FormatElement::Empty => FormatElement::Empty,
            FormatElement::Space => FormatElement::Empty,
            FormatElement::Line(_) => FormatElement::Empty,
            FormatElement::Indent(i) => i.content.trim_start(),
            FormatElement::Group(g) => g.content.trim_start(),
            FormatElement::Fill(list) => FormatElement::Fill(list.trim_start()),
            FormatElement::ConditionalGroupContent(g) => g.content.trim_start(),
            FormatElement::List(list) => FormatElement::List(list.trim_start()),
            FormatElement::Token(s) => FormatElement::Token(s.trim_start()),
            FormatElement::LineSuffix(s) => FormatElement::LineSuffix(Box::new(s.trim_start())),
        }
    }

    /// Remove all spaces, line breaks, indents from the end of
    /// the [FormatElement].
    /// Including "whitespace" characters of the [FormatElement::Token] variant.
    pub fn trim_end(&self) -> FormatElement {
        match self {
            FormatElement::Empty => FormatElement::Empty,
            FormatElement::Space => FormatElement::Empty,
            FormatElement::Line(_) => FormatElement::Empty,
            FormatElement::Indent(i) => i.content.trim_end(),
            FormatElement::Group(g) => g.content.trim_end(),
            FormatElement::ConditionalGroupContent(g) => g.content.trim_end(),
            FormatElement::Fill(list) => FormatElement::Fill(list.trim_end()),
            FormatElement::List(list) => FormatElement::List(list.trim_end()),
            FormatElement::Token(s) => FormatElement::Token(s.trim_end()),
            FormatElement::LineSuffix(s) => FormatElement::LineSuffix(Box::new(s.trim_end())),
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

    use crate::format_element::{empty_element, join_elements, List};
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
    fn format_element_trim() {
        use crate::format_element::*;
        let f = concat_elements([
            FormatElement::Empty,
            FormatElement::Indent(Indent::new(FormatElement::Empty)),
            FormatElement::Line(Line::new(LineMode::Hard)),
            FormatElement::Space,
            FormatElement::Token(Token::new_static(" \t \n")),
            FormatElement::List(List::new(vec![FormatElement::Empty])),
            FormatElement::Group(Group::new(FormatElement::Empty)),
            FormatElement::ConditionalGroupContent(ConditionalGroupContent::new(
                FormatElement::Empty,
                GroupPrintMode::Flat,
            )),
        ]);

        let f = f.trim_start();
        matches!(f.trim_start(), FormatElement::Empty);
        matches!(f.trim_end(), FormatElement::Empty);
    }
}
