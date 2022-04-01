///! This module if a fork of https://github.com/brendanzab/codespan,
/// adapted to use the `rome_console` markup for formatting
use std::collections::BTreeMap;
use std::io;
use std::ops::Range;

use crate::fmt::{Display, Formatter};
use crate::Markup;

use self::render::{MultiLabel, Renderer, SingleLabel};

mod render;

const START_CONTEXT_LINES: usize = 3;
const END_CONTEXT_LINES: usize = 1;

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Clone)]
pub struct Label<'diagnostic> {
    /// The style of the label.
    pub style: LabelStyle,
    /// The range in bytes we are going to include in the final snippet.
    pub range: Range<usize>,
    /// An optional message to provide some additional information for the
    /// underlined code. These should not include line breaks.
    pub message: Markup<'diagnostic>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum LabelStyle {
    /// Labels that describe the primary cause of a diagnostic.
    Primary,
    /// Labels that provide additional context for a diagnostic.
    Secondary,
}

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum Severity {
    /// An unexpected bug.
    Bug,
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
}

/// The 'location focus' of a source code snippet.
pub enum Locus {
    File {
        /// The user-facing name of the file.
        name: String,
    },
    FileLocation {
        /// The user-facing name of the file.
        name: String,
        /// The location.
        location: Location,
    },
}

/// A user-facing location in a source file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    /// The user-facing line number.
    pub line_number: usize,
    /// The user-facing column number.
    pub column_number: usize,
}

pub struct Codespan<'diagnostic> {
    severity: Severity,
    locus: Option<Locus>,
    source_file: &'diagnostic SourceFile<'diagnostic>,
    labeled_file: Option<LabeledFile<'diagnostic>>,
    outer_padding: usize,
}

impl<'diagnostic> Codespan<'diagnostic> {
    /// Create a new codespan from a slice of source text, an overall severity
    /// level and an optional "locus" to be displayed at the top
    pub fn new(
        source_file: &'diagnostic SourceFile<'diagnostic>,
        severity: Severity,
        locus: Option<Locus>,
    ) -> Self {
        Self {
            severity,
            locus,
            source_file,
            labeled_file: None,
            outer_padding: 0,
        }
    }

