use crate::prelude::*;
use crate::utils::jsx::{JsxSpace, JSX_WHITESPACE_CHARS};
use rome_formatter::{write, FormatResult};
use rome_js_syntax::{JsxText, JsxTextFields, TextSize};
use std::borrow::Cow;
use std::ops::Range;
use std::str::CharIndices;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxText;

impl FormatNodeRule<JsxText> for FormatJsxText {
    fn fmt_fields(&self, node: &JsxText, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxTextFields { value_token } = node.as_fields();
        let token = value_token?;
        let (leading_whitespace_type, new_text, start, trailing_whitespace_type) =
            clean_jsx_text(token.text(), token.text_range().start());
        if matches!(
            leading_whitespace_type,
            Some(WhitespaceType::HasNewline) | None
        ) && new_text.is_empty()
            && matches!(
                trailing_whitespace_type,
                Some(WhitespaceType::HasNewline) | None
            )
        {
            return write![f, [format_removed(&token)]];
        }

        let new_token = syntax_token_cow_slice(new_text, &token, start);
        let new_text = format_replaced(&token, &new_token);

        write![
            f,
            [leading_whitespace_type, new_text, trailing_whitespace_type]
        ]
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
                if !JSX_WHITESPACE_CHARS.contains(&c) {
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
                if JSX_WHITESPACE_CHARS.contains(&c) {
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
    /// Does *not* add leading or trailing whitespace. Leading or trailing whitespace must be a JSX
    /// space.
    fn clean_text(&self) -> Option<String> {
        if self.whitespace_ranges.is_empty() {
            return None;
        }

        let mut char_indices = self.text.char_indices();

        let mut output_string = String::new();

        if self.leading_whitespace_type.is_some() {
            for (_, c) in char_indices.by_ref() {
                if !JSX_WHITESPACE_CHARS.contains(&c) {
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
                if !JSX_WHITESPACE_CHARS.contains(&c) {
                    output_string.push(c)
                }
            }
        }

        Some(output_string)
    }
}

impl Format<JsFormatContext> for WhitespaceType {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if self == &WhitespaceType::NoNewline {
            write![f, [JsxSpace::default()]]?;
        }

        Ok(())
    }
}

/// Leading and trailing whitespace can either have newlines or not
/// If whitespace has newlines, we normalize it to no spaces.
/// If whitespace has no newlines, we normalize it to a single space
#[derive(Debug, Copy, Clone, PartialEq)]
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
        if !JSX_WHITESPACE_CHARS.contains(&c) {
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
        if JSX_WHITESPACE_CHARS.contains(&c) {
            if c == '\n' {
                trailing_type = Some(WhitespaceType::HasNewline);
            } else if trailing_type.is_none() {
                trailing_type = Some(WhitespaceType::NoNewline);
            }
        }
    }

    trailing_type
}

fn clean_jsx_text(
    text: &str,
    text_start: TextSize,
) -> (
    Option<WhitespaceType>,
    Cow<str>,
    TextSize,
    Option<WhitespaceType>,
) {
    if text.is_empty() {
        return (None, Cow::Borrowed(text), text_start, None);
    }

    let text_cleaner = TextCleaner::build(text);

    let cleaned_text = if let Some(normalized_text) = text_cleaner.clean_text() {
        Cow::Owned(normalized_text)
    } else {
        Cow::Borrowed(text_cleaner.text.trim_matches(&JSX_WHITESPACE_CHARS[..]))
    };

    let start_idx: TextSize = text_cleaner
        .start_idx
        .try_into()
        .expect("index is larger than 2^32 bits");

    (
        text_cleaner.leading_whitespace_type,
        cleaned_text,
        text_start + start_idx,
        text_cleaner.trailing_whitespace_type,
    )
}

#[cfg(test)]
mod tests {
    use crate::jsx::auxiliary::text::{clean_jsx_text, WhitespaceType};
    use std::borrow::Cow;

    #[test]
    fn clean_jsx_text_works() {
        assert_eq!(
            (None, Cow::Borrowed(""), 0.into(), None),
            clean_jsx_text("", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::NoNewline),
                Cow::Borrowed(""),
                1.into(),
                None
            ),
            clean_jsx_text(" ", 0.into())
        );
        assert_eq!(
            (None, Cow::Borrowed("Foo"), 0.into(), None),
            clean_jsx_text("Foo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::NoNewline),
                Cow::Borrowed("Foo"),
                1.into(),
                None
            ),
            clean_jsx_text(" Foo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::HasNewline),
                Cow::Borrowed("Foo"),
                1.into(),
                None
            ),
            clean_jsx_text("\nFoo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::NoNewline),
                Cow::Borrowed("Foo"),
                1.into(),
                None
            ),
            clean_jsx_text("\tFoo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::HasNewline),
                Cow::Borrowed("Foo"),
                4.into(),
                None
            ),
            clean_jsx_text("\n \t Foo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::HasNewline),
                Cow::Borrowed("Foo"),
                8.into(),
                None
            ),
            clean_jsx_text("\n \t \n \t\nFoo", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::NoNewline),
                Cow::Borrowed("Foo bar lorem"),
                1.into(),
                None
            ),
            clean_jsx_text(" Foo bar lorem", 0.into())
        );
        assert_eq!(
            (
                None,
                Cow::Borrowed("Foo"),
                0.into(),
                Some(WhitespaceType::NoNewline)
            ),
            clean_jsx_text("Foo ", 0.into())
        );
        assert_eq!(
            (
                None,
                Cow::Borrowed("Foo"),
                0.into(),
                Some(WhitespaceType::HasNewline)
            ),
            clean_jsx_text("Foo\n", 0.into())
        );
        assert_eq!(
            (
                None,
                Cow::Borrowed("Foo"),
                0.into(),
                Some(WhitespaceType::NoNewline)
            ),
            clean_jsx_text("Foo\t", 0.into())
        );
        assert_eq!(
            (
                None,
                Cow::Borrowed("Foo"),
                0.into(),
                Some(WhitespaceType::HasNewline)
            ),
            clean_jsx_text("Foo\t \n ", 0.into())
        );
        assert_eq!(
            (
                None,
                Cow::Borrowed("Foo"),
                0.into(),
                Some(WhitespaceType::HasNewline)
            ),
            clean_jsx_text("Foo\n \t \n \t\n", 0.into())
        );
        assert_eq!(
            (None, Cow::Owned("Foo Bar".to_string()), 0.into(), None),
            clean_jsx_text("Foo\n \t\t\n \tBar", 0.into())
        );
        assert_eq!(
            (
                Some(WhitespaceType::HasNewline),
                Cow::Owned("Foo Bar".to_string()),
                7.into(),
                Some(WhitespaceType::HasNewline)
            ),
            clean_jsx_text("\n \t\t\n \tFoo\n \t\t\n \tBar\n \t\t\n \t", 0.into())
        );
    }
}
