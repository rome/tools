use crate::context::QuoteStyle;
use crate::prelude::*;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsComputedMemberExpression, JsStaticMemberExpression,
    JsSyntaxKind, JsxAnyChild, JsxExpressionChild, JsxTagExpression, TextLen,
};
use rome_rowan::{SyntaxResult, SyntaxTokenText, TextRange, TextSize};
use std::iter::{FusedIterator, Peekable};
use std::str::Chars;

pub(crate) static JSX_WHITESPACE_CHARS: [char; 4] = [' ', '\n', '\t', '\r'];

/// Meaningful JSX text is defined to be text that has either non-whitespace
/// characters, or does not contain a newline. Whitespace is defined as ASCII
/// whitespace.
///
/// ```
/// use rome_js_formatter::utils::jsx::is_meaningful_jsx_text;
///
/// assert_eq!(is_meaningful_jsx_text("     \t\r   "), true);
/// assert_eq!(is_meaningful_jsx_text("     \n\r   "), false);
/// assert_eq!(is_meaningful_jsx_text("  Alien   "), true);
/// assert_eq!(is_meaningful_jsx_text("\n  Alien   "), true);
/// assert_eq!(is_meaningful_jsx_text("  Alien   \n"), true);
/// assert_eq!(is_meaningful_jsx_text(""), true);
/// ```
pub fn is_meaningful_jsx_text(text: &str) -> bool {
    let mut has_newline = false;
    for c in text.chars() {
        // If there is a non-whitespace character
        if !JSX_WHITESPACE_CHARS.contains(&c) {
            return true;
        } else if c == '\n' {
            has_newline = true;
        }
    }

    !has_newline
}

/// Indicates that an element should always be wrapped in parentheses, should be wrapped
/// only when it's line broken, or should not be wrapped at all.
#[derive(Copy, Clone, Debug)]
pub(crate) enum WrapState {
    /// For a JSX element that is never wrapped in parentheses.
    /// For instance, a JSX element that is another element's attribute
    /// should never be wrapped:
    /// ```jsx
    ///  <Route path="/" component={<HomePage />} />
    /// ```
    NoWrap,
    /// For a JSX element that must be wrapped in parentheses when line broken.
    /// For instance, a JSX element nested in a let binding is wrapped on line break:
    /// ```jsx
    ///  let component = <div> La Haine dir. Mathieu Kassovitz </div>;
    ///
    ///  let component = (
    ///   <div> Uncle Boonmee Who Can Recall His Past Lives dir. Apichatpong Weerasethakul </div>
    ///  );
    /// ```
    WrapOnBreak,
}

/// Checks if a JSX Element should be wrapped in parentheses. Returns a [WrapState] which
/// indicates when the element should be wrapped in parentheses.
pub(crate) fn get_wrap_state(node: &JsxTagExpression) -> WrapState {
    // We skip the first item because the first item in ancestors is the node itself, i.e.
    // the JSX Element in this case.
    let parent = node.syntax().parent();

    parent.map_or(WrapState::NoWrap, |parent| match parent.kind() {
        JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
        | JsSyntaxKind::JSX_ATTRIBUTE
        | JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE
        | JsSyntaxKind::JSX_EXPRESSION_CHILD
        | JsSyntaxKind::JS_EXPRESSION_STATEMENT
        | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        | JsSyntaxKind::JS_EXPRESSION_SNIPPED
        | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => WrapState::NoWrap,
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
            let member = JsStaticMemberExpression::unwrap_cast(parent);

            if member.is_optional_chain() {
                WrapState::NoWrap
            } else {
                WrapState::WrapOnBreak
            }
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let member = JsComputedMemberExpression::unwrap_cast(parent);

            if member.is_optional_chain() {
                WrapState::NoWrap
            } else {
                WrapState::WrapOnBreak
            }
        }
        _ => WrapState::WrapOnBreak,
    })
}

/// Creates either a space using an expression child and a string literal,
/// or a regular space, depending on whether the group breaks or not.
///
/// ```jsx
///  <div> Winter Light </div>;
///
///  <div>
///    {" "}Winter Light
///    Through A Glass Darkly
///    The Silence
///    Seventh Seal
///    Wild Strawberries
///  </div>
/// ```
#[derive(Default)]
pub(crate) struct JsxSpace;

