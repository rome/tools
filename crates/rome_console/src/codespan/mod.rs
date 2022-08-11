//! This module is a fork of <https://github.com/brendanzab/codespan>
//! adapted to use the `rome_console` markup for formatting
use std::collections::BTreeMap;
use std::io;

use crate::fmt::{Display, Formatter};
use crate::markup::MarkupBuf;
use text_size::{TextRange, TextSize};

use self::render::{MultiLabel, Renderer, SingleLabel};

mod render;

pub use self::render::WithSeverity;

const START_CONTEXT_LINES: usize = 3;
const END_CONTEXT_LINES: usize = 1;

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Clone)]
pub struct Label {
    /// The style of the label.
    pub style: LabelStyle,
    /// The range in bytes we are going to include in the final snippet.
    pub range: TextRange,
    /// An optional message to provide some additional information for the
    /// underlined code. These should not include line breaks.
    pub message: MarkupBuf,
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum Severity {
    /// A help message.
    Help,
    /// A note.
    Note,
    /// A warning.
    Warning,
    /// An error.
    Error,
    /// An unexpected bug.
    Bug,
}

impl From<Severity> for &'static str {
    fn from(level: Severity) -> Self {
        match level {
            Severity::Bug => "bug",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Help => "help",
            Severity::Note => "note",
        }
    }
}

/// The 'location focus' of a source code snippet.
#[derive(Copy, Clone)]
pub enum Locus<'diagnostic> {
    File {
        /// The user-facing name of the file.
        name: &'diagnostic str,
    },
    FileLocation {
        /// The user-facing name of the file.
        name: &'diagnostic str,
        /// The location.
        location: Location,
    },
}

impl<'diagnostic> Display for Locus<'diagnostic> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        match self {
            Locus::File { name } => write!(fmt, "{name}"),
            Locus::FileLocation { name, location } => write!(
                fmt,
                "{name}:{line_number}:{column_number}",
                name = name,
                line_number = location.line_number,
                column_number = location.column_number,
            ),
        }
    }
}

/// A user-facing location in a source file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    /// The user-facing line number.
    pub line_number: usize,
    /// The user-facing column number.
    pub column_number: usize,
}

#[derive(Copy, Clone)]
pub struct Codespan<'diagnostic> {
    /// Source code and line indices for the file being annotated
    pub source_file: SourceFile<'diagnostic>,
    /// Overall severity of the codespan, used to select a color for primary labels
    pub severity: Severity,
    /// Optional locus to show at the top of the codespan
    pub locus: Option<Locus<'diagnostic>>,
    /// List of labels to draw on top of the source file
    pub labels: &'diagnostic [Label],
}

