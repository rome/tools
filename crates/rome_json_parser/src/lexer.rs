use std::ops::Range;

use logos::Logos;
use rome_diagnostics::Diagnostic;
use rome_json_syntax::{JsonSyntaxKind, TextRange, TextSize, T};

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[regex(r#""([^"\\]|\\u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]|\\["\\/bfnrt])*""#)]
    String,

    #[regex(r#"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?"#)]
    Number,

    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r"\r\n|\n")]
    NewLine,

    #[error]
    Error,
}

impl From<&TokenKind> for JsonSyntaxKind {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::LeftBrace => T!['{'],
            TokenKind::RightBrace => T!['}'],
            TokenKind::Colon => T![:],
            TokenKind::Comma => T![,],
            TokenKind::LeftBracket => T!['['],
            TokenKind::RightBracket => T![']'],
            TokenKind::True => T![true],
            TokenKind::False => T![false],
            TokenKind::Null => T![null],
            TokenKind::String => JsonSyntaxKind::JSON_STRING_LITERAL,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER_LITERAL,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::NewLine => JsonSyntaxKind::NEWLINE,
            TokenKind::Error => JsonSyntaxKind::ERROR_TOKEN,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Vector of tokens, with it's [JsonSyntaxKind] and [TextRange].
    tokens_with_span: Vec<(JsonSyntaxKind, TextRange)>,
    /// Source code of tokens
    source: &'a str,
    /// Custom file_id
    file_id: usize,
    /// Current cursor of non_trivia_index_list, we use such way to make our get next non trivia token function faster in parser phase.
    /// If we want to get the first non trivia token, we need to get the index of the first non trivia token in `token_with_span` vector.
    /// then use the index to get `(kind, range)` from `token_with_span` vector.
    /// ```rs
    /// let index = lexer.non_trivia_index_list[lexer.none_trivia_index_cursor];
    /// let (token_kind, token_range) = lexer.token_with_span[index];
    /// ```
    /// The benefit of doing so is we could easily collecting trivia token between latest none trivia token and next none trivia token by
    /// just iterating `tokens_with_span` vector from index of latest none trivia token to index of next none trivia token.
    /// ```rs
    /// let left = lexer.current_none_trivia_cursor();
    /// let right = lexer.next_none_trivia_cursor();
    /// for index in (left + 1)..right {
    ///   let trivia_token = lexer.token_with_span[index];
    ///   // do something here.
    /// }
    /// ```
    none_trivia_cursor: usize,
    /// We collect all index of non-trivia in the `token_with_span` ahead of time.
    non_trivia_index_list: Vec<usize>,
    error_token_range_list: Vec<TextRange>,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        let mut lexer = LogosLexer::new(source).peekable();
        let mut tokens_with_span = vec![];
        let mut non_trivia_index_list = vec![];
        let mut error_token_range_list = vec![];
        let mut i = 0;
        while let Some((kind, range)) = lexer.next() {
            let (start, end, kind) = match kind {
                TokenKind::Whitespace | TokenKind::NewLine => (range.start, range.end, kind),
                TokenKind::Error => {
                    // Because `logos` has a badly error recover strategy, we need to merge continuous error tokens.
                    let mut end = range.end;
                    while let Some((TokenKind::Error, inner_loop_range)) = lexer.peek() {
                        end = inner_loop_range.end;
                        lexer.next();
                    }
                    error_token_range_list.push(TextRange::new(
                        (range.start as u32).into(),
                        (end as u32).into(),
                    ));
                    (range.start, end, kind)
                }
                _ => {
                    non_trivia_index_list.push(i);
                    (range.start, range.end, kind)
                }
            };

            tokens_with_span.push((
                (&kind).into(),
                TextRange::new(TextSize::from(start as u32), TextSize::from(end as u32)),
            ));
            i += 1;
        }