impl Format<JsFormatContext> for JsxSpace {
    fn fmt(&self, formatter: &mut JsFormatter) -> FormatResult<()> {
        write![
            formatter,
            [
                if_group_breaks(&format_args![JsxRawSpace, hard_line_break()]),
                if_group_fits_on_line(&space())
            ]
        ]
    }
}

pub(crate) struct JsxRawSpace;

impl Format<JsFormatContext> for JsxRawSpace {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let jsx_space = match f.options().quote_style() {
            QuoteStyle::Double => r#"{" "}"#,
            QuoteStyle::Single => "{' '}",
        };

        write!(f, [text(jsx_space)])
    }
}

pub(crate) fn is_whitespace_jsx_expression(child: &JsxExpressionChild) -> bool {
    match child.expression() {
        Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(literal),
        )) => {
            match (
                child.l_curly_token(),
                literal.value_token(),
                child.r_curly_token(),
            ) {
                (Ok(l_curly_token), Ok(value_token), Ok(r_curly_token)) => {
                    let is_empty = matches!(value_token.text_trimmed(), "\" \"" | "' '");

                    let has_comments = l_curly_token.has_trailing_comments()
                        || r_curly_token.has_leading_comments()
                        || value_token.has_leading_non_whitespace_trivia()
                        || value_token.has_trailing_comments();

                    is_empty && !has_comments
                }
                _ => false,
            }
        }
        _ => false,
    }
}

