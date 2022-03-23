use crate::arc::{HeaderSlice, ThinArc};
use crate::TriviaPiece;
use bitfield::BitRange;
use countme::Count;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;
use std::mem::ManuallyDrop;
use text_size::TextSize;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct GreenTriviaHead {
    _c: Count<GreenTrivia>,
}

type ReprThin = HeaderSlice<GreenTriviaHead, [TriviaPiece; 0]>;

#[repr(transparent)]
pub(crate) struct GreenTriviaData {
    data: ReprThin,
}

impl GreenTriviaData {
    #[allow(unused)]
    #[inline]
    pub fn header(&self) -> &GreenTriviaHead {
        &self.data.header
    }

    #[inline]
    pub fn pieces(&self) -> &[TriviaPiece] {
        self.data.slice()
    }
}

impl PartialEq for GreenTriviaData {
    fn eq(&self, other: &Self) -> bool {
        self.pieces() == other.pieces()
    }
}

impl fmt::Debug for GreenTriviaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.pieces().iter()).finish()
    }
}

type TriviaPtr = ThinArc<GreenTriviaHead, TriviaPiece>;

bitfield::bitfield! {
    #[derive(Clone, Copy)]
    pub struct GreenTriviaBits([TriviaPiece; 2]);
    no default BitRange;
    impl Debug;
    /// Set to true if a valid [TriviaPiece] is being stored in inline slot 0
    has_0, set_has_0: 0;
    /// Set to true if a valid [TriviaPiece] is being stored in inline slot 1
    has_1, set_has_1: 32;
}

impl BitRange<u8> for GreenTriviaBits {
    fn bit_range(&self, msb: usize, lsb: usize) -> u8 {
        if msb < 32 && lsb < 32 {
            return self.0[0].bit_range(msb, lsb);
        }

        if msb >= 32 && lsb >= 32 {
            return self.0[1].bit_range(msb - 32, lsb - 32);
        }

        // All the fields in GreenTriviaBits are single bit so reads can only
        // fall in the first or second TriviaPiece but never in-between
        unreachable!("unsupported")
    }

    fn set_bit_range(&mut self, msb: usize, lsb: usize, value: u8) {
        if msb < 32 && lsb < 32 {
            return self.0[0].set_bit_range(msb, lsb, value);
        }

        if msb >= 32 && lsb >= 32 {
            return self.0[1].set_bit_range(msb - 32, lsb - 32, value);
        }

        // All the fields in GreenTriviaBits are single bit so reads can only
        // fall in the first or second TriviaPiece but never in-between
        unreachable!("unsupported")
    }
}

impl GreenTriviaBits {
    /// Creates an empty [GreenTrivia]
    const fn empty() -> Self {
        Self([TriviaPiece::zeroed(); 2])
    }

    /// Returns true if this [GreenTrivia] contains no piece, that is if its
    /// binary representation is all zeroes
    const fn is_empty(&self) -> bool {
        self.0[0].is_zero() && self.0[1].is_zero()
    }

    /// Returns the number of trivia pieces contained in this GreenTrivia
    fn len(&self) -> usize {
        self.has_0() as usize + self.has_1() as usize
    }
}

/// Internal memory layout of GreenTrivia
///
/// A [GreenTrivia] is represented in memory as a 64-bits integer that is either:
/// - If the value is 0, this is an empty trivia
/// - If the least significant bit of the value is 1, its interpreted as one or
/// two [TriviaPiece] stored inline in the [GreenTrivia]
/// - Otherwise the value is interpreted as a [TriviaPtr], that is a [ThinArc]
/// pointing to the actual slice of [TriviaPiece]
///
/// This encoding relies on the following invariants:
/// - The pointer contained in a [ThinArc] is `NonNull`, meaning a valid
/// [ThinArc] will never contain a value where all the bits are zeroes
/// - The data pointed to by a [ThinArc] contains an `AtomicUsize` and is thus
/// aligned to a pointer-sized boundary (8 bytes on 64 bits architectures).
/// This means the three least-significant bits of a valid [ThinArc] will
/// always be zero
/// - The layout of [TriviaPiece] is specified manually using `bitfield` to fit
/// within 32 bits, and have its least significant bit set to one for all valid
/// values
union GreenTriviaRepr {
    bits: GreenTriviaBits,
    ptr: ManuallyDrop<TriviaPtr>,
}

/// List of trivia. Used to store either the leading or trailing trivia of a token.
/// The identity of a trivia is defined by the kinds and lengths of its items but not by
/// the texts of an individual piece. That means, that `\r` and `\n` can both be represented
/// by the same trivia, a trivia with a single `LINEBREAK` piece with the length 1.
/// This is safe because the text is stored on the token to which the trivia belongs and
/// `a\n` and `a\r` never resolve to the same tokens. Thus, they only share the trivia but are
/// otherwise two different tokens.
#[repr(transparent)]
pub(crate) struct GreenTrivia {
    inner: GreenTriviaRepr,
}

