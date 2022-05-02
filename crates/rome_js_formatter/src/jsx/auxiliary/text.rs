use crate::{FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::{FormatResult, Token};
use rome_js_syntax::{JsxText, JsxTextFields};
use std::borrow::Cow;
use std::ops::Range;
use std::str::CharIndices;

impl FormatNode for JsxText {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTextFields { value_token } = self.as_fields();
        let token = value_token?;
        let new_text = clean_jsx_text(token.text());
        let start = token.text_range().start();
        let new_token = Token::from_syntax_token_cow_slice(new_text, &token, start);

        Ok(formatter.format_replaced(&token, FormatElement::from(new_token)))
    }
}

static TERMINATORS: [char; 4] = [' ', '\n', '\t', '\r'];

struct TextInfo<'a> {
    pub text: &'a str,
    pub leading_whitespace_type: Option<WhitespaceType>,
    /// Whitespace ranges are the ranges of text that contain whitespace. We keep track of them
    /// so that on our second pass, we strip them out.
    pub whitespace_ranges: Vec<Range<usize>>,
    pub trailing_whitespace_type: Option<WhitespaceType>,
}

impl<'a> TextInfo<'a> {
    fn build(text: &'a str) -> Self {
        let mut char_indices = text.char_indices();

        // Once `get_leading_whitespace_type` is done, we have consumed our first non-whitespace character
        let leading_whitespace_type = get_leading_whitespace_type(&mut char_indices);

        let mut whitespace_ranges = Vec::new();
        let mut current_whitespace_range_start: Option<usize> = None;

        for (idx, c) in char_indices {
            // If we've already started a whitespace range...
            if let Some(start) = current_whitespace_range_start {
                // If the character is *not* a whitespace character...
                if !TERMINATORS.contains(&c) {
                    // We push the range into the vector
                    whitespace_ranges.push(start..idx);
                    current_whitespace_range_start = None;
                }
            } else {
                // If we have not started a whitespace range
                // and we come across a whitespace character,
                if TERMINATORS.contains(&c) {
                    // We start a whitespace range
                    current_whitespace_range_start = Some(idx);
                }
            }
        }

        let trailing_whitespace_type = current_whitespace_range_start
            .and_then(|start| get_trailing_whitespace_type(&text[start..]));

        TextInfo {
            text,
            leading_whitespace_type,
            whitespace_ranges,
            trailing_whitespace_type,
        }
    }

    fn create_normalized_text(&self) -> Option<String> {
        if self.whitespace_ranges.is_empty() {
            None
        } else {
            let mut char_indices = self.text.char_indices();

            let mut output_string = match self.leading_whitespace_type {
                None | Some(WhitespaceType::HasNewline) => String::new(),
                Some(WhitespaceType::NoNewline) => " ".to_string(),
            };

            if matches!(
                self.leading_whitespace_type,
                Some(WhitespaceType::HasNewline) | Some(WhitespaceType::NoNewline)
            ) {
                while let Some((_, c)) = char_indices.next() {
                    if !TERMINATORS.contains(&c) {
                        output_string.push(c);
                        break;
                    }
                }
            }

            let mut current_whitespace_range_idx = 0;

            // Invariant: idx is **never** larger than the end of the current whitespace range
            for (idx, c) in char_indices {
                let current_whitespace_range =
                    self.whitespace_ranges.get(current_whitespace_range_idx);
                // If the index is the end of the current whitespace range,
                // then we increment the whitespace range index and
                // push on an empty string
                if let Some(range) = current_whitespace_range {
                    if idx == range.end {
                        output_string.push(' ');
                        output_string.push(c);
                        current_whitespace_range_idx = current_whitespace_range_idx + 1;
                    }

                    // If our index is less than the start of the current whitespace range
                    // we push on characters
                    if idx < range.start {
                        output_string.push(c)
                    }
                } else if !TERMINATORS.contains(&c) {
                    output_string.push(c)
                }
            }

            if matches!(
                self.trailing_whitespace_type,
                Some(WhitespaceType::NoNewline)
            ) {
                output_string.push(' ');
            }

            Some(output_string)
        }
    }
}

