use crate::prelude::*;
use crate::utils::jsx_utils::WHITESPACE;
use crate::FormatNodeFields;
use crate::{FormatElement, Formatter, JsFormatter};
use rome_formatter::{FormatResult, Token};
use rome_js_syntax::{JsxText, JsxTextFields, TextSize};
use std::borrow::Cow;
use std::ops::Range;
use std::str::CharIndices;

impl FormatNodeFields<JsxText> for FormatNodeRule<JsxText> {
    fn format_fields(
        node: &JsxText,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsxTextFields { value_token } = node.as_fields();
        let token = value_token?;
        let (new_text, start) = clean_jsx_text(token.text(), token.text_range().start());
        let new_token = Token::from_syntax_token_cow_slice(new_text, &token, start);

        Ok(formatter.format_replaced(&token, FormatElement::from(new_token)))
    }
}

struct TextCleaner<'a> {
    pub text: &'a str,
    pub leading_whitespace_type: Option<WhitespaceType>,
    pub start_idx: usize,
    /// Whitespace ranges are the ranges of text that contain whitespace. We keep track of them
    /// so that on our second pass, we strip them out.
    ///
    ///  "A  Brighter \n\t Summer  \n\n Day"
    ///    ^^        ^^^^^^      ^^^^^^^
    pub whitespace_ranges: Vec<Range<usize>>,
    pub trailing_whitespace_type: Option<WhitespaceType>,
}

impl<'a> TextCleaner<'a> {
    fn build(text: &'a str) -> Self {
        let mut char_indices = text.char_indices();

        // Once `get_leading_whitespace_type` is done, we have consumed our first non-whitespace character
        let (leading_whitespace_type, start_idx) = get_leading_whitespace_type(&mut char_indices);

        let mut whitespace_ranges = Vec::new();
        let mut current_whitespace_range_start: Option<usize> = None;

        for (idx, c) in char_indices {
            // If we've already started a whitespace range...
            if let Some(start) = current_whitespace_range_start {
                // If the character is *not* a whitespace character...
                //
                //  input:  "Yi  Yi"
                //               ^
                if !WHITESPACE.contains(&c) {
                    // We push the range into the vector
                    whitespace_ranges.push(start..idx);
                    current_whitespace_range_start = None;
                }
            } else {
                // If we have not started a whitespace range
                // and we come across a whitespace character,
                //
                //  input: "Yi   Yi"
                //            ^
                if WHITESPACE.contains(&c) {
                    // We start a whitespace range
                    current_whitespace_range_start = Some(idx);
                }
            }
        }

        // If, at the end of the loop, we still have a `current_whitespace_range_start` that is
        // Some, this indicates we have trailing whitespace:
        //
        //  input: "Taipei  Story   \t"
        //                       ^ started unterminated whitespace range here
        //
        let trailing_whitespace_type = current_whitespace_range_start
            .and_then(|start| get_trailing_whitespace_type(&text[start..]));

        TextCleaner {
            text,
            start_idx,
            leading_whitespace_type,
            whitespace_ranges,
            trailing_whitespace_type,
        }
    }

