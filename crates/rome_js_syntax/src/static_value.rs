use rome_rowan::TextRange;

use crate::JsSyntaxToken;

#[derive(Debug, Clone, Eq, PartialEq)]
/// static values defined in JavaScript's expressions
pub enum StaticValue {
    Boolean(JsSyntaxToken),
    Null(JsSyntaxToken),
    Undefined(JsSyntaxToken),
    Number(JsSyntaxToken),
    NaN(JsSyntaxToken),
    BigInt(JsSyntaxToken),
    String(JsSyntaxToken),
    TemplateChunk(JsSyntaxToken),
    EmptyTemplate(TextRange),
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
            StaticValue::Null(_) | StaticValue::EmptyTemplate(_) => true,
            StaticValue::Undefined(_) | StaticValue::NaN(_) => true,
            StaticValue::Number(token) | StaticValue::BigInt(token) => {
                matches!(token.text_trimmed(), "0" | "0n")
            }
            StaticValue::String(_) | StaticValue::TemplateChunk(_) => self.text().is_empty(),
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
            StaticValue::Boolean(token)
            | StaticValue::Null(token)
            | StaticValue::Undefined(token)
            | StaticValue::Number(token)
            | StaticValue::NaN(token)
            | StaticValue::BigInt(token) => token.text_trimmed(),
            StaticValue::String(token) => {
                let text_trimmed = token.text_trimmed();
                &text_trimmed[1..text_trimmed.len() - 1]
            }
            StaticValue::TemplateChunk(token) => token.text_trimmed(),
            StaticValue::EmptyTemplate(_) => "",
        }
    }

    /// Return a string if the static value is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_syntax::static_value::StaticValue;
    /// use rome_js_factory::make;
    /// use rome_rowan::TriviaPieceKind;
    ///
    /// let literal = make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// assert_eq!(StaticValue::String(literal).as_string_constant().unwrap(), "foo");
    /// ```
    pub fn as_string_constant(&self) -> Option<&str> {
        match self {
            StaticValue::String(_)
            | StaticValue::TemplateChunk(_)
            | StaticValue::EmptyTemplate(_) => Some(self.text()),
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
