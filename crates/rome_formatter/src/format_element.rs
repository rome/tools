use rome_js_syntax::{AstNode, SyntaxNode};
use rome_rowan::api::SyntaxTriviaPieceComments;
use rome_rowan::{Language, SyntaxToken, TextRange};

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

/// A line break that only gets printed if the enclosing `Group` doesn't fit on a single line.
/// It's omitted if the enclosing `Group` fits on a single line.
/// A soft line break is identical to a hard line break when not enclosed inside of a `Group`.
///
/// ## Examples
///
/// Soft line breaks are omitted if the enclosing `Group` fits on a single line
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
/// `Group` fits on a single line.
///
/// Soft line breaks are emitted if the enclosing `Group` doesn't fit on a single line
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

/// A forced line break that are always printed. A hard line break forces any enclosing `Group`
/// to be printed over multiple lines.
///
/// ## Examples
///
/// It forces a line break, even if the enclosing `Group` would otherwise fit on a single line.
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

/// A line break if the enclosing `Group` doesn't fit on a single line, a space otherwise.
///
/// ## Examples
///
/// The line breaks are emitted as spaces if the enclosing `Group` fits on a a single line:
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
/// The printer breaks the lines if the enclosing `Group` doesn't fit on a single line:
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
/// The [crate::Printer] converts the line feed characters to the character specified in the [crate::PrinterOptions].
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

/// Mark a [FormatElement] as being a piece of trivia
///
/// This does not directly influence how this content will be printed, but some
/// parts of the formatter may chose to handle this element in a specific way
///
/// ## Examples
///
/// ```
/// use rome_formatter::{FormatOptions, token, format_element, comment, format_elements, group_elements, empty_line, soft_line_break_or_space};
///
/// let elements = group_elements(format_elements![comment(empty_line()), token("a"), soft_line_break_or_space(), token("b")]);
///
/// assert_eq!("\na b", format_element(&elements, FormatOptions::default()).as_code());
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