        Self {
            tokens_with_span,
            source,
            file_id,
            none_trivia_cursor: 0,
            non_trivia_index_list,
            error_token_range_list,
        }
    }

    /// Get nth non trivia token_kind from latest_cursor, return `current_token_kind` if n = 0.
    pub fn nth(&self, n: usize) -> JsonSyntaxKind {
        let index = self.none_trivia_cursor + n;
        match self.non_trivia_index_list.get(index) {
            Some(i) => {
                let (kind, _) = &self.tokens_with_span[*i];
                *kind
            }
            None => JsonSyntaxKind::EOF,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tokens_with_span.len()
    }

    /// Get the next non trivia token and pointed the latest to it.
    pub fn advance(&mut self) {
        self.none_trivia_cursor += 1;
    }

    pub fn file_id(&self) -> usize {
        self.file_id
    }
    // Currently it is only used in testing
    #[cfg(test)]
    pub(crate) fn current_token_slice(&self) -> &'a str {
        &self.source[self.current_range()]
    }

    /// Get current token_kind of latest non trivia token.
    pub fn current_token_kind(&self) -> JsonSyntaxKind {
        self.current()
            .map(|item| item.0)
            .unwrap_or_else(|| JsonSyntaxKind::EOF)
    }

    /// Get current (token_kind, range) of latest non trivia token.
    fn current(&self) -> Option<&(JsonSyntaxKind, TextRange)> {
        self.non_trivia_index_list
            .get(self.none_trivia_cursor)
            .map(|i| &self.tokens_with_span[*i])
    }

    /// Get any (token_kind, range) at given index, including both trivia and none trivia tokens.
    pub fn token_at(&self, i: usize) -> (JsonSyntaxKind, TextRange) {
        *self.tokens_with_span.get(i).unwrap_or({
            &(
                JsonSyntaxKind::EOF,
                TextRange::new(
                    TextSize::from(self.source.len() as u32),
                    TextSize::from(self.source.len() as u32),
                ),
            )
        })
    }

    /// Get text range of latest none trivia token.
    pub fn current_range(&self) -> TextRange {
        match self.current() {
            Some((_, range)) => *range,
            None => TextRange::new(
                TextSize::from(self.source.len() as u32),
                TextSize::from(self.source.len() as u32),
            ),
        }
    }

    /// Get source code of lexing phase.
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Get all diagnostics of lexing phase.
    pub fn finish(self) -> Vec<Diagnostic> {
        self.error_token_range_list
            .into_iter()
            .map(|range| {
                Diagnostic::error(self.file_id, self.source, "Lexing error")
                    .primary(range, "Unexpected token")
            })
            .collect()
    }

    /// Get index of `tokens_with_span` vector of latest none trivia token.
    #[inline]
    pub fn current_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor))
            .unwrap_or(&self.tokens_with_span.len())
    }

    /// Get index of `tokens_with_span` vector of next none trivia token.
    #[inline]
    pub fn next_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor + 1))
            .unwrap_or(&self.tokens_with_span.len())
    }

    pub fn tokens_with_span(&self) -> &[(JsonSyntaxKind, TextRange)] {
        self.tokens_with_span.as_ref()
    }
}

pub struct LogosLexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> LogosLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for LogosLexer<'a> {
    type Item = (TokenKind, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let span = self.inner.span();

        Some((kind, span))
    }
}

#[cfg(test)]
mod test_lexer {

    use super::*;

    fn lexing_kind(content: &str) -> Vec<JsonSyntaxKind> {
        let mut lexer = Lexer::new(content, 0);
        let mut tokens = vec![];
        while !matches!(lexer.current_token_kind(), JsonSyntaxKind::EOF) {
            tokens.push(lexer.current_token_kind());
            lexer.advance();
        }
        tokens
    }

    fn lexing_kind_and_content(content: &str) -> Vec<(JsonSyntaxKind, &str)> {
        let lexer = Lexer::new(content, 0);
        let mut tokens = vec![];
        for (token, span) in lexer
            .tokens_with_span
            .iter()
            .take_while(|(k, _)| k != &JsonSyntaxKind::EOF)
        {
            tokens.push((
                *token,
                &content[u32::from(span.start()) as usize..u32::from(span.end()) as usize],
            ));
        }
        tokens
    }

    fn assert_lexing_kind(content_list: Vec<&str>, kind: Vec<Vec<JsonSyntaxKind>>) {
        for i in 0..content_list.len() {
            let content = content_list[i];
            assert_eq!(lexing_kind(content), kind[i]);
        }
    }

    fn assert_lexing_kind_and_content(
        content_list: Vec<&str>,
        kind: Vec<Vec<(JsonSyntaxKind, &str)>>,
    ) {
        for i in 0..content_list.len() {
            let content = content_list[i];
            assert_eq!(lexing_kind_and_content(content), kind[i]);
        }
    }
    #[test]
    fn test_literal() {
        assert_lexing_kind(
            vec!["true", "false", "null"],
            vec![vec![T![true]], vec![T![false]], vec![T![null]]],
        );
    }

