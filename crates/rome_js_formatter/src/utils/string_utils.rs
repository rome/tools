use std::borrow::Cow;

pub trait ToAsciiLowercaseCow {
    /// Returns the same value as String::to_lowercase. The only difference
    /// is that this functions returns ```Cow``` and does not allocate
    /// if the string is already in lowercase.
    fn to_ascii_lowercase_cow(&self) -> Cow<str>;
}

impl ToAsciiLowercaseCow for str {
    fn to_ascii_lowercase_cow(&self) -> Cow<str> {
        debug_assert!(self.is_ascii());

        let bytes = self.as_bytes();

        for idx in 0..bytes.len() {
            let chr = bytes[idx];
            if chr != chr.to_ascii_lowercase() {
                let mut s = bytes.to_vec();
                for b in &mut s[idx..] {
                    b.make_ascii_lowercase();
                }
                return Cow::Owned(unsafe { String::from_utf8_unchecked(s) });
            }
        }

        Cow::Borrowed(self)
    }
}

impl ToAsciiLowercaseCow for String {
    #[inline(always)]
    fn to_ascii_lowercase_cow(&self) -> Cow<str> {
        self.as_str().to_ascii_lowercase_cow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::quickcheck_utils::*;
    use quickcheck_macros::*;

    #[quickcheck]
    fn to_ascii_lowercase_cow_always_returns_same_value_as_string_to_lowercase(txt: AsciiString) {
        assert_eq!(
            txt.to_lowercase(),
            txt.to_ascii_lowercase_cow().into_owned()
        );
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_borrowed_when_all_chars_are_lowercase(txt: AsciiString) {
        let txt = txt.to_lowercase();
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Borrowed(s) if s == txt));
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_owned_when_some_chars_are_not_lowercase(txt: AsciiString) {
        let txt = format!("{}A", txt); //guarantees at least one uppercase letter
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Owned(s) if s == txt.to_lowercase()));
    }
}
