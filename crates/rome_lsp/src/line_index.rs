//! `LineIndex` maps flat `TextSize` offsets into `(Line, Column)`
//! representation.
//!
//! Copied from rust-analyzer

use rome_rowan::TextSize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LineIndex {
    /// Offset the the beginning of each line, zero-based
    pub(crate) newlines: Vec<TextSize>,
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
        let mut curr_row = 0.into();

        for c in text.chars() {
            let c_len = TextSize::of(c);
            curr_row += c_len;

            if c == '\n' {
                newlines.push(curr_row);
            }
        }

        LineIndex { newlines }
    }

    pub(crate) fn line_col(&self, offset: TextSize) -> LineCol {
        let line = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines[line];
        let col = offset - line_start_offset;
        LineCol {
            line: line as u32,
            col: col.into(),
        }
    }

    pub(crate) fn offset(&self, line_col: LineCol) -> Option<TextSize> {
        let line_index = usize::try_from(line_col.line).ok()?;
        let line_offset = self.newlines.get(line_index)?;
        Some(*line_offset + TextSize::from(line_col.col))
    }
}

#[cfg(test)]
mod tests {
    use super::{LineCol, LineIndex};

    #[test]
    fn out_of_bounds() {
        let line_index = LineIndex::new("abcde\nfghij\n");
        let offset = line_index.offset(LineCol { line: 5, col: 0 });
        assert!(offset.is_none());
    }
}