    #[test]
    fn test_number() {
        assert_lexing_kind_and_content(
            vec!["1", "1.0", "1.000", "1.5", "-1.5", "123e5", "123e-5"],
            vec![
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "1")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "1.0")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "1.000")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "1.5")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "-1.5")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "123e5")],
                vec![(JsonSyntaxKind::JSON_NUMBER_LITERAL, "123e-5")],
            ],
        );
    }

    #[test]
    fn test_string_literal() {
        assert_lexing_kind_and_content(
            vec![
                r#""""#,
                r#""a""#,
                r#""abcd""#,
                r#""\"\\/\\b\\t\\n\\f\\r\\\\""#,
                r#""hi \\u0066\\u0069\\u006E\\u006E""#,
            ],
            vec![
                vec![(JsonSyntaxKind::JSON_STRING_LITERAL, "\"\"")],
                vec![(JsonSyntaxKind::JSON_STRING_LITERAL, "\"a\"")],
                vec![(JsonSyntaxKind::JSON_STRING_LITERAL, "\"abcd\"")],
                vec![(
                    JsonSyntaxKind::JSON_STRING_LITERAL,
                    r#""\"\\/\\b\\t\\n\\f\\r\\\\""#,
                )],
                vec![(
                    JsonSyntaxKind::JSON_STRING_LITERAL,
                    r#""hi \\u0066\\u0069\\u006E\\u006E""#,
                )],
            ],
        );
    }

    #[test]
    fn test_illegals() {
        use JsonSyntaxKind::*;
        assert_lexing_kind_and_content(
            vec!["012", "2e", "1.", "-", r#""hi"#, r#""\0""#, "undefined"],
            vec![
                vec![(JSON_NUMBER_LITERAL, "0"), (JSON_NUMBER_LITERAL, "12")],
                vec![(JSON_NUMBER_LITERAL, "2"), (ERROR_TOKEN, "e")],
                vec![(JSON_NUMBER_LITERAL, "1"), (ERROR_TOKEN, ".")],
                vec![(JsonSyntaxKind::ERROR_TOKEN, "-")],
                vec![(JsonSyntaxKind::ERROR_TOKEN, "\"hi")],
                vec![
                    (ERROR_TOKEN, "\"\\"),
                    (JSON_NUMBER_LITERAL, "0"),
                    (ERROR_TOKEN, "\""),
                ],
                vec![(ERROR_TOKEN, "undefined")],
            ],
        );
    }

    #[test]
    fn test_array() {
        use JsonSyntaxKind::*;
        assert_lexing_kind_and_content(
            vec!["[]", "[1]", "[1,2]", "[true,2,\"3\"]"],
            vec![
                vec![(L_BRACK, "["), (R_BRACK, "]")],
                vec![(L_BRACK, "["), (JSON_NUMBER_LITERAL, "1"), (R_BRACK, "]")],
                vec![
                    (L_BRACK, "["),
                    (JSON_NUMBER_LITERAL, "1"),
                    (T![,], ","),
                    (JSON_NUMBER_LITERAL, "2"),
                    (R_BRACK, "]"),
                ],
                vec![
                    (L_BRACK, "["),
                    (T![true], "true"),
                    (T![,], ","),
                    (JSON_NUMBER_LITERAL, "2"),
                    (T![,], ","),
                    (JSON_STRING_LITERAL, r#""3""#),
                    (R_BRACK, "]"),
                ],
            ],
        );
    }

    #[test]
    fn test_object() {
        use JsonSyntaxKind::*;
        assert_lexing_kind_and_content(
            vec![
                r#"{"a":"b"}"#,
                "\t\t\n\t{\"a\":  \t  \"b\"}",
                r#"{"a" : "b"}"#,
                "\t{\"a\" : \"b\"\n}\t",
                r#"{"a":{"b":1}}"#,
            ],
            vec![
                vec![
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"a\""),
                    (T![:], ":"),
                    (JSON_STRING_LITERAL, "\"b\""),
                    (T!['}'], "}"),
                ],
                vec![
                    (WHITESPACE, "\t\t"),
                    (NEWLINE, "\n"),
                    (WHITESPACE, "\t"),
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"a\""),
                    (T![:], ":"),
                    (WHITESPACE, "  \t  "),
                    (JSON_STRING_LITERAL, "\"b\""),
                    (T!['}'], "}"),
                ],
                vec![
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"a\""),
                    (WHITESPACE, " "),
                    (T![:], ":"),
                    (WHITESPACE, " "),
                    (JSON_STRING_LITERAL, "\"b\""),
                    (T!['}'], "}"),
                ],
                vec![
                    (WHITESPACE, "\t"),
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"a\""),
                    (WHITESPACE, " "),
                    (T![:], ":"),
                    (WHITESPACE, " "),
                    (JSON_STRING_LITERAL, "\"b\""),
                    (NEWLINE, "\n"),
                    (T!['}'], "}"),
                    (WHITESPACE, "\t"),
                ],
                vec![
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"a\""),
                    (T![:], ":"),
                    (T!['{'], "{"),
                    (JSON_STRING_LITERAL, "\"b\""),
                    (T![:], ":"),
                    (JSON_NUMBER_LITERAL, "1"),
                    (T!['}'], "}"),
                    (T!['}'], "}"),
                ],
            ],
        );
    }
}
