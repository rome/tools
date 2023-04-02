use crate::{
    JsSyntaxKind::{JSX_IDENT, JSX_STRING_LITERAL, JS_STRING_LITERAL},
    JsSyntaxToken,
};

use std::ops::Deref;

#[derive(PartialEq, Eq, Clone)]
pub struct QuotedString(JsSyntaxToken);

impl QuotedString {
    pub fn new(token: JsSyntaxToken) -> Self {
        assert!(matches!(
            token.kind(),
            JSX_IDENT | JS_STRING_LITERAL | JSX_STRING_LITERAL
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

    pub fn is_null_or_undefined(&self) -> bool {
        matches!(self, StaticValue::Null(_) | StaticValue::Undefined(_))
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, StaticValue::Undefined(_))
    }
}

pub enum StringConstant {
    TemplateChunk(Option<JsSyntaxToken>),
    String(QuotedString),
}

impl StringConstant {
    pub fn text(&self) -> &str {
        match self {
            StringConstant::TemplateChunk(token) => {
                token.as_ref().map_or("", |it| it.text_trimmed())
            }
            StringConstant::String(quoted) => quoted.text(),
        }
    }
}

impl Deref for StringConstant {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl TryFrom<StaticValue> for StringConstant {
    type Error = ();

    fn try_from(from: StaticValue) -> Result<Self, Self::Error> {
        match from {
            StaticValue::TemplateChunk(token) => Ok(StringConstant::TemplateChunk(token)),
            StaticValue::String(token) => Ok(StringConstant::String(token)),
            _ => Err(()),
        }
    }
}
