pub mod rename;
#[cfg(test)]
pub mod tests;

#[macro_export]
macro_rules! assert_rename_ok {
    ($(#[$attr:meta])* $($name:ident, $before:expr, $expected:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_rename_ok($before, $expected);
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_rename_nok {
    ($(#[$attr:meta])* $($name:ident, $before:expr,)*) => {
        $(
            #[test]
            pub fn $name() {
                $crate::utils::tests::assert_rename_nok($before);
            }
        )*
    };
}

#[derive(Debug, PartialEq)]
pub(crate) enum EscapeError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> Iterator for InterpretEscapedString<'a> {
    type Item = Result<char, EscapeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(EscapeError::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some(c) => Err(EscapeError::InvalidEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}

/// unescape   
///
pub(crate) fn escape_string(s: &str) -> Result<String, EscapeError> {
    (InterpretEscapedString { s: s.chars() }).collect()
}
