use std::{
    borrow::Borrow,
    fmt,
    mem::{self, ManuallyDrop},
    ops, ptr,
};

use countme::Count;

use crate::api::TriviaPieceKind;
use crate::{
    api::TriviaPiece,
    arc::{Arc, HeaderSlice, ThinArc},
    green::RawSyntaxKind,
    TextSize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::box_collection)]
pub enum GreenTokenTrivia {
    None,
    Single(TriviaPieceKind, TextSize),
    Many(Box<Vec<TriviaPiece>>),
}

impl GreenTokenTrivia {
    pub fn whitespace<L: Into<TextSize>>(len: L) -> GreenTokenTrivia {
        GreenTokenTrivia::Single(TriviaPieceKind::Whitespace, len.into())
    }

    pub fn single_line_comment<L: Into<TextSize>>(len: L) -> GreenTokenTrivia {
        GreenTokenTrivia::Single(TriviaPieceKind::SingleLineComment, len.into())
    }

    pub fn text_len(&self) -> TextSize {
        match self {
            GreenTokenTrivia::None => 0.into(),
            GreenTokenTrivia::Single(_, len) => *len,
            GreenTokenTrivia::Many(v) => {
                let r = v.iter().fold(Some(TextSize::from(0)), |len, trivia| {
                    len.and_then(|x| x.checked_add(trivia.text_len()))
                });

                // Realistically we will never have files bigger than usize::MAX, nor u32::MAX
                r.unwrap_or_else(|| TextSize::from(u32::MAX))
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        match self {
            GreenTokenTrivia::None => 0,
            GreenTokenTrivia::Single(..) => 1,
            GreenTokenTrivia::Many(v) => v.len(),
        }
    }

    pub(crate) fn get_piece(&self, index: usize) -> Option<TriviaPiece> {
        match self {
            GreenTokenTrivia::Single(kind, len) => {
                if index == 0 {
                    Some(TriviaPiece::new(*kind, *len))
                } else {
                    None
                }
            }
            GreenTokenTrivia::Many(v) => v.get(index).copied(),
            GreenTokenTrivia::None => None,
        }
    }
}

impl From<&[TriviaPiece]> for GreenTokenTrivia {
    fn from(trivias: &[TriviaPiece]) -> Self {
        match trivias {
            [] => GreenTokenTrivia::None,
            [TriviaPiece { kind, length }] => GreenTokenTrivia::Single(*kind, *length),
            _ => GreenTokenTrivia::Many(Box::new(trivias.to_vec())),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct GreenTokenHead {
    kind: RawSyntaxKind,
    leading: GreenTokenTrivia,
    trailing: GreenTokenTrivia,
    _c: Count<GreenToken>,
}

type Repr = HeaderSlice<GreenTokenHead, [u8]>;
type ReprThin = HeaderSlice<GreenTokenHead, [u8; 0]>;
#[repr(transparent)]
pub(crate) struct GreenTokenData {
    data: ReprThin,
}

impl PartialEq for GreenTokenData {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind() && self.text() == other.text()
    }
}

/// Leaf node in the immutable tree.
#[derive(PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub(crate) struct GreenToken {
    ptr: ThinArc<GreenTokenHead, u8>,
}

impl ToOwned for GreenTokenData {
    type Owned = GreenToken;

    #[inline]
    fn to_owned(&self) -> GreenToken {
        unsafe {
            let green = GreenToken::from_raw(ptr::NonNull::from(self));
            let green = ManuallyDrop::new(green);
            GreenToken::clone(&green)
        }
    }
}

impl Borrow<GreenTokenData> for GreenToken {
    #[inline]
    fn borrow(&self) -> &GreenTokenData {
        &*self
    }
}

impl fmt::Debug for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GreenToken")
            .field("kind", &self.kind())
            .field("text", &self.text())
            .field("leading", &self.leading_trivia())
            .field("trailing", &self.trailing_trivia())
            .finish()
    }
}

impl fmt::Debug for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: &GreenTokenData = &*self;
        fmt::Debug::fmt(data, f)
    }
}

impl fmt::Display for GreenToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: &GreenTokenData = &*self;
        fmt::Display::fmt(data, f)
    }
}