pub(crate) fn jsx_split_children<I>(children: I) -> SyntaxResult<Vec<JsxChild>>
where
    I: IntoIterator<Item = JsxAnyChild>,
{
    let mut result = Vec::new();

    for child in children.into_iter() {
        match child {
            JsxAnyChild::JsxText(text) => {
                // Split the text into words
                // Keep track if there's any leading/trailing empty line, new line or whitespace

                let value_token = text.value_token()?;
                let mut chunks = JsxSplitChunksIterator::new(value_token.text()).peekable();

                // Text starting with a whitespace
                if let Some((_, JsxTextChunk::Whitespace(_whitespace))) = chunks.peek() {
                    match chunks.next() {
                        Some((_, JsxTextChunk::Whitespace(whitespace))) => {
                            if whitespace.contains('\n') {
                                if chunks.peek().is_none() {
                                    // A text only consisting of whitespace that also contains a new line isn't considered meaningful text.
                                    // It can be entirely removed from the content without changing the semantics.
                                    let newlines =
                                        whitespace.chars().filter(|c| *c == '\n').count();

                                    // Keep up to one blank line between tags/expressions and text.
                                    // ```javascript
                                    // <div>
                                    //
                                    //   <MyElement />
                                    // </div>
                                    // ```
                                    if newlines > 1 {
                                        result.push(JsxChild::EmptyLine);
                                    }

                                    continue;
                                }

                                result.push(JsxChild::Newline)
                            } else if !matches!(result.last(), Some(JsxChild::Whitespace)) {
                                result.push(JsxChild::Whitespace)
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                while let Some(chunk) = chunks.next() {
                    match chunk {
                        (_, JsxTextChunk::Whitespace(whitespace)) => {
                            // Only handle trailing whitespace. Words must always be joined by new lines
                            if chunks.peek().is_none() {
                                if whitespace.contains('\n') {
                                    result.push(JsxChild::Newline);
                                } else {
                                    result.push(JsxChild::Whitespace)
                                }
                            }
                        }

                        (relative_start, JsxTextChunk::Word(word)) => {
                            result.push(JsxChild::Word(JsxWord {
                                text: value_token
                                    .token_text()
                                    .slice(TextRange::at(relative_start, word.text_len())),
                                source_position: value_token.text_range().start() + relative_start,
                            }));
                        }
                    }
                }
            }

            JsxAnyChild::JsxExpressionChild(child) => {
                if is_whitespace_jsx_expression(&child) {
                    match result.last() {
                        Some(JsxChild::Whitespace) => {
                            // Ignore
                        }
                        _ => result.push(JsxChild::Whitespace),
                    }
                } else {
                    result.push(JsxChild::NonText(child.into()))
                }
            }
            child => {
                result.push(JsxChild::NonText(child));
            }
        }
    }

    Ok(result)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum JsxChild {
    /// A Single word in a JSX text. For example, the words for `a b\nc` are `[a, b, c]`
    Word(JsxWord),

    /// A ` ` or `${" "}` whitespace
    ///
    /// ```javascript
    /// <div> </div>
    /// <div>a </div>
    /// <div> a</div>
    /// <div>{' '}a</div>
    /// <div>a{' '}</div>
    /// <div>{' '}</div>
    /// <div>a
    /// {' '}b</div>
    /// ```
    ///
    /// Whitespace between two words is not represented as whitespace
    /// ```javascript
    /// <div>a b</div>
    /// ```
    /// The space between `a` and `b` is not considered a whitespace.
    Whitespace,

    /// A new line at the start or end of a [JsxText] with meaningful content. (that isn't all whitespace
    /// and contains a new line).
    ///
    /// ```javascript
    /// <div>
    ///     a
    /// </div>
    /// ```
    Newline,

    /// A [JsxText] that only consists of whitespace and has at least two line breaks;
    ///
    /// ```javascript
    /// <div>
    ///
    ///   <test />
    /// </div>
    /// ```
    ///
    /// The text between `<div>` and `<test />` is an empty line text.
    EmptyLine,

    /// Any other content that isn't a text. Should be formatted as is.
    NonText(JsxAnyChild),
}

impl JsxChild {
    pub(crate) const fn is_any_line(&self) -> bool {
        matches!(self, JsxChild::EmptyLine | JsxChild::Newline)
    }
}

/// A word in a Jsx Text. A word is string sequence that isn't separated by any JSX whitespace.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct JsxWord {
    text: SyntaxTokenText,
    source_position: TextSize,
}

impl Format<JsFormatContext> for JsxWord {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        f.write_element(FormatElement::Text(Text::SyntaxTokenTextSlice {
            source_position: self.source_position,
            slice: self.text.clone(),
        }))
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum JsxTextChunk<'a> {
    Whitespace(&'a str),
    Word(&'a str),
}

/// Splits a text into whitespace only and non-whitespace chunks.
///
/// See `jsx_split_chunks_iterator` test for examples
struct JsxSplitChunksIterator<'a> {
    position: TextSize,
    text: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> JsxSplitChunksIterator<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            position: TextSize::default(),
            text,
            chars: text.chars().peekable(),
        }
    }
}

impl<'a> Iterator for JsxSplitChunksIterator<'a> {
    type Item = (TextSize, JsxTextChunk<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars.next()?;

        let start = self.position;
        self.position += char.text_len();

        let is_whitespace = matches!(char, ' ' | '\n' | '\t' | '\r');

        while let Some(next) = self.chars.peek() {
            let next_is_whitespace = matches!(next, ' ' | '\n' | '\t' | '\r');

            if is_whitespace != next_is_whitespace {
                break;
            }

            self.position += next.text_len();
            self.chars.next();
        }

        let range = TextRange::new(start, self.position);
        let slice = &self.text[range];

        let chunk = if is_whitespace {
            JsxTextChunk::Whitespace(slice)
        } else {
            JsxTextChunk::Word(slice)
        };

        Some((start, chunk))
    }
}

impl FusedIterator for JsxSplitChunksIterator<'_> {}

#[cfg(test)]
mod tests {
    use crate::utils::jsx::{jsx_split_children, JsxChild, JsxSplitChunksIterator, JsxTextChunk};
    use rome_diagnostics::file::FileId;
    use rome_js_parser::parse;
    use rome_js_syntax::{JsxChildList, JsxText, SourceType};
    use rome_rowan::{AstNode, TextSize};

    fn assert_jsx_text_chunks(text: &str, expected_chunks: Vec<(TextSize, JsxTextChunk)>) {
        let parse = parse(
            &std::format!("<>{text}</>"),
            FileId::zero(),
            SourceType::jsx(),
        );
        assert!(
            !parse.has_errors(),
            "Source should not have any errors {:?}",
            parse.diagnostics()
        );

        let jsx_text = parse
            .syntax()
            .descendants()
            .find_map(JsxText::cast)
            .expect("Expected a JSX Text child");

        let value_token = jsx_text.value_token().unwrap();
        let chunks = JsxSplitChunksIterator::new(value_token.text()).collect::<Vec<_>>();
        assert_eq!(chunks, expected_chunks);
    }

    #[test]
    fn jsx_split_chunks_iterator() {
        assert_jsx_text_chunks(
            "a b c",
            vec![
                (TextSize::from(0), JsxTextChunk::Word("a")),
                (TextSize::from(1), JsxTextChunk::Whitespace(" ")),
                (TextSize::from(2), JsxTextChunk::Word("b")),
                (TextSize::from(3), JsxTextChunk::Whitespace(" ")),
                (TextSize::from(4), JsxTextChunk::Word("c")),
            ],
        );

        // merges consequent spaces
        assert_jsx_text_chunks(
            "a\n\rb",
            vec![
                (TextSize::from(0), JsxTextChunk::Word("a")),
                (TextSize::from(1), JsxTextChunk::Whitespace("\n\r")),
                (TextSize::from(3), JsxTextChunk::Word("b")),
            ],
        );

        // merges consequent non whitespace characters
        assert_jsx_text_chunks(
            "abcd efg",
            vec![
                (TextSize::from(0), JsxTextChunk::Word("abcd")),
                (TextSize::from(4), JsxTextChunk::Whitespace(" ")),
                (TextSize::from(5), JsxTextChunk::Word("efg")),
            ],
        );

        // whitespace at the beginning
        assert_jsx_text_chunks(
            "\n\n abcd",
            vec![
                (TextSize::from(0), JsxTextChunk::Whitespace("\n\n ")),
                (TextSize::from(3), JsxTextChunk::Word("abcd")),
            ],
        );

        // whitespace at the end
        assert_jsx_text_chunks(
            "abcd \n\n",
            vec![
                (TextSize::from(0), JsxTextChunk::Word("abcd")),
                (TextSize::from(4), JsxTextChunk::Whitespace(" \n\n")),
            ],
        );
    }

    fn parse_jsx_children(children: &str) -> JsxChildList {
        let parse = parse(
            &std::format!("<div>{children}</div>"),
            FileId::zero(),
            SourceType::jsx(),
        );

        assert!(
            !parse.has_errors(),
            "Expected source text to not have any errors: {:?}",
            parse.diagnostics()
        );

        parse
            .syntax()
            .descendants()
            .find_map(JsxChildList::cast)
            .expect("Expect a JsxChildList")
    }

    #[test]
    fn split_children_words_only() {
        let child_list = parse_jsx_children("a b c");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(3, children.len());
        assert_word(&children[0], "a");
        assert_word(&children[1], "b");
        assert_word(&children[2], "c");
    }

    #[test]
    fn split_non_meaningful_text() {
        let child_list = parse_jsx_children("  \n ");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(children, vec![]);
    }

    #[test]
    fn split_non_meaningful_leading_multiple_lines() {
        let child_list = parse_jsx_children("  \n  \n ");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(children, vec![JsxChild::EmptyLine]);
    }

    #[test]
    fn split_meaningful_whitespace() {
        let child_list = parse_jsx_children("  ");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(children, vec![JsxChild::Whitespace]);
    }

    #[test]
    fn split_children_leading_newlines() {
        let child_list = parse_jsx_children("  \n a b");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(3, children.len());
        assert_eq!(children[0], JsxChild::Newline);
        assert_word(&children[1], "a");
        assert_word(&children[2], "b");
    }

    #[test]
    fn split_children_trailing_whitespace() {
        let child_list = parse_jsx_children("a b    \t ");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(3, children.len());
        assert_word(&children[0], "a");
        assert_word(&children[1], "b");
        assert_eq!(children[2], JsxChild::Whitespace);
    }

    #[test]
    fn split_children_trailing_newline() {
        let child_list = parse_jsx_children("a b \n   \t ");

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(3, children.len());
        assert_word(&children[0], "a");
        assert_word(&children[1], "b");
        assert_eq!(children[2], JsxChild::Newline);
    }

    #[test]
    fn split_children_empty_expression() {
        let child_list = parse_jsx_children(r#"a{' '}c{" "}"#);

        let children = jsx_split_children(&child_list).unwrap();

        assert_eq!(
            4,
            children.len(),
            "Expected to contain four elements. Actual:\n{children:#?} "
        );
        assert_word(&children[0], "a");
        assert_eq!(children[1], JsxChild::Whitespace);
        assert_word(&children[2], "c");
        assert_eq!(children[3], JsxChild::Whitespace);
    }

    fn assert_word(child: &JsxChild, text: &str) {
        match child {
            JsxChild::Word(word) => {
                assert_eq!(word.text.text(), text)
            }
            child => {
                panic!("Expected a word but found {child:#?}");
            }
        }
    }
}
