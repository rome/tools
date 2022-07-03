#[derive(Debug, PartialEq)]
pub enum MyError {
    EscapeAtEndOfString,
    InvalidEscapedChar(char),
}

struct InterpretEscapedString<'a> {
    s: std::str::Chars<'a>,
}

impl<'a> Iterator for InterpretEscapedString<'a> {
    type Item = Result<char, MyError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.s.next().map(|c| match c {
            '\\' => match self.s.next() {
                None => Err(MyError::EscapeAtEndOfString),
                Some('n') => Ok('\n'),
                Some('\\') => Ok('\\'),
                Some(c) => Err(MyError::InvalidEscapedChar(c)),
            },
            c => Ok(c),
        })
    }
}

/// unescape string  
pub fn interpret_escaped_string(s: &str) -> Result<String, MyError> {
    (InterpretEscapedString { s: s.chars() }).collect()
}
