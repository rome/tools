use crate::{
    JsSyntaxKind::{IDENT, JSX_IDENT, JSX_STRING_LITERAL, JS_STRING_LITERAL},
    JsSyntaxToken,
};

use std::ops::Deref;

#[derive(PartialEq, Eq, Clone)]
pub struct QuotedString(JsSyntaxToken);

impl QuotedString {
    pub fn new(token: JsSyntaxToken) -> Self {
        assert!(matches!(
            token.kind(),
            IDENT | JSX_IDENT | JS_STRING_LITERAL | JSX_STRING_LITERAL
        ));

        Self(token)
    }

    pub fn text(&self) -> &str {
        self.0
            .text_trimmed()
            .trim_start_matches(['"', '\''])
            .trim_end_matches(['"', '\''])
    }

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

    pub fn is_string_constant(&self, text: &str) -> bool {
        match self {
            StaticValue::String(_) | StaticValue::TemplateChunk(_) => self.text() == text,
            _ => false,
        }
    }

    pub fn as_string_constant(&self) -> Option<&str> {
        match self {
            StaticValue::String(_) | StaticValue::TemplateChunk(_) => Some(self.text()),
            _ => None,
        }
    }

    pub fn is_null_or_undefined(&self) -> bool {
        matches!(self, StaticValue::Null(_) | StaticValue::Undefined(_))
    }
}