impl fmt::Display for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl GreenTokenData {
    /// Kind of this Token.
    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        self.data.header.kind
    }

    /// Whole text of this Token, including all trivia.
    #[inline]
    pub fn text(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.data.slice()) }
    }

    pub(crate) fn leading_trailing_total_len(&self) -> (TextSize, TextSize, TextSize) {
        let leading_len = self.data.header.leading.text_len();
        let trailing_len = self.data.header.trailing.text_len();
        let total_len = self.data.slice().len() as u32;
        (leading_len, trailing_len, total_len.into())
    }

    /// Text of this Token, excluding all trivia.
    #[inline]
    pub fn text_trimmed(&self) -> &str {
        let (leading_len, trailing_len, total_len) = self.leading_trailing_total_len();

        let start: usize = leading_len.into();
        let end: usize = (total_len - trailing_len).into();
        let text = unsafe { std::str::from_utf8_unchecked(self.data.slice()) };
        &text[start..end]
    }

    #[inline]
    pub fn text_leading_trivia(&self) -> &str {
        let leading_len = self.leading_trivia().text_len();

        let end: usize = leading_len.into();
        let text = unsafe { std::str::from_utf8_unchecked(self.data.slice()) };
        &text[0..end]
    }

    #[inline]
    pub fn text_trailing_trivia(&self) -> &str {
        let (_, trailing_len, total_len) = self.leading_trailing_total_len();

        let start: usize = (total_len - trailing_len).into();
        let text = unsafe { std::str::from_utf8_unchecked(self.data.slice()) };
        &text[start..]
    }

    /// Returns the length of the text covered by this token.
    #[inline]
    pub fn text_len(&self) -> TextSize {
        TextSize::of(self.text())
    }

    #[inline]
    pub fn leading_trivia(&self) -> &GreenTokenTrivia {
        &self.data.header.leading
    }

    #[inline]
    pub fn trailing_trivia(&self) -> &GreenTokenTrivia {
        &self.data.header.trailing
    }
}

impl GreenToken {
    #[inline]
    #[cfg(test)]
    pub fn new(kind: RawSyntaxKind, text: &str) -> GreenToken {
        Self::with_trivia(kind, text, GreenTokenTrivia::None, GreenTokenTrivia::None)
    }

    #[inline]
    pub fn with_trivia(
        kind: RawSyntaxKind,
        text: &str,
        leading: GreenTokenTrivia,
        trailing: GreenTokenTrivia,
    ) -> GreenToken {
        let head = GreenTokenHead {
            kind,
            leading,
            trailing,
            _c: Count::new(),
        };
        let ptr = ThinArc::from_header_and_iter(head, text.bytes());
        GreenToken { ptr }
    }

    #[inline]
    pub(crate) fn into_raw(this: GreenToken) -> ptr::NonNull<GreenTokenData> {
        let green = ManuallyDrop::new(this);
        let green: &GreenTokenData = &*green;
        ptr::NonNull::from(&*green)
    }

    #[inline]
    pub(crate) unsafe fn from_raw(ptr: ptr::NonNull<GreenTokenData>) -> GreenToken {
        let arc = Arc::from_raw(&ptr.as_ref().data as *const ReprThin);
        let arc = mem::transmute::<Arc<ReprThin>, ThinArc<GreenTokenHead, u8>>(arc);
        GreenToken { ptr: arc }
    }
}

impl ops::Deref for GreenToken {
    type Target = GreenTokenData;

    #[inline]
    fn deref(&self) -> &GreenTokenData {
        unsafe {
            let repr: &Repr = &self.ptr;
            let repr: &ReprThin = &*(repr as *const Repr as *const ReprThin);
            mem::transmute::<&ReprThin, &GreenTokenData>(repr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::TriviaPiece;

    use super::*;
    use quickcheck_macros::*;

    #[test]
    fn green_token_text_and_len() {
        let t = GreenToken::with_trivia(
            RawSyntaxKind(0),
            "\n\t let \t\t",
            GreenTokenTrivia::whitespace(3),
            GreenTokenTrivia::whitespace(3),
        );

        assert_eq!("\n\t let \t\t", t.text());
        assert_eq!(TextSize::from(9), t.text_len());

        assert_eq!("let", t.text_trimmed());

        assert_eq!("\n\t ", t.text_leading_trivia());
        assert_eq!(" \t\t", t.text_trailing_trivia());

        assert_eq!("\n\t let \t\t", format!("{}", t));
    }

    #[test]
    fn none_text_len() {
        assert_eq!(TextSize::from(0), GreenTokenTrivia::None.text_len());
    }

    #[quickcheck]
    fn whitespace_and_comments_text_len(len: u32) {
        let len = TextSize::from(len);
        assert_eq!(len, GreenTokenTrivia::whitespace(len).text_len());
        assert_eq!(len, GreenTokenTrivia::single_line_comment(len).text_len());
    }

    #[test]
    fn many_text_len_dont_panic() {
        let trivia = GreenTokenTrivia::Many(Box::new(vec![
            TriviaPiece::whitespace(u32::MAX),
            TriviaPiece::single_line_comment(1),
        ]));
        assert_eq!(TextSize::from(u32::MAX), trivia.text_len());
    }

    #[quickcheck]
    fn many_text_len(lengths: Vec<u32>) {
        let trivia: Vec<_> = lengths
            .iter()
            .map(|x| TriviaPiece::whitespace(*x))
            .collect();
        let trivia = GreenTokenTrivia::Many(Box::new(trivia));

        let total_len = lengths.iter().fold(0u32, |acc, x| acc.saturating_add(*x));
        assert_eq!(TextSize::from(total_len), trivia.text_len());
    }

    #[test]
    fn sizes() {
        assert_eq!(16, std::mem::size_of::<GreenTokenTrivia>());
        assert_eq!(8, std::mem::size_of::<GreenToken>());
    }
}