fn get_trimmed_text(
    text: &str,
    leading_whitespace_type: Option<WhitespaceType>,
    trailing_whitespace_type: Option<WhitespaceType>,
) -> Cow<str> {
    match (leading_whitespace_type, trailing_whitespace_type) {
        (Some(WhitespaceType::HasNewline), Some(WhitespaceType::HasNewline)) => {
            Cow::Borrowed(text.trim())
        }
        (None, None) => Cow::Borrowed(text),
        (Some(WhitespaceType::HasNewline), None) => Cow::Borrowed(text.trim_start()),
        (None, Some(WhitespaceType::HasNewline)) => Cow::Borrowed(text.trim_end()),
        (Some(WhitespaceType::NoNewline), Some(WhitespaceType::NoNewline)) => {
            Cow::Owned(format!(" {} ", text.trim()))
        }
        (Some(WhitespaceType::NoNewline), _) => Cow::Owned(format!(" {}", text.trim())),
        (_, Some(WhitespaceType::NoNewline)) => Cow::Owned(format!("{} ", text.trim())),
    }
}

/// Leading and trailing whitespace can either have newlines or not
/// If whitespace has newlines, we normalize it to no spaces.
/// If whitespace has no newlines, we normaliez it to a single space
#[derive(Debug, Copy, Clone)]
enum WhitespaceType {
    NoNewline,
    HasNewline,
}

/// We push the CharIndices iterator forward until we get to a non-whitespace character
fn get_leading_whitespace_type(char_indices: &mut CharIndices) -> Option<WhitespaceType> {
    let mut leading_type = None;

    while let Some((_, c)) = char_indices.next() {
        if !TERMINATORS.contains(&c) {
            return leading_type;
        }
        if c == '\n' {
            leading_type = Some(WhitespaceType::HasNewline);
        } else if leading_type.is_none() {
            leading_type = Some(WhitespaceType::NoNewline);
        }
    }

    leading_type
}

fn get_trailing_whitespace_type(end_whitespace: &str) -> Option<WhitespaceType> {
    let mut trailing_type = None;
    for c in end_whitespace.chars() {
        if c.is_whitespace() {
            if c == '\n' {
                trailing_type = Some(WhitespaceType::HasNewline);
            } else if trailing_type.is_none() {
                trailing_type = Some(WhitespaceType::NoNewline);
            }
        }
    }

    trailing_type
}

fn clean_jsx_text(text: &str) -> Cow<str> {
    if text.is_empty() {
        return Cow::Borrowed(text);
    }

    let text_info = TextInfo::build(text);

    if let Some(normalized_text) = text_info.create_normalized_text() {
        Cow::Owned(normalized_text)
    } else {
        get_trimmed_text(
            text_info.text,
            text_info.leading_whitespace_type,
            text_info.trailing_whitespace_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::jsx::auxiliary::text::clean_jsx_text;

    #[test]
    fn clean_jsx_text_works() {
        assert_eq!("", clean_jsx_text(""));
        assert_eq!(" ", clean_jsx_text(" "));
        assert_eq!("Foo", clean_jsx_text("Foo"));
        assert_eq!(" Foo", clean_jsx_text(" Foo"));
        assert_eq!("Foo", clean_jsx_text("\nFoo"));
        assert_eq!(" Foo", clean_jsx_text("\tFoo"));
        assert_eq!("Foo", clean_jsx_text("\n \t Foo"));
        assert_eq!("Foo", clean_jsx_text("\n \t \n \t\nFoo"));
        assert_eq!(" Foo bar lorem", clean_jsx_text(" Foo bar lorem"));
        assert_eq!("Foo ", clean_jsx_text("Foo "));
        assert_eq!("Foo", clean_jsx_text("Foo\n"));
        assert_eq!("Foo ", clean_jsx_text("Foo\t"));
        assert_eq!("Foo", clean_jsx_text("Foo\t \n "));
        assert_eq!("Foo", clean_jsx_text("Foo\n \t \n \t\n"));
        assert_eq!("Foo Bar", clean_jsx_text("Foo\n \t\t\n \tBar"));
        assert_eq!(
            "Foo Bar",
            clean_jsx_text("\n \t\t\n \tFoo\n \t\t\n \tBar\n \t\t\n \t")
        );
    }
}
