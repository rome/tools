#[macro_use]
mod generated;
pub mod syntax_node;

pub use self::generated::*;
pub use rome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

use rome_rowan::RawSyntaxKind;

impl From<u16> for MdSyntaxKind {
    fn from(d: u16) -> MdSyntaxKind {
        assert!(d <= (MdSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, MdSyntaxKind>(d) }
    }
}

impl From<MdSyntaxKind> for u16 {
    fn from(k: MdSyntaxKind) -> u16 {
        k as u16
    }
}

impl MdSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, MdSyntaxKind::NEWLINE | MdSyntaxKind::WHITESPACE)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        false
    }
}

impl rome_rowan::SyntaxKind for MdSyntaxKind {
    const TOMBSTONE: Self = MdSyntaxKind::TOMBSTONE;
    const EOF: Self = MdSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, MdSyntaxKind::MD_BOGUS)
    }

    fn to_bogus(&self) -> Self {
        match self {
            _ => MdSyntaxKind::MD_BOGUS,
        }
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
        matches!(self, MdSyntaxKind::MD_ROOT)
    }

    fn is_list(&self) -> bool {
        MdSyntaxKind::is_list(*self)
    }

    fn to_string(&self) -> Option<&'static str> {
        MdSyntaxKind::to_string(self)
    }
}

impl TryFrom<MdSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: MdSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                MdSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                MdSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
