#[macro_use]
mod generated;
pub mod ast;
mod syntax_node;
pub mod util;
pub use util::SyntaxNodeExt;

pub use self::generated::*;
pub use ast::{AstNode, AstNodeList, AstSeparatedList, AstToken, SyntaxError, SyntaxResult};
pub use rome_rowan::{
    SyntaxText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use syntax_node::*;

use crate::CssSyntaxKind::*;
use rome_rowan::RawSyntaxKind;

impl From<u16> for CssSyntaxKind {
    fn from(d: u16) -> CssSyntaxKind {
        assert!(d <= (CssSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, CssSyntaxKind>(d) }
    }
}

impl From<CssSyntaxKind> for u16 {
    fn from(k: CssSyntaxKind) -> u16 {
        k as u16
    }
}

impl CssSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            CssSyntaxKind::NEWLINE | CssSyntaxKind::WHITESPACE | CssSyntaxKind::COMMENT
        )
    }

    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        true
    }

    /// Returns `true` for contextual keywords (excluding strict mode contextual keywords)
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u16) >= (ALICEBLUE_KW as u16) && (self as u16) <= (CSS_SELECTOR as u16)
    }

    /// Returns true for all non-contextual keywords (includes future reserved keywords)
    #[inline]
    pub const fn is_non_contextual_keyword(self) -> bool {
        self.is_keyword() && !self.is_contextual_keyword()
    }
}

impl rome_rowan::SyntaxKind for CssSyntaxKind {
    fn is_unknown(&self) -> bool {
        matches!(self, CSS_UNKNOWN)
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

impl TryFrom<CssSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: CssSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                CssSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                CssSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                CssSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