    /// Insert a new label into this codespan
    pub fn add_label(&mut self, label: Label<'diagnostic>) -> Result<(), OverflowError> {
        let start_line_index = self.source_file.line_index(label.range.start);
        let start_line_number = self.source_file.line_number(start_line_index);

        let start_line_range = self.source_file.line_range(start_line_index)?;
        let end_line_index = self.source_file.line_index(label.range.end);
        let end_line_number = self.source_file.line_number(end_line_index);

        let end_line_range = self.source_file.line_range(end_line_index)?;

        self.outer_padding = std::cmp::max(self.outer_padding, count_digits(start_line_number));
        self.outer_padding = std::cmp::max(self.outer_padding, count_digits(end_line_number));

        let labeled_file = match &mut self.labeled_file {
            Some(labeled_file) => {
                // other labezls already exist in this codespan
                if labeled_file.max_label_style > label.style
                    || (labeled_file.max_label_style == label.style
                        && labeled_file.start > label.range.start)
                {
                    // this label has a higher style or has the same style but starts earlier
                    labeled_file.start = label.range.start;
                    labeled_file.location = self.source_file.location(label.range.start)?;
                    labeled_file.max_label_style = label.style;
                }
                labeled_file
            }
            None => {
                // this is the first label inserted into this codespan
                self.labeled_file.get_or_insert(LabeledFile {
                    start: label.range.start,
                    location: self.source_file.location(label.range.start)?,
                    num_multi_labels: 0,
                    lines: BTreeMap::new(),
                    max_label_style: label.style,
                })
            }
        };

        if start_line_index == end_line_index {
            // Single line
            //
            // ```text
            // 2 │ (+ test "")
            //   │         ^^ expected `Int` but found `String`
            // ```
            let label_start = label.range.start - start_line_range.start;
            // Ensure that we print at least one caret, even when we
            // have a zero-length source range.
            let label_end = usize::max(label.range.end - start_line_range.start, label_start + 1);

            let line = labeled_file.get_or_insert_line(
                start_line_index,
                start_line_range,
                start_line_number,
            );

            // Ensure that the single line labels are lexicographically
            // sorted by the range of source code that they cover.
            let index = match line.single_labels.binary_search_by(|(_, range, _)| {
                // `Range<usize>` doesn't implement `Ord`, so convert to `(usize, usize)`
                // to piggyback off its lexicographic comparison implementation.
                (range.start, range.end).cmp(&(label_start, label_end))
            }) {
                // If the ranges are the same, order the labels in reverse
                // to how they were originally specified in the diagnostic.
                // This helps with printing in the renderer.
                Ok(index) | Err(index) => index,
            };

            line.single_labels
                .insert(index, (label.style, label_start..label_end, label.message));

            // If this line is not rendered, the SingleLabel is not visible.
            line.must_render = true;
        } else {
            // Multiple lines
            //
            // ```text
            // 4 │   fizz₁ num = case (mod num 5) (mod num 3) of
            //   │ ╭─────────────^
            // 5 │ │     0 0 => "FizzBuzz"
            // 6 │ │     0 _ => "Fizz"
            // 7 │ │     _ 0 => "Buzz"
            // 8 │ │     _ _ => num
            //   │ ╰──────────────^ `case` clauses have incompatible types
            // ```

            let label_index = labeled_file.num_multi_labels;
            labeled_file.num_multi_labels += 1;

            // First labeled line
            let label_start = label.range.start - start_line_range.start;

            let start_line = labeled_file.get_or_insert_line(
                start_line_index,
                start_line_range,
                start_line_number,
            );

            start_line
                .multi_labels
                .push((label_index, label.style, MultiLabel::Top(label_start)));

            // The first line has to be rendered so the start of the label is visible.
            start_line.must_render = true;

            // Marked lines
            //
            // ```text
            // 5 │ │     0 0 => "FizzBuzz"
            // 6 │ │     0 _ => "Fizz"
            // 7 │ │     _ 0 => "Buzz"
            // ```
            for line_index in (start_line_index + 1)..end_line_index {
                let line_range = self.source_file.line_range(line_index)?;
                let line_number = self.source_file.line_number(line_index);

                self.outer_padding = std::cmp::max(self.outer_padding, count_digits(line_number));

                let line = labeled_file.get_or_insert_line(line_index, line_range, line_number);

                line.multi_labels
                    .push((label_index, label.style, MultiLabel::Left));

                // The line should be rendered to match the configuration of how much context to show.
                line.must_render |=
                        // Is this line part of the context after the start of the label?
                        line_index - start_line_index <= START_CONTEXT_LINES
                        ||
                        // Is this line part of the context before the end of the label?
                        end_line_index - line_index <= END_CONTEXT_LINES;
            }

            // Last labeled line
            //
            // ```text
            // 8 │ │     _ _ => num
            //   │ ╰──────────────^ `case` clauses have incompatible types
            // ```
            let label_end = label.range.end - end_line_range.start;

            let end_line =
                labeled_file.get_or_insert_line(end_line_index, end_line_range, end_line_number);

            end_line.multi_labels.push((
                label_index,
                label.style,
                MultiLabel::Bottom(label_end, label.message),
            ));

            // The last line has to be rendered so the end of the label is visible.
            end_line.must_render = true;
        }

        Ok(())
    }
}