    /// Tries to clean the text with the whitespace ranges. If we have no ranges, we return None
    /// because there's no cleaning to be done.
    fn clean_text(&self) -> Option<String> {
        if self.whitespace_ranges.is_empty() {
            return None;
        }

        let mut char_indices = self.text.char_indices();

        let mut output_string = match self.leading_whitespace_type {
            None | Some(WhitespaceType::HasNewline) => String::new(),
            Some(WhitespaceType::NoNewline) => " ".to_string(),
        };

        if self.leading_whitespace_type.is_some() {
            for (_, c) in char_indices.by_ref() {
                if !WHITESPACE.contains(&c) {
                    output_string.push(c);
                    break;
                }
            }
        }

        let mut current_whitespace_range_idx = 0;

        // Invariant: idx is **never** larger than the end of the current whitespace range
        for (idx, c) in char_indices {
            let current_whitespace_range = self.whitespace_ranges.get(current_whitespace_range_idx);
            if let Some(range) = current_whitespace_range {
                // If the index is the end of the current whitespace range,
                // then we increment the whitespace range index and
                // push on a space character.
                //
                //   input:  "hello    world"
                //                    ^
                //   output: "hello "
                if idx == range.end - 1 {
                    output_string.push(' ');
                    current_whitespace_range_idx += 1;
                }

                // If our index is less than the start of the current whitespace range
                // we push on characters.
                //
                //   input: "hello  world"
                //             ^
                //   output: "hel"
                //
                if idx < range.start {
                    output_string.push(c)
                }
            } else {
                // If None, we are past the whitespace ranges
                //
                //   input: "hello  world"
                //                    ^
                //   output: "hello wor"
                //
                // If the character is not whitespace, we push it on.
                // If it is whitespace, it is trailing whitespace, so we ignore it.
                if !WHITESPACE.contains(&c) {
                    output_string.push(c)
                }
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

/// We cannot have this inside the TextCleaner because the text reference
/// cannot be moved out of the struct, which it is via the Cow<str>
fn get_trimmed_text(
    text: &str,
    leading_whitespace_type: Option<WhitespaceType>,
    trailing_whitespace_type: Option<WhitespaceType>,
) -> Cow<str> {
    match (leading_whitespace_type, trailing_whitespace_type) {
        (Some(WhitespaceType::HasNewline), Some(WhitespaceType::HasNewline)) => {
            Cow::Borrowed(text.trim_matches(&WHITESPACE[..]))
        }
        (None, None) => Cow::Borrowed(text),
        (Some(WhitespaceType::HasNewline), None) => {
            Cow::Borrowed(text.trim_start_matches(&WHITESPACE[..]))
        }
        (None, Some(WhitespaceType::HasNewline)) => {
            Cow::Borrowed(text.trim_end_matches(&WHITESPACE[..]))
        }
        (Some(WhitespaceType::NoNewline), Some(WhitespaceType::NoNewline)) => {
            Cow::Owned(format!(" {} ", text.trim_matches(&WHITESPACE[..])))
        }
        (Some(WhitespaceType::NoNewline), _) => {
            Cow::Owned(format!(" {}", text.trim_matches(&WHITESPACE[..])))
        }
        (_, Some(WhitespaceType::NoNewline)) => {
            Cow::Owned(format!("{} ", text.trim_matches(&WHITESPACE[..])))
        }
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
///
/// Returns the whitespace type (if whitespace exists), ond the start index of the non-whitespace
/// text
///
/// NOTE: It's okay that we consume this non-whitespace character, as it won't affect our
///       whitespace group finding logic.
fn get_leading_whitespace_type(char_indices: &mut CharIndices) -> (Option<WhitespaceType>, usize) {
    let mut leading_type = None;
    let mut start_idx = 0;

    for (i, c) in char_indices.by_ref() {
        if !WHITESPACE.contains(&c) {
            return (leading_type, i);
        }
        start_idx = i;
        if c == '\n' {
            leading_type = Some(WhitespaceType::HasNewline);
        } else if leading_type.is_none() {
            leading_type = Some(WhitespaceType::NoNewline);
        }
    }

    (leading_type, start_idx + 1)
}

/// Get the whitespace type for the trailing whitespace.
/// This uses a slice instead of an iterator because we cannot know what is the trailing
/// whitespace a priori.
fn get_trailing_whitespace_type(end_whitespace: &str) -> Option<WhitespaceType> {
    let mut trailing_type = None;
    for c in end_whitespace.chars() {
        if WHITESPACE.contains(&c) {
            if c == '\n' {
                trailing_type = Some(WhitespaceType::HasNewline);
            } else if trailing_type.is_none() {
                trailing_type = Some(WhitespaceType::NoNewline);
            }
        }
    }

    trailing_type
}

fn clean_jsx_text(text: &str, text_start: TextSize) -> (Cow<str>, TextSize) {
    if text.is_empty() {
        return (Cow::Borrowed(text), text_start);
    }

    let text_cleaner = TextCleaner::build(text);

    let cleaned_text = if let Some(normalized_text) = text_cleaner.clean_text() {
        Cow::Owned(normalized_text)
    } else {
        get_trimmed_text(
            text_cleaner.text,
            text_cleaner.leading_whitespace_type,
            text_cleaner.trailing_whitespace_type,
        )
    };

    let start_idx: TextSize = text_cleaner
        .start_idx
        .try_into()
        .expect("index is larger than 2^32 bits");

    (cleaned_text, text_start + start_idx)
}

#[cfg(test)]
mod tests {
    use crate::jsx::auxiliary::text::clean_jsx_text;
    use std::borrow::Cow;

    #[test]
    fn clean_jsx_text_works() {
        assert_eq!((Cow::Borrowed(""), 0.into()), clean_jsx_text("", 0.into()));
        assert_eq!(
            (Cow::Borrowed(" "), 1.into()),
            clean_jsx_text(" ", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 0.into()),
            clean_jsx_text("Foo", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed(" Foo"), 1.into()),
            clean_jsx_text(" Foo", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 1.into()),
            clean_jsx_text("\nFoo", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed(" Foo"), 1.into()),
            clean_jsx_text("\tFoo", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 4.into()),
            clean_jsx_text("\n \t Foo", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 8.into()),
            clean_jsx_text("\n \t \n \t\nFoo", 0.into())
        );
        assert_eq!(
            (Cow::Owned(" Foo bar lorem".to_string()), 1.into()),
            clean_jsx_text(" Foo bar lorem", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo "), 0.into()),
            clean_jsx_text("Foo ", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 0.into()),
            clean_jsx_text("Foo\n", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo "), 0.into()),
            clean_jsx_text("Foo\t", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 0.into()),
            clean_jsx_text("Foo\t \n ", 0.into())
        );
        assert_eq!(
            (Cow::Borrowed("Foo"), 0.into()),
            clean_jsx_text("Foo\n \t \n \t\n", 0.into())
        );
        assert_eq!(
            (Cow::Owned("Foo Bar".to_string()), 0.into()),
            clean_jsx_text("Foo\n \t\t\n \tBar", 0.into())
        );
        assert_eq!(
            (Cow::Owned("Foo Bar".to_string()), 7.into()),
            clean_jsx_text("\n \t\t\n \tFoo\n \t\t\n \tBar\n \t\t\n \t", 0.into())
        );
    }
}
