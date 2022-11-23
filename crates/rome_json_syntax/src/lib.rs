#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
pub use rome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

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
        matches!(self, JsonSyntaxKind::NEWLINE | JsonSyntaxKind::WHITESPACE)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![null] | T![true] | T![false])
    }
}

impl rome_rowan::SyntaxKind for JsonSyntaxKind {
    const TOMBSTONE: Self = JsonSyntaxKind::TOMBSTONE;

    fn is_unknown(&self) -> bool {
        matches!(self, JsonSyntaxKind::JSON_UNKNOWN)
    }

    fn to_unknown(&self) -> Self {
        JsonSyntaxKind::JSON_UNKNOWN
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, JsonSyntaxKind::JSON_ROOT)
    }

    fn is_list(&self) -> bool {
        JsonSyntaxKind::is_list(*self)
    }

    fn to_string(&self) -> Option<&'static str> {
        JsonSyntaxKind::to_string(self)
    }
}

impl TryFrom<JsonSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: JsonSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsonSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                JsonSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
