use crate::{
    JsSyntaxKind::{IDENT, JSX_IDENT, JSX_STRING_LITERAL, JS_STRING_LITERAL},
    JsSyntaxToken,
};

use std::ops::Deref;

#[derive(PartialEq, Eq, Clone)]
pub struct QuotedString(JsSyntaxToken);

/// A string literal that is wrapped in quotes
impl QuotedString {
    pub fn new(token: JsSyntaxToken) -> Self {
        assert!(matches!(
            token.kind(),
            IDENT | JSX_IDENT | JS_STRING_LITERAL | JSX_STRING_LITERAL
        ));

        Self(token)
    }

    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::static_value::QuotedString;
    /// use rome_js_factory::make::ident;
    /// use rome_rowan::TriviaPieceKind;
    ///
    /// let ident = ident("\"foo\"").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// assert_eq!(QuotedString::new(ident).text(), "foo");
    /// ```
    pub fn text(&self) -> &str {
        self.0
            .text_trimmed()
            .trim_start_matches(['"', '\''])
            .trim_end_matches(['"', '\''])
    }

    /// Get the inner text of a string including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::static_value::QuotedString;
    /// use rome_js_factory::make::ident;
    /// use rome_rowan::TriviaPieceKind;
    ///
    /// let ident = ident("\"foo\"").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// assert_eq!(QuotedString::new(ident).quoted_text(), "\"foo\"");
    /// ```
    pub fn quoted_text(&self) -> &str {
        self.0.text_trimmed()
    }
}

impl Deref for QuotedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

/// static values defined in JavaScript's expressions
pub enum StaticValue {
    Boolean(JsSyntaxToken),
    Null(JsSyntaxToken),
    Undefined(JsSyntaxToken),
    Number(JsSyntaxToken),
    BigInt(JsSyntaxToken),
    String(QuotedString),
    TemplateChunk(Option<JsSyntaxToken>),
}

impl StaticValue {
    /// Return `true` if the value is falsy
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{T, static_value::StaticValue};
    /// use rome_js_factory::make::{js_boolean_literal_expression, token};
    ///
    /// let bool = js_boolean_literal_expression(token(T![false]));
    /// assert!(StaticValue::Boolean(bool.value_token().ok().unwrap()).is_falsy());
    /// ```
    pub fn is_falsy(&self) -> bool {
        match self {
            StaticValue::Boolean(token) => token.text_trimmed() == "false",
            StaticValue::Null(_) => true,
            StaticValue::Undefined(_) => true,
            StaticValue::Number(token) => matches!(token.text_trimmed(), "0" | "-0" | "+0" | "NaN"),
            StaticValue::BigInt(token) => matches!(token.text_trimmed(), "0n"),
            StaticValue::String(token) => token.text().is_empty(),
            StaticValue::TemplateChunk(token) => token
                .as_ref()
                .map_or(true, |it| it.text_trimmed().is_empty()),
        }
    }

    /// Return a string of the static value
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{T, static_value::StaticValue};
    /// use rome_js_factory::make::{js_boolean_literal_expression, token};
    ///
    /// let bool = js_boolean_literal_expression(token(T![false]));
    /// assert_eq!(StaticValue::Boolean(bool.value_token().ok().unwrap()).text(), "false");
    /// ```
    pub fn text(&self) -> &str {
        match self {
            StaticValue::Boolean(token) => token.text_trimmed(),
            StaticValue::Null(token) => token.text_trimmed(),
            StaticValue::Undefined(token) => token.text_trimmed(),
            StaticValue::Number(token) => token.text_trimmed(),
            StaticValue::BigInt(token) => token.text_trimmed(),
            StaticValue::String(token) => token.text(),
            StaticValue::TemplateChunk(token) => token.as_ref().map_or("", |it| it.text_trimmed()),
        }
    }

    /// Return `true` if the static value match the given string value and it is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::static_value::{StaticValue, QuotedString};
    /// use rome_js_factory::make::{js_string_literal_expression, ident};
    /// use rome_rowan::TriviaPieceKind;
    ///
    /// let ident = ident("\"foo\"").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// let quoted_string = QuotedString::new(ident);
    /// assert!(StaticValue::String(quoted_string).is_string_constant("foo"));
    /// ```
    pub fn is_string_constant(&self, text: &str) -> bool {
        match self {
            StaticValue::String(_) | StaticValue::TemplateChunk(_) => self.text() == text,
            _ => false,
        }
    }

    /// Return a string if the static value is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::static_value::{StaticValue, QuotedString};
    /// use rome_js_factory::make::{js_string_literal_expression, ident};
    /// use rome_rowan::TriviaPieceKind;
    ///
    /// let ident = ident("\"foo\"").with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// let quoted_string = QuotedString::new(ident);
    /// assert_eq!(StaticValue::String(quoted_string).as_string_constant().unwrap(), "foo");
    /// ```
    pub fn as_string_constant(&self) -> Option<&str> {
        match self {
            StaticValue::String(_) | StaticValue::TemplateChunk(_) => Some(self.text()),
            _ => None,
        }
    }

    /// Return `true` if the value is null or undefined
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::{T, static_value::StaticValue};
    /// use rome_js_factory::make::{js_null_literal_expression, token};
    ///
    /// let null = js_null_literal_expression(token(T![null]));
    /// assert!(StaticValue::Null(null.value_token().ok().unwrap()).is_null_or_undefined());
    /// ```
    pub fn is_null_or_undefined(&self) -> bool {
        matches!(self, StaticValue::Null(_) | StaticValue::Undefined(_))
    }
}
