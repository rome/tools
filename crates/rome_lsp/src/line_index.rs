//! `LineIndex` maps flat `TextSize` offsets into `(Line, Column)`
//! representation.
//!
//! Copied from rust-analyzer

use std::cmp::Ordering;

use rome_rowan::{TextRange, TextSize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LineIndex {
    text: String,
    /// Offset the the beginning of each line, zero-based
    newlines: Vec<TextSize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LineCol {
    /// Zero-based
    pub(crate) line: u32,
    /// Zero-based utf8 offset
    pub(crate) col: u32,
}

impl LineIndex {
    pub(crate) fn new(text: &str) -> LineIndex {
        let mut newlines = vec![0.into()];

        for (offset, c) in text.char_indices() {
            if c == '\n' {
                let char_offset = TextSize::try_from(offset).expect("TextSize overflow");
                newlines.push(char_offset + TextSize::of(c));
            }
        }

        LineIndex {
            text: text.into(),
            newlines,
        }
    }

    pub(crate) fn line_col(&self, offset: TextSize) -> Option<LineCol> {
        // Fast path for offset == 0
        if offset == TextSize::from(0) {
            return Some(LineCol { line: 0, col: 0 });
        }

        // Find the index of the line `offset` belongs to
        let line_index = self
            .newlines
            .partition_point(|&it| it <= offset)
            .checked_sub(1)?;

        // Calculate the text range corresponding to `line_index`
        let line_start_offset = self.newlines.get(line_index)?;
        let line_end_offset = self
            .newlines
            .get(line_index + 1)
            .cloned()
            .unwrap_or_else(|| TextSize::of(&self.text));

        let line_range = TextRange::new(*line_start_offset, line_end_offset);
        let line_text = &self.text[line_range];

        // Calculate the byte offset of the character within the line and find
        // a column matching this offset
        let char_offset = usize::from(offset - *line_start_offset);
        let char_index = match char_offset.cmp(&line_text.len()) {
            Ordering::Less => {
                line_text
                    .char_indices()
                    .enumerate()
                    .find_map(|(index, (offset, _))| {
                        if offset == char_offset {
                            Some(index)
                        } else {
                            None
                        }
                    })?
            }
            // If the character offset is equal to the length of the line, the
            // character index is the total number of columns in the line
            Ordering::Equal => line_text.chars().count(),
            // The character offset is greater than the length of the line,
            // abort since the provided offset is invalid
            Ordering::Greater => return None,
        };

        Some(LineCol {
            line: u32::try_from(line_index).ok()?,
            col: u32::try_from(char_index).ok()?,
        })
    }

    pub(crate) fn offset(&self, line_col: LineCol) -> Option<TextSize> {
        // Convert the line and column indices to usize (this should never fail
        // on 32- and 64-bits platforms)
        let line_index = usize::try_from(line_col.line).ok()?;
        let col_index = usize::try_from(line_col.col).ok()?;

        // Load the byte offset for the start of line `line_index`
        let line_offset = self.newlines.get(line_index)?;
        let col_offset = if col_index > 0 {
            // Calculate the text range corresponding to `line_index`
            let line_start = usize::from(*line_offset);
            let line_end = self
                .newlines
                .get(line_index + 1)
                .map(|offset| usize::from(*offset))
                .unwrap_or_else(|| self.text.len());

            let line_text = self.text.get(line_start..line_end)?;
            let num_chars = line_text.chars().count();

            // If the column index is equal to the number of characters in the
            // line, return the byte offset for the end of the line
            let col_offset = if col_index == num_chars {
                line_text.len()
            } else {
                // Accumulate byte offsets for each character in the line and
                // return the value corresponding to `col_index`
                let (col_offset, _) = line_text.char_indices().nth(col_index)?;
                col_offset
            };

            TextSize::try_from(col_offset).ok()?
        } else {
            // Fast path for col == 0
            TextSize::from(0)
        };

        Some(*line_offset + col_offset)
    }

    /// Return the text slice used to build the index
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    /// Return the number of lines in the index, clamped to [u32::MAX]
    pub(crate) fn len(&self) -> u32 {
        self.newlines.len().try_into().unwrap_or(u32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use rome_rowan::TextSize;

    use super::{LineCol, LineIndex};

    macro_rules! check_conversion {
        ($line_index:ident : $line_col:expr => $text_size:expr ) => {
            let offset = $line_index.offset($line_col);
            assert_eq!(offset, Some($text_size));

            let line_col = $line_index.line_col($text_size);
            assert_eq!(line_col, Some($line_col));
        };
    }

    #[test]
    fn empty_string() {
        let line_index = LineIndex::new("");
        check_conversion!(line_index: LineCol { line: 0, col: 0 } => TextSize::from(0));
    }

    #[test]
    fn empty_line() {
        let line_index = LineIndex::new("\n\n");
        check_conversion!(line_index: LineCol { line: 1, col: 0 } => TextSize::from(1));
    }

    #[test]
    fn line_end() {
        let line_index = LineIndex::new("abc\ndef\nghi");
        check_conversion!(line_index: LineCol { line: 1, col: 3 } => TextSize::from(7));
    }

    #[test]
    fn out_of_bounds_line() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.offset(LineCol { line: 5, col: 0 });
        assert!(offset.is_none());
    }

    #[test]
    fn out_of_bounds_col() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.offset(LineCol { line: 1, col: 7 });
        assert!(offset.is_none());
    }

    #[test]
    fn out_of_bounds_offset() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.line_col(TextSize::from(13));
        assert!(offset.is_none());
    }

    #[test]
    fn unicode() {
        let line_index = LineIndex::new("'Jan 1, 2018 – Jan 1, 2019'");

        check_conversion!(line_index: LineCol { line: 0, col: 0 } => TextSize::from(0));
        check_conversion!(line_index: LineCol { line: 0, col: 1 } => TextSize::from(1));
        check_conversion!(line_index: LineCol { line: 0, col: 12 } => TextSize::from(12));
        check_conversion!(line_index: LineCol { line: 0, col: 13 } => TextSize::from(15));
        check_conversion!(line_index: LineCol { line: 0, col: 14 } => TextSize::from(18));
        check_conversion!(line_index: LineCol { line: 0, col: 15 } => TextSize::from(21));
        check_conversion!(line_index: LineCol { line: 0, col: 26 } => TextSize::from(32));
        check_conversion!(line_index: LineCol { line: 0, col: 27 } => TextSize::from(33));
    }
}
