use crate::arc::{HeaderSlice, ThinArc};
use crate::TriviaPiece;
use countme::Count;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;
use std::mem::ManuallyDrop;
use std::slice;
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

/// Internal memory layout of GreenTrivia
///
/// A [GreenTrivia] is represented in memory as a 64-bits integer that is either:
/// - If the value is 0, this is an empty trivia
/// - If the least significant bit of the value is 1, its interpreted as a single
/// [TriviaPiece] stored inline in the [GreenTrivia]
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
/// - [TriviaPiece] and all the type it contains are using "forced
/// representations" ([TriviaPiece] itself is `repr(C)`, `TriviaPieceKind` is a
/// `repr(u8)` enum with manually specified discriminants, and `TextSize` is a
/// `repr(transparent)` newtype struct wrapping a `u32`): this allows this type
/// to have a stable memory layout that can be relied upon, an specifically to
/// uphold the invariant that the least-significant bit of a valid [TriviaPiece]
/// is alway set to one
/// - The target platform must be using little-endian byte order for the [TriviaPiece]
/// struct to be laid out correctly. This invariant is weaker than the previous ones
/// as it could be lifted by using a slightly different different logic depending on
/// the target platform endianness, but since Rome doesn't support any big-endian
/// platform for now the code is just set to fail compiling on those
#[cfg(target_endian = "little")]
union GreenTriviaRepr {
    bits: u64,
    inline: TriviaPiece,
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
            if self.inner.bits == 0 || (self.inner.bits & 1) == 1 {
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
        if items.len() == 0 {
            return GreenTrivia {
                inner: GreenTriviaRepr { bits: 0 },
            };
        }

        if items.len() == 1 {
            // SAFETY: Unwrap guarded by above call to len
            let item = GreenTriviaRepr {
                inline: items.next().unwrap(),
            };

            // SAFETY: This turns a TriviaPiece into a u64, this is safe since
            // that struct only stores plain integers and enums
            unsafe {
                debug_assert_eq!(
                    item.bits & 1,
                    1,
                    "unexpected bit pattern for TriviaPiece: {:0>64b}",
                    item.bits,
                );
            }

            return GreenTrivia { inner: item };
        }

        let data = GreenTriviaRepr {
            ptr: ManuallyDrop::new(ThinArc::from_header_and_iter(
                GreenTriviaHead { _c: Count::new() },
                items,
            )),
        };

        // SAFETY: this turns a ThinArc into a u64, this is safe since ThinArc
        // in turn only contains a NonNull that get reinterpreted as a
        // pointer-sized unsigned integer
        unsafe {
            debug_assert_eq!(
                data.bits & 1,
                0,
                "unexpected bit pattern for ThinArc: {:0>64b}",
                data.bits,
            );
        }

        GreenTrivia { inner: data }
    }

    /// Returns the total length of all pieces
    pub fn text_len(&self) -> TextSize {
        let mut len: Option<TextSize> = Some(TextSize::default());

        for piece in self.pieces() {
            len = len.and_then(|len| len.checked_add(piece.length))
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
            if self.inner.bits == 0 {
                return 0;
            }

            if (self.inner.bits & 1) == 1 {
                return 1;
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
            if self.inner.bits == 0 {
                return &[];
            }

            if (self.inner.bits & 1) == 1 {
                return slice::from_ref(&self.inner.inline);
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
            if self.inner.bits == 0 || (self.inner.bits & 1) == 1 {
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

    #[test]
    fn trivia_piece_layout() {
        assert_eq!(
            std::mem::size_of::<u64>(),
            std::mem::size_of::<TriviaPiece>()
        );

        let piece = TriviaPiece {
            kind: TriviaPieceKind::Newline,
            length: TextSize::from(0),
        };

        let ptr = &piece as *const TriviaPiece as *const u8;
        let kind = &piece.kind as *const TriviaPieceKind as *const u8;
        let length = &piece.length as *const TextSize as *const u8;

        let kind_offset = unsafe { kind.offset_from(ptr) };
        let length_offset = unsafe { length.offset_from(ptr) };

        assert_eq!(kind_offset, 0);
        assert_eq!(length_offset, 4);

        let bits: u64 = unsafe { std::mem::transmute(piece) };
        assert_eq!(
            bits & 1,
            1,
            "unexpected bit pattern for TriviaPiece: {:0>64b}",
            bits,
        );
    }
}