impl<'diagnostic> Display for Codespan<'diagnostic> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let mut outer_padding = 0;
        let mut labeled_file: Option<LabeledFile> = None;

        for label in self.labels {
            let start_line_index = self.source_file.line_index(label.range.start());
            let start_line_number = self.source_file.line_number(start_line_index);

            let start_line_range = self.source_file.line_range(start_line_index)?;
            let end_line_index = self.source_file.line_index(label.range.end());
            let end_line_number = self.source_file.line_number(end_line_index);

            let end_line_range = self.source_file.line_range(end_line_index)?;

            outer_padding = std::cmp::max(outer_padding, count_digits(start_line_number));
            outer_padding = std::cmp::max(outer_padding, count_digits(end_line_number));

            let labeled_file = match &mut labeled_file {
                Some(labeled_file) => {
                    // other labezls already exist in this codespan
                    if labeled_file.max_label_style > label.style
                        || (labeled_file.max_label_style == label.style
                            && labeled_file.start > label.range.start())
                    {
                        // this label has a higher style or has the same style but starts earlier
                        labeled_file.start = label.range.start();
                        labeled_file.location = self.source_file.location(label.range.start())?;
                        labeled_file.max_label_style = label.style;
                    }
                    labeled_file
                }
                None => {
                    // this is the first label inserted into this codespan
                    labeled_file.get_or_insert(LabeledFile {
                        start: label.range.start(),
                        location: self.source_file.location(label.range.start())?,
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
                let label_start = label.range.start() - start_line_range.start();
                // Ensure that we print at least one caret, even when we
                // have a zero-length source range.
                let label_end = TextSize::max(
                    label.range.end() - start_line_range.start(),
                    label_start + TextSize::from(1u32),
                );

                let line = labeled_file.get_or_insert_line(
                    start_line_index,
                    start_line_range,
                    start_line_number,
                );

                // Ensure that the single line labels are lexicographically
                // sorted by the range of source code that they cover.
                let index = match line.single_labels.binary_search_by(|(_, range, _)| {
                    // `TextRange` doesn't implement `Ord`, so convert to `(usize, usize)`
                    // to piggyback off its lexicographic comparison implementation.
                    (range.start(), range.end()).cmp(&(label_start, label_end))
                }) {
                    // If the ranges are the same, order the labels in reverse
                    // to how they were originally specified in the diagnostic.
                    // This helps with printing in the renderer.
                    Ok(index) | Err(index) => index,
                };

                line.single_labels.insert(
                    index,
                    (
                        label.style,
                        TextRange::new(label_start, label_end),
                        &label.message,
                    ),
                );

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
                let label_start = label.range.start() - start_line_range.start();

                let start_line = labeled_file.get_or_insert_line(
                    start_line_index,
                    start_line_range,
                    start_line_number,
                );

                start_line.multi_labels.push((
                    label_index,
                    label.style,
                    MultiLabel::Top(label_start),
                ));

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

                    outer_padding = std::cmp::max(outer_padding, count_digits(line_number));

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
                let label_end = label.range.end() - end_line_range.start();

                let end_line = labeled_file.get_or_insert_line(
                    end_line_index,
                    end_line_range,
                    end_line_number,
                );

                end_line.multi_labels.push((
                    label_index,
                    label.style,
                    MultiLabel::Bottom(label_end, &label.message),
                ));

                // The last line has to be rendered so the end of the label is visible.
                end_line.must_render = true;
            }
        }

        let mut renderer = Renderer::new(&mut *fmt);
        let file = match &labeled_file {
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
                renderer.render_snippet_start(outer_padding, locus)?;
            }

            renderer.render_snippet_empty(
                outer_padding,
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
                outer_padding,
                line.number,
                line.range,
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
                            outer_padding,
                            line_number,
                            line_range,
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
                            outer_padding,
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

/// Representation of a single source file holding additional information for
/// efficiently rendering [Codespan]
#[derive(Clone, Copy)]
pub struct SourceFile<'diagnostic> {
    /// The source code of the file.
    pub source: &'diagnostic str,
    /// The starting byte indices in the source code.
    line_starts: &'diagnostic [TextSize],
}

impl<'diagnostic> SourceFile<'diagnostic> {
    /// Create a new [SourceFile] from a slice of text
    pub fn new(source: &'diagnostic str, line_starts: &'diagnostic [TextSize]) -> Self {
        Self {
            source,
            line_starts,
        }
    }

    /// Return the starting byte index of each line in the source string.
    pub fn line_starts(source: &'_ str) -> impl '_ + Iterator<Item = TextSize> {
        std::iter::once(0)
            .chain(source.match_indices(&['\n', '\r']).filter_map(|(i, _)| {
                let bytes = source.as_bytes();

                match bytes[i] {
                    // Filter out the `\r` in `\r\n` to avoid counting the line break twice
                    b'\r' if i + 1 < bytes.len() && bytes[i + 1] == b'\n' => None,
                    _ => Some(i + 1),
                }
            }))
            .map(|i| TextSize::try_from(i).expect("integer overflow"))
    }

    /// Return the starting byte index of the line with the specified line index.
    /// Convenience method that already generates errors if necessary.
    fn line_start(&self, line_index: usize) -> io::Result<TextSize> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self
                .line_starts
                .get(line_index)
                .cloned()
                .expect("failed despite previous check")),
            Ordering::Equal => Ok(TextSize::of(self.source)),
            Ordering::Greater => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "overflow error",
            )),
        }
    }

    fn line_index(&self, byte_index: TextSize) -> usize {
        self.line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1)
    }

    fn line_range(&self, line_index: usize) -> io::Result<TextRange> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(TextRange::new(line_start, next_line_start))
    }

    fn line_number(&self, line_index: usize) -> usize {
        line_index + 1
    }

    fn column_number(&self, line_index: usize, byte_index: TextSize) -> io::Result<usize> {
        let source = self.source;
        let line_range = self.line_range(line_index)?;
        let column_index = column_index(source, line_range, byte_index);

        Ok(column_index + 1)
    }

    /// Get a source location from a byte index into the text of this file
    pub fn location(&self, byte_index: TextSize) -> io::Result<Location> {
        let line_index = self.line_index(byte_index);

        Ok(Location {
            line_number: self.line_number(line_index),
            column_number: self.column_number(line_index, byte_index)?,
        })
    }
}