impl<'diagnostic> Display for Codespan<'diagnostic> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let mut renderer = Renderer::new(&mut *fmt);
        let file = match &self.labeled_file {
            Some(file) => file,
            None => return Ok(()),
        };

        // Top left border and locus.
        //
        // ```text
        // ┌─ test:2:9
        // ```
        if !file.lines.is_empty() {
            if let Some(locus) = &self.locus {
                renderer.render_snippet_start(self.outer_padding, locus)?;
            }

            renderer.render_snippet_empty(
                self.outer_padding,
                self.severity,
                file.num_multi_labels,
                &[],
            )?;
        }

        let mut lines = file
            .lines
            .iter()
            .filter(|(_, line)| line.must_render)
            .peekable();

        while let Some((line_index, line)) = lines.next() {
            renderer.render_snippet_source(
                self.outer_padding,
                line.number,
                line.range.clone(),
                self.source_file.source,
                self.severity,
                &line.single_labels,
                file.num_multi_labels,
                &line.multi_labels,
            )?;

            // Check to see if we need to render any intermediate stuff
            // before rendering the next line.
            if let Some((next_line_index, _)) = lines.peek() {
                match next_line_index.checked_sub(*line_index) {
                    // Consecutive lines
                    Some(1) => {}
                    // One line between the current line and the next line
                    Some(2) => {
                        // This line was not intended to be rendered initially.
                        // To render the line right, we have to get back the original labels.
                        let labels = file
                            .lines
                            .get(&(line_index + 1))
                            .map_or(&[][..], |line| &line.multi_labels[..]);

                        let line_number = self.source_file.line_number(line_index + 1);
                        let line_range = self
                            .source_file
                            .line_range(line_index + 1)
                            .map_err(|_| io::Error::new(io::ErrorKind::Other, "overflow error"))?;

                        renderer.render_snippet_source(
                            self.outer_padding,
                            line_number,
                            line_range.clone(),
                            self.source_file.source,
                            self.severity,
                            &[],
                            file.num_multi_labels,
                            labels,
                        )?;
                    }
                    // More than one line between the current line and the next line.
                    Some(_) | None => {
                        // Source break
                        //
                        // ```text
                        // ·
                        // ```
                        renderer.render_snippet_break(
                            self.outer_padding,
                            self.severity,
                            file.num_multi_labels,
                            &line.multi_labels,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// Error type returned when a label is inserted with a range that falls
/// outside of the source file
#[derive(Debug)]
pub struct OverflowError;

/// Representation of a single source file holding additional information for
/// efficiently rendering [Codespan]
pub struct SourceFile<'diagnostic> {
    /// The source code of the file.
    source: &'diagnostic str,
    /// The starting byte indices in the source code.
    line_starts: Vec<usize>,
}

impl<'diagnostic> SourceFile<'diagnostic> {
    /// Create a new [SourceFile] from a slice of text
    pub fn new(source: &'diagnostic str) -> Self {
        Self {
            source,
            line_starts: line_starts(source).collect(),
        }
    }

    /// Return the starting byte index of the line with the specified line index.
    /// Convenience method that already generates errors if necessary.
    fn line_start(&self, line_index: usize) -> Result<usize, OverflowError> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self
                .line_starts
                .get(line_index)
                .cloned()
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(OverflowError),
        }
    }

    fn line_index(&self, byte_index: usize) -> usize {
        self.line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1)
    }

    fn line_range(&self, line_index: usize) -> Result<Range<usize>, OverflowError> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }

    fn line_number(&self, line_index: usize) -> usize {
        line_index + 1
    }

    fn column_number(&self, line_index: usize, byte_index: usize) -> Result<usize, OverflowError> {
        let source = self.source;
        let line_range = self.line_range(line_index)?;
        let column_index = column_index(source, line_range, byte_index);

        Ok(column_index + 1)
    }

    fn location(&self, byte_index: usize) -> Result<Location, OverflowError> {
        let line_index = self.line_index(byte_index);

        Ok(Location {
            line_number: self.line_number(line_index),
            column_number: self.column_number(line_index, byte_index)?,
        })
    }
}

