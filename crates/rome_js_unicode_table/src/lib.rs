use crate::bytes::DISPATCHER;
use crate::tables::derived_property::{ID_Continue, ID_Start};

mod bytes;
mod tables;

pub use crate::bytes::Dispatch;

/// Tests if `c` is a valid start of an identifier
#[inline]
pub fn is_id_start(c: char) -> bool {
    c == '_' || c == '$' || ID_Start(c)
}

/// Tests if `c` is a valid continuation of an identifier.
#[inline]
pub fn is_id_continue(c: char) -> bool {
    c == '$' || c == '\u{200d}' || c == '\u{200c}' || ID_Continue(c)
}

/// Looks up a byte in the lookup table.
#[inline]
pub fn lookup_byte(byte: u8) -> Dispatch {
    // Safety: the lookup table maps all values of u8, so it's impossible for a u8 to be out of bounds
    unsafe { *DISPATCHER.get_unchecked(byte as usize) }
}