/// The column index at the given byte index in the source file.
/// This is the number of characters to the given byte index.
///
/// If the byte index is smaller than the start of the line, then `0` is returned.
/// If the byte index is past the end of the line, the column index of the last
/// character `+ 1` is returned.
fn column_index(source: &str, line_range: TextRange, byte_index: TextSize) -> usize {
    let end_index = std::cmp::min(
        byte_index,
        std::cmp::min(line_range.end(), TextSize::of(source)),
    );

    (usize::from(line_range.start())..usize::from(end_index))
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
    start: TextSize,
    location: Location,
    num_multi_labels: usize,
    lines: BTreeMap<usize, Line<'diagnostic>>,
    max_label_style: LabelStyle,
}

impl<'diagnostic> LabeledFile<'diagnostic> {
    fn get_or_insert_line(
        &mut self,
        line_index: usize,
        line_range: TextRange,
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
    range: TextRange,
    // TODO: How do we reuse these allocations?
    single_labels: Vec<SingleLabel<'diagnostic>>,
    multi_labels: Vec<(usize, LabelStyle, MultiLabel<'diagnostic>)>,
    must_render: bool,
}

#[cfg(test)]
mod tests {
    use text_size::{TextRange, TextSize};

    use crate::codespan::SourceFile;
    use crate::{self as rome_console, BufferConsole, ConsoleExt, LogLevel, Markup};
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

        const DIAGNOSTIC: Markup<'static> = markup! {
            "  "<Info>"┌─"</Info>" file_name:2:12\n  "
                <Info>"│"</Info>"  \n"
                <Info>"2"</Info>" "<Info>"│"</Info>"   consectetur "<Error>"adipiscing elit"</Error>",\n  "
                <Info>"│"</Info>                   "               "<Error>"^^^^^^^^^^^^^^^"</Error>" "<Error><Emphasis>"Important"</Emphasis>" message"</Error>"\n"
                <Info>"3"</Info>" "<Info>"│"</Info>"   sed do eiusmod tempor incididunt ut\n  "
                <Info>"│"</Info>" "<Info>          "┌───────────────'"</Info>"\n"
                <Info>"4"</Info>" "<Info>          "│"</Info>" "<Info>"│"</Info>" labore et dolore magna aliqua\n  "
                <Info>"│"</Info>" "<Info>          "│"</Info>"        "<Info>"---------"</Info>" "<Info>"Secondary message"</Info>"\n  "
                <Info>"│"</Info>" "<Info>          "└──────' Multiline message"</Info>"\n"
        };

        let lines_starts: Vec<_> = SourceFile::line_starts(SOURCE).collect();
        let source_file = SourceFile::new(SOURCE, &lines_starts);

        let codespan = Codespan {
            source_file,
            severity: Severity::Error,
            locus: Some(Locus::FileLocation {
                name: "file_name",
                location: Location {
                    line_number: 2,
                    column_number: 12,
                },
            }),
            labels: &[
                Label {
                    style: LabelStyle::Primary,
                    range: TextRange::new(TextSize::from(40u32), TextSize::from(55u32)),
                    message: markup! {
                        <Emphasis>"Important"</Emphasis>" message"
                    }
                    .to_owned(),
                },
                Label {
                    style: LabelStyle::Secondary,
                    range: TextRange::new(TextSize::from(71u32), TextSize::from(99u32)),
                    message: markup! {
                        "Multiline message"
                    }
                    .to_owned(),
                },
                Label {
                    style: LabelStyle::Secondary,
                    range: TextRange::new(TextSize::from(100u32), TextSize::from(109u32)),
                    message: markup! {
                        "Secondary message"
                    }
                    .to_owned(),
                },
            ],
        };

        let mut console = BufferConsole::default();
        console.log(markup! {
            {codespan}
        });

        let mut iter = console.out_buffer.into_iter();

        let message = iter
            .next()
            .expect("the buffer console should have a message in memory");

        assert_eq!(message.level, LogLevel::Log);
        assert_eq!(message.content, DIAGNOSTIC.to_owned());

        assert!(iter.next().is_none());
    }

    #[test]
    fn line_starts_with_carriage_return_line_feed() {
        let input = "a\r\nb\r\nc";
        let starts = SourceFile::line_starts(input).collect::<Vec<_>>();

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(3u32),
                TextSize::from(6u32)
            ],
            starts
        );
    }

    #[test]
    fn line_starts_with_carriage_return() {
        let input = "a\rb\rc";
        let starts = SourceFile::line_starts(input).collect::<Vec<_>>();

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(2u32),
                TextSize::from(4u32)
            ],
            starts
        );
    }

    #[test]
    fn line_starts_with_line_feed() {
        let input = "a\nb\nc";
        let starts = SourceFile::line_starts(input).collect::<Vec<_>>();

        assert_eq!(
            vec![
                TextSize::from(0u32),
                TextSize::from(2u32),
                TextSize::from(4u32)
            ],
            starts
        );
    }
}