/// Return the starting byte index of each line in the source string.
///
/// This can make it easier to implement [`Files::line_index`] by allowing
/// implementors of [`Files`] to pre-compute the line starts, then search for
/// the corresponding line range, as shown in the example below.
///
/// [`Files`]: Files
/// [`Files::line_index`]: Files::line_index
fn line_starts(source: &'_ str) -> impl '_ + Iterator<Item = usize> {
    std::iter::once(0).chain(source.match_indices('\n').map(|(i, _)| i + 1))
}

/// The column index at the given byte index in the source file.
/// This is the number of characters to the given byte index.
///
/// If the byte index is smaller than the start of the line, then `0` is returned.
/// If the byte index is past the end of the line, the column index of the last
/// character `+ 1` is returned.
fn column_index(source: &str, line_range: Range<usize>, byte_index: usize) -> usize {
    let end_index = std::cmp::min(byte_index, std::cmp::min(line_range.end, source.len()));

    (line_range.start..end_index)
        .filter(|byte_index| source.is_char_boundary(byte_index + 1))
        .count()
}

/// Count the number of decimal digits in `n`.
fn count_digits(mut n: usize) -> usize {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n /= 10; // remove last digit
    }
    count
}

struct LabeledFile<'diagnostic> {
    start: usize,
    location: Location,
    num_multi_labels: usize,
    lines: BTreeMap<usize, Line<'diagnostic>>,
    max_label_style: LabelStyle,
}

impl<'diagnostic> LabeledFile<'diagnostic> {
    fn get_or_insert_line(
        &mut self,
        line_index: usize,
        line_range: Range<usize>,
        line_number: usize,
    ) -> &mut Line<'diagnostic> {
        self.lines.entry(line_index).or_insert_with(|| Line {
            range: line_range,
            number: line_number,
            single_labels: vec![],
            multi_labels: vec![],
            // This has to be false by default so we know if it must be rendered by another condition already.
            must_render: false,
        })
    }
}

struct Line<'diagnostic> {
    number: usize,
    range: Range<usize>,
    // TODO: How do we reuse these allocations?
    single_labels: Vec<SingleLabel<'diagnostic>>,
    multi_labels: Vec<(usize, LabelStyle, MultiLabel<'diagnostic>)>,
    must_render: bool,
}

#[cfg(test)]
mod tests {
    use crate::codespan::SourceFile;
    use crate::{self as rome_console, BufferConsole, Console, Message};
    use crate::{
        codespan::{Codespan, Label, LabelStyle, Location, Locus, Severity},
        markup,
    };

    #[test]
    fn test_codespan() {
        const SOURCE: &str = "Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut
labore et dolore magna aliqua";

        const DIAGNOSTIC: &str = "  ┌─ file_name:2:12
  │  
2 │   consectetur adipiscing elit,
  │               ^^^^^^^^^^^^^^^ Important message
3 │   sed do eiusmod tempor incididunt ut
  │ ╭──────────────'
4 │ │ labore et dolore magna aliqua
  │ │        --------- Secondary message
  │ ╰──────' Multiline message
";

        let source = SourceFile::new(SOURCE);

        let mut codespan = Codespan::new(
            &source,
            Severity::Error,
            Some(Locus::FileLocation {
                name: String::from("file_name"),
                location: Location {
                    line_number: 2,
                    column_number: 12,
                },
            }),
        );

        codespan
            .add_label(Label {
                style: LabelStyle::Primary,
                range: 40..55,
                message: markup! {
                    <Emphasis>"Important"</Emphasis>" message"
                },
            })
            .unwrap();

        codespan
            .add_label(Label {
                style: LabelStyle::Secondary,
                range: 71..99,
                message: markup! {
                    "Multiline message"
                },
            })
            .unwrap();

        codespan
            .add_label(Label {
                style: LabelStyle::Secondary,
                range: 100..109,
                message: markup! {
                    "Secondary message"
                },
            })
            .unwrap();

        let mut console = BufferConsole::default();
        console.message(markup! {
            {codespan}
        });

        let mut iter = console.buffer.into_iter();

        let message = match iter.next() {
            Some(Message::Message(msg)) => msg,
            other => panic!("unexpected message {other:?}"),
        };

        assert_eq!(message, DIAGNOSTIC);

        assert!(iter.next().is_none());
    }
}