impl Eq for GreenTrivia {}

impl PartialEq for GreenTrivia {
    fn eq(&self, other: &Self) -> bool {
        self.pieces() == other.pieces()
    }
}

impl Hash for GreenTrivia {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pieces().hash(state);
    }
}

impl Clone for GreenTrivia {
    fn clone(&self) -> Self {
        // SAFETY: This reads the content of `inner` as plain bits first,
        // and either copies those directly or call clone on the contained
        // ThinArc if the content is determined to be a pointer
        unsafe {
            if self.inner.bits.is_empty() || self.inner.bits.has_0() {
                GreenTrivia {
                    inner: GreenTriviaRepr {
                        bits: self.inner.bits,
                    },
                }
            } else {
                GreenTrivia {
                    inner: GreenTriviaRepr {
                        ptr: self.inner.ptr.clone(),
                    },
                }
            }
        }
    }
}

impl fmt::Debug for GreenTrivia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.pieces(), f)
    }
}

impl GreenTrivia {
    /// Creates a new trivia containing the passed in pieces
    pub fn new<I>(pieces: I) -> Self
    where
        I: IntoIterator<Item = TriviaPiece>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut items = pieces.into_iter();
        if items.len() < 3 {
            let mut bits = GreenTriviaBits::empty();

            if let Some(piece) = items.next() {
                bits.0[0] = piece;
            }

            if let Some(piece) = items.next() {
                bits.0[1] = piece;
            }

            return GreenTrivia {
                inner: GreenTriviaRepr { bits },
            };
        }

        let data = GreenTriviaRepr {
            ptr: ManuallyDrop::new(ThinArc::from_header_and_iter(
                GreenTriviaHead { _c: Count::new() },
                items,
            )),
        };

        GreenTrivia { inner: data }
    }

    /// Returns the total length of all pieces
    pub fn text_len(&self) -> TextSize {
        let mut len: Option<TextSize> = Some(TextSize::default());

        for piece in self.pieces() {
            len = len.and_then(|len| len.checked_add(piece.length()))
        }

        // Realistically we will never have files bigger than usize::MAX, nor u32::MAX
        len.unwrap_or_else(|| TextSize::from(u32::MAX))
    }

    /// Returns the pieces count
    pub fn len(&self) -> usize {
        // SAFETY: This reads the content of inner as bits, and use it to
        // detect whether the content is empty, an inline trivia piece or a
        // pointer
        unsafe {
            if self.inner.bits.is_empty() {
                return 0;
            }

            if self.inner.bits.has_0() {
                return self.inner.bits.len();
            }

            self.inner.ptr.len()
        }
    }

    /// Returns the pieces of the trivia
    pub fn pieces(&self) -> &[TriviaPiece] {
        // SAFETY: This reads the content of inner as bits, and use it to
        // detect whether the content is empty, an inline trivia piece or a
        // pointer
        unsafe {
            if self.inner.bits.is_empty() {
                return &[];
            }

            if self.inner.bits.has_0() {
                let len = self.inner.bits.len();
                return &self.inner.bits.0[..len];
            }

            self.inner.ptr.slice()
        }
    }

    /// Returns the piece at the given index.
    pub fn get_piece(&self, index: usize) -> Option<&TriviaPiece> {
        self.pieces().get(index)
    }
}

impl Drop for GreenTrivia {
    fn drop(&mut self) {
        // SAFETY: This reads the content of inner as bits, and manually calls
        // the implementation of Drop for ThinArc if the content is determined
        // to be a pointer
        unsafe {
            if self.inner.bits.is_empty() || self.inner.bits.has_0() {
                return;
            }

            ManuallyDrop::drop(&mut self.inner.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::TriviaPieceKind;
    use crate::green::trivia::{GreenTrivia, GreenTriviaHead};
    use crate::TriviaPiece;
    use text_size::TextSize;

    impl GreenTrivia {
        /// Creates a trivia with a single whitespace piece
        pub fn whitespace<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::Whitespace, len.into())
        }

        /// Creates a trivia with one single line comment piece
        pub fn single_line_comment<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::SingleLineComment, len.into())
        }

        /// Creates a trivia containing a single piece
        pub fn single<L: Into<TextSize>>(kind: TriviaPieceKind, len: L) -> Self {
            Self::new(std::iter::once(TriviaPiece::new(kind, len)))
        }
    }

    #[test]
    fn sizes() {
        assert_eq!(0, std::mem::size_of::<GreenTriviaHead>());
        assert_eq!(8, std::mem::size_of::<GreenTrivia>());
    }
}