/// Concatenates the content of multiple [FormatElement]s.
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
    let mut elements = elements.into_iter();

    let (lower_bound, upper_bound) = elements.size_hint();
    let size_hint = upper_bound.unwrap_or(lower_bound);

    // If the first non empty element is a vec, use it,
    // otherwise create a new one with the current element
    let mut concatenated = loop {
        match elements.next() {
            Some(FormatElement::Empty) => continue,
            Some(FormatElement::List(list)) => {
                let mut v = list.content;
                v.reserve(size_hint);
                break v;
            }
            Some(element) => {
                let mut v = Vec::with_capacity(size_hint);
                v.push(element);
                break v;
            }
            None => return empty_element(),
        }
    };

    // continue to the rest of the list
    for element in elements {
        match element {
            FormatElement::List(list) => concatenated.extend(list.content),
            FormatElement::Empty => {}
            element => concatenated.push(element),
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
                if piece.is_newline() {
                    line_count += 1;
                } else if piece.is_comments() {
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
/// [block_indent] and [soft_block_indent]
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
/// Indents the content by one level and puts in new lines if the enclosing `Group` doesn't fit on a single line
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
/// Doesn't change the formatting if the enclosing `Group` fits on a single line
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
/// Only adds a space if the enclosing `Group` fits on a single line
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
/// The printer breaks the `Group` over multiple lines if its content doesn't fit on a single line
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
    let content: FormatElement = content.into();
    let (leading, content, trailing) = content.split_trivia();
    format_elements![leading, Group::new(content), trailing]
}

/// Creates a group that forces all elements inside it to be printed on a
/// single line. This behavior can in turn be escaped by introducing an inner
/// `Group` element that will resume the normal breaking behavior of the printer.
///
/// This is useful for constructs that have a non-breaking head and a breaking
/// body, such class declarations:
/// ```js
///    abstract /* comment */ class Example
/// // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ non-breaking part
/// { // <
/// } // < breaking part
/// ```
///
/// # Example
/// ```
/// use rome_formatter::{
///   group_elements, format_element, format_elements, token, hard_group_elements,
///   FormatOptions, empty_line, if_group_breaks, if_group_fits_on_single_line
/// };
///
/// let elements = group_elements(hard_group_elements(format_elements![
///   if_group_breaks(token("not printed")),
///   empty_line(),
///   if_group_fits_on_single_line(token("printed")),
/// ]));
///
/// assert_eq!("\nprinted", format_element(&elements, FormatOptions::default()).as_code());
/// ```
#[inline]
pub fn hard_group_elements<T: Into<FormatElement>>(content: T) -> FormatElement {
    let content = content.into();

    if content.is_empty() {
        content
    } else {
        FormatElement::HardGroup(Group::new(content))
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
/// Prints the trailing comma for the last array element if the `Group` doesn't fit on a single line
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

/// Adds a conditional content specific for `Group`s that fit on a single line. The content isn't
/// emitted for `Group`s spanning multiple lines.
///
/// See [if_group_breaks] if you're looking for a way to print content only for groups spanning multiple lines.
///
/// ## Examples
///
/// Adds the trailing comma for the last array element if the `Group` fits on a single line
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
/// Omits the trailing comma for the last array element if the `Group` doesn't fit on a single line
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

    /// See [crate::hard_group_elements] for documentation and examples.
    HardGroup(Group),

    /// Allows to specify content that gets printed depending on whatever the enclosing group
    /// is printed on a single line or multiple lines. See [crate::if_group_breaks] for examples.
    ConditionalGroupContent(ConditionalGroupContent),

    /// Concatenates multiple elements together. See [concat_elements] and [join_elements] for examples.
    List(List),

    /// Concatenates multiple elements together with spaces or line breaks to fill the print width. See [fill_elements].
    Fill(List),

    /// A token that should be printed as is, see [token] for documentation and examples.
    Token(Token),

    /// Delay the printing of its content until the next line break
    LineSuffix(Content),

    /// Special semantic element letting the printer and formatter know this is
    /// a trivia content, and it should only have a limited influence on the
    /// formatting (for instance line breaks contained within will not cause
    /// the parent group to break if this element is at the start of it)
    Comment(Content),

    /// A token that tracks tokens/nodes that are printed using [`format_verbatim`](crate::Formatter::format_verbatim) API
    Verbatim(Verbatim),
}

#[derive(Clone, Eq, PartialEq)]
pub enum VerbatimKind {
    Unknown,
    Suppressed,
    Verbatim {
        /// the range that belongs to the node/token formatted verbatim
        range: TextRange,
        /// the text that belongs to the node/token formatted verbatim
        text: String,
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
    pub fn new_verbatim(element: FormatElement, text: String, range: TextRange) -> Self {
        Self {
            element: Box::new(element),
            kind: VerbatimKind::Verbatim { range, text },
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

    pub(crate) fn is_unknown(&self) -> bool {
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
            FormatElement::HardGroup(content) => {
                write!(fmt, "HardGroup")?;
                content.fmt(fmt)
            }
            FormatElement::ConditionalGroupContent(content) => content.fmt(fmt),
            FormatElement::List(content) => {
                write!(fmt, "List ")?;
                content.fmt(fmt)
            }
            FormatElement::Fill(content) => {
                write!(fmt, "Fill ")?;
                content.fmt(fmt)
            }
            FormatElement::Token(content) => content.fmt(fmt),
            FormatElement::LineSuffix(content) => {
                fmt.debug_tuple("LineSuffix").field(content).finish()
            }
            FormatElement::Comment(content) => fmt.debug_tuple("Comment").field(content).finish(),
            FormatElement::Verbatim(verbatim) => fmt
                .debug_tuple("Verbatim")
                .field(&verbatim.element)
                .finish(),
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
        fmt.debug_tuple("").field(&self.content).finish()
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

    /// Returns true if this [FormatElement] recursively contains any hard line break
    /// ([hard_line_break], [empty_line], [Token] containing the '\n' character)
    pub fn has_hard_line_breaks(&self) -> bool {
        match self {
            FormatElement::Empty => false,
            FormatElement::Space => false,
            FormatElement::Line(line) => matches!(line.mode, LineMode::Hard | LineMode::Empty),
            FormatElement::Indent(indent) => indent.content.has_hard_line_breaks(),
            FormatElement::Group(group) | FormatElement::HardGroup(group) => {
                group.content.has_hard_line_breaks()
            }
            FormatElement::ConditionalGroupContent(group) => group.content.has_hard_line_breaks(),
            FormatElement::List(list) | FormatElement::Fill(list) => {
                list.content.iter().any(FormatElement::has_hard_line_breaks)
            }
            FormatElement::Token(token) => token.contains('\n'),
            FormatElement::LineSuffix(_) => true,
            FormatElement::Comment(content) => content.has_hard_line_breaks(),
            FormatElement::Verbatim(verbatim) => verbatim.element.has_hard_line_breaks(),
        }
    }

    /// Splits off the leading and trailing trivias (comments) from this [FormatElement]
    ///
    /// For [FormatElement::HardGroup] and [FormatElement::Group], the trailing and leading trivias
    /// are automatically moved  outside of the group. The group itself is then recreated around the
    /// content itself.
    pub fn split_trivia(self) -> (FormatElement, FormatElement, FormatElement) {
        match self {
            FormatElement::List(list) => {
                // Find the index of the first non-comment element in the list
                let content_start = list
                    .content
                    .iter()
                    .position(|elem| !matches!(elem, FormatElement::Comment(_)))
                    .unwrap_or(list.content.len());

                // Split the list at the found index
                let mut leading = list.content;
                let mut content = leading.split_off(content_start);

                // Find the index of the last non-comment element in the list
                let content_end = content
                    .iter()
                    .rposition(|elem| !matches!(elem, FormatElement::Comment(_)))
                    .map_or(0, |index| index + 1);

                // Split the list at the found index, content now holds the inner elements
                let trailing = content.split_off(content_end);

                (
                    concat_elements(leading),
                    concat_elements(content),
                    concat_elements(trailing),
                )
            }
            FormatElement::HardGroup(group) => {
                let (leading, content, trailing) = group.content.split_trivia();
                // re-create the grouping around the content only
                (leading, hard_group_elements(content), trailing)
            }

            FormatElement::Group(group) => {
                let (leading, content, trailing) = group.content.split_trivia();
                // re-create the grouping around the content only
                (leading, group_elements(content), trailing)
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
            FormatElement::List(list) | FormatElement::Fill(list) => {
                list.iter().rev().find_map(|element| element.last_element())
            }

            FormatElement::Empty | FormatElement::Line(_) | FormatElement::Comment(_) => None,

            FormatElement::Indent(indent) => indent.content.last_element(),
            FormatElement::Group(group) | FormatElement::HardGroup(group) => {
                group.content.last_element()
            }

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
