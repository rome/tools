#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
pub use rome_rowan::{
    SyntaxText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use syntax_node::*;

use crate::JsonSyntaxKind::*;
use rome_rowan::RawSyntaxKind;

impl From<u16> for JsonSyntaxKind {
    fn from(d: u16) -> JsonSyntaxKind {
        assert!(d <= (JsonSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, JsonSyntaxKind>(d) }
    }
}

impl From<JsonSyntaxKind> for u16 {
    fn from(k: JsonSyntaxKind) -> u16 {
        k as u16
    }
}

impl JsonSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            JsonSyntaxKind::NEWLINE | JsonSyntaxKind::WHITESPACE | JsonSyntaxKind::COMMENT
        )
    }

    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        true
    }

}

impl rome_rowan::SyntaxKind for JsonSyntaxKind {
    fn is_unknown(&self) -> bool {
        matches!(self, Json_UNKNOWN)
    }

    fn to_unknown(&self) -> Self {
        todo!()
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }
}

impl TryFrom<JsonSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: JsonSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsonSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                JsonSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                JsonSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
