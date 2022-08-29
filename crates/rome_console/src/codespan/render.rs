use std::io;
use std::{io::Error, ops::Range};

use rome_text_size::{TextRange, TextSize};

use crate::fmt::Display;
use crate::markup::MarkupBuf;
use crate::{self as rome_console, MarkupNode};
use crate::{
    codespan::{LabelStyle, Locus, Severity},
    fmt::Formatter,
    markup, Markup, MarkupElement,
};

const MAX_LINE_LENGTH: u32 = 250;

const SOURCE_BORDER_TOP_LEFT: char = '┌';
const SOURCE_BORDER_TOP: char = '─';
const SOURCE_BORDER_LEFT: char = '│';
const SOURCE_BORDER_LEFT_BREAK: char = '·';

const SINGLE_PRIMARY_CARET: char = '^';
const SINGLE_SECONDARY_CARET: char = '-';

const MULTI_PRIMARY_CARET_START: char = '^';
const MULTI_SECONDARY_CARET_START: char = '\'';
const MULTI_TOP_LEFT: char = '┌';
const MULTI_TOP: char = '─';
const MULTI_BOTTOM_LEFT: char = '└';
const MULTI_BOTTOM: char = '─';
const MULTI_LEFT: char = '│';

const POINTER_LEFT: char = '│';

/// Prints a piece of markup with the appropriate formatting for the given
/// label style and severity
#[derive(Clone, Copy)]
pub struct WithSeverity<'a>(pub LabelStyle, pub Severity, pub &'a dyn Display);

impl<'a> Display for WithSeverity<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let element = match (self.0, self.1) {
            (LabelStyle::Primary, Severity::Bug) => MarkupElement::Error,
            (LabelStyle::Primary, Severity::Error) => MarkupElement::Error,
            (LabelStyle::Primary, Severity::Warning) => MarkupElement::Warn,
            (LabelStyle::Primary, Severity::Note) => MarkupElement::Success,
            (LabelStyle::Primary, Severity::Help) => MarkupElement::Info,
            (LabelStyle::Secondary, _) => MarkupElement::Info,
        };

        fmt.write_markup(Markup(&[MarkupNode {
            elements: &[element],
            content: self.2,
        }]))
    }
}

/// Single-line label, with an optional message.
///
/// ```text
/// ^^^^^^^^^ blah blah
/// ```
pub(super) type SingleLabel<'diagnostic> = (LabelStyle, TextRange, &'diagnostic MarkupBuf);

/// A multi-line label to render.
///
/// Locations are relative to the start of where the source code is rendered.
pub(super) enum MultiLabel<'diagnostic> {
    /// Multi-line label top.
    /// The contained value indicates where the label starts.
    ///
    /// ```text
    /// ╭────────────^
    /// ```
    ///
    /// Can also be rendered at the beginning of the line
    /// if there is only whitespace before the label starts.
    ///
    /// /// ```text
    /// ╭
    /// ```
    Top(TextSize),
    /// Left vertical labels for multi-line labels.
    ///
    /// ```text
    /// │
    /// ```
    Left,
    /// Multi-line label bottom, with an optional message.
    /// The first value indicates where the label ends.
    ///
    /// ```text
    /// ╰────────────^ blah blah
    /// ```
    Bottom(TextSize, &'diagnostic MarkupBuf),
}

#[derive(Copy, Clone)]
enum VerticalBound {
    Top,
    Bottom,
}

type Underline = (LabelStyle, VerticalBound);

/// A renderer of display list entries.
///
/// The following diagram gives an overview of each of the parts of the renderer's output:
///
/// ```text
///                     ┌ outer gutter
///                     │ ┌ left border
///                     │ │ ┌ inner gutter
///                     │ │ │   ┌─────────────────────────── source ─────────────────────────────┐
///                     │ │ │   │                                                                │
///                  ┌────────────────────────────────────────────────────────────────────────────
/// snippet start ── │    ┌─ test:9:0
/// snippet empty ── │    │
///  snippet line ── │  9 │   ╭ Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
///  snippet line ── │ 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
///                  │    │ ╭─│─────────^
/// snippet break ── │    · │ │
///  snippet line ── │ 33 │ │ │ Muffin danish chocolate soufflé pastry icing bonbon oat cake.
///  snippet line ── │ 34 │ │ │ Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
///                  │    │ │ ╰─────────────────────────────^ blah blah
/// snippet break ── │    · │
///  snippet line ── │ 38 │ │   Brownie lemon drops chocolate jelly-o candy canes. Danish marzipan
///  snippet line ── │ 39 │ │   jujubes soufflé carrot cake marshmallow tiramisu caramels candy canes.
///                  │    │ │           ^^^^^^^^^^^^^^^^^^^ -------------------- blah blah
///                  │    │ │           │
///                  │    │ │           blah blah
///                  │    │ │           note: this is a note
///  snippet line ── │ 40 │ │   Fruitcake jelly-o danish toffee. Tootsie roll pastry cheesecake
///  snippet line ── │ 41 │ │   soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
///  snippet line ── │ 42 │ │   cupcake. Candy canes cupcake toffee gingerbread candy canes muffin
///                  │    │ │                                ^^^^^^^^^^^^^^^^^^ blah blah
///                  │    │ ╰──────────^ blah blah
/// snippet break ── │    ·
///  snippet line ── │ 82 │     gingerbread toffee chupa chups chupa chups jelly-o cotton candy.
///                  │    │                 ^^^^^^                         ------- blah blah
///         empty ── │
/// ```
///
/// Filler text from http://www.cupcakeipsum.com
pub(super) struct Renderer<'render, 'fmt> {
    writer: &'render mut Formatter<'fmt>,
}

impl<'render, 'fmt> Renderer<'render, 'fmt> {
    /// Construct a renderer from the given writer and config.
    pub(super) fn new(writer: &'render mut Formatter<'fmt>) -> Renderer<'render, 'fmt> {
        Renderer { writer }
    }

    /// Top left border and locus.
    ///
    /// ```text
    /// ┌─ test:2:9
    /// ```
    pub(super) fn render_snippet_start(
        &mut self,
        outer_padding: usize,
        locus: &Locus,
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;
        self.writer.write_markup(markup! {
            <Info>{SOURCE_BORDER_TOP_LEFT}{SOURCE_BORDER_TOP}</Info>" "{locus}"\n"
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn render_snippet_source_impl(
        &mut self,
        outer_padding: usize,
        line_number: usize,
        source: &str,
        severity: Severity,
        single_labels: &[SingleLabel<'_>],
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        // Trim trailing newlines, linefeeds, and null chars from source, if they exist.
        // FIXME: Use the number of trimmed placeholders when rendering single line carets
        let source = source.trim_end_matches(['\n', '\r', '\0'].as_ref());

        // Write source line
        //
        // ```text
        // 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
        // ```
        {
            // Write outer gutter (with line number) and border
            self.outer_gutter_number(line_number, outer_padding)?;
            self.border_left()?;

            // Write inner gutter (with multi-line continuations on the left if necessary)
            let mut multi_labels_iter = multi_labels.iter().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((label_index, label_style, label)) if *label_index == label_column => {
                        match label {
                            MultiLabel::Top(start)
                                if *start
                                    <= TextSize::of(source) - TextSize::of(source.trim_start()) =>
                            {
                                self.label_multi_top_left(severity, *label_style)?;
                            }
                            MultiLabel::Top(..) => self.inner_gutter_space()?,
                            MultiLabel::Left | MultiLabel::Bottom(..) => {
                                self.label_multi_left(severity, *label_style, None)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _, _)) | None => self.inner_gutter_space()?,
                }
            }

            // Write source text
            write!(self.writer, " ")?;
            for (metrics, ch) in self.char_metrics(source.char_indices()) {
                let column_range =
                    TextRange::new(metrics.byte_index, metrics.byte_index + TextSize::of(ch));

                // Check if we are overlapping a primary label
                let is_primary = single_labels.iter().any(|(ls, range, _)| {
                    *ls == LabelStyle::Primary && is_overlapping(range, &column_range)
                }) || multi_labels.iter().any(|(_, ls, label)| {
                    *ls == LabelStyle::Primary
                        && match label {
                            MultiLabel::Top(start) => column_range.start() >= *start,
                            MultiLabel::Left => true,
                            MultiLabel::Bottom(start, _) => column_range.end() <= *start,
                        }
                });

                match ch {
                    '\t' => {
                        (0..metrics.unicode_width).try_for_each(|_| write!(self.writer, " "))?
                    }
                    _ => {
                        // Set the source color if we are in a primary label
                        if is_primary {
                            let style = match severity {
                                Severity::Bug | Severity::Error => MarkupElement::Error,
                                Severity::Warning => MarkupElement::Warn,
                                Severity::Note => MarkupElement::Info,
                                Severity::Help => MarkupElement::Info,
                            };

                            self.writer.write_markup(Markup(&[MarkupNode {
                                elements: &[style],
                                content: &ch,
                            }]))?
                        } else {
                            write!(self.writer, "{}", ch)?
                        }
                    }
                }
            }
            writeln!(self.writer)?;
        }

        // Write single labels underneath source
        //
        // ```text
        //   │     - ---- ^^^ second mutable borrow occurs here
        //   │     │ │
        //   │     │ first mutable borrow occurs here
        //   │     first borrow later used by call
        //   │     help: some help here
        // ```
        if !single_labels.is_empty() {
            // Our plan is as follows:
            //
            // 1. Do an initial scan to find:
            //    - The number of non-empty messages.
            //    - The right-most start and end positions of labels.
            //    - A candidate for a trailing label (where the label's message
            //      is printed to the left of the caret).
            // 2. Check if the trailing label candidate overlaps another label -
            //    if so we print it underneath the carets with the other labels.
            // 3. Print a line of carets, and (possibly) the trailing message
            //    to the left.
            // 4. Print vertical lines pointing to the carets, and the messages
            //    for those carets.
            //
            // We try our best avoid introducing new dynamic allocations,
            // instead preferring to iterate over the labels multiple times. It
            // is unclear what the performance tradeoffs are however, so further
            // investigation may be required.

            // The number of non-empty messages to print.
            let mut num_messages = 0;
            // The right-most start position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //           │
            //           right-most start position
            // ```
            let mut max_label_start = TextSize::from(0u32);
            // The right-most end position, eg:
            //
            // ```text
            // -^^^^---- ^^^^^^^
            //                 │
            //                 right-most end position
            // ```
            let mut max_label_end = TextSize::from(0u32);
            // A trailing message, eg:
            //
            // ```text
            // ^^^ second mutable borrow occurs here
            // ```
            let mut trailing_label = None;

            for (label_index, label) in single_labels.iter().enumerate() {
                let (_, range, message) = label;
                if !message.is_empty() {
                    num_messages += 1;
                }
                max_label_start = std::cmp::max(max_label_start, range.start());
                max_label_end = std::cmp::max(max_label_end, range.end());
                // This is a candidate for the trailing label, so let's record it.
                if range.end() == max_label_end {
                    if message.is_empty() {
                        trailing_label = None;
                    } else {
                        trailing_label = Some((label_index, label));
                    }
                }
            }

            if let Some((trailing_label_index, (_, trailing_range, _))) = trailing_label {
                // Check to see if the trailing label candidate overlaps any of
                // the other labels on the current line.
                if single_labels
                    .iter()
                    .enumerate()
                    .filter(|(label_index, _)| *label_index != trailing_label_index)
                    .any(|(_, (_, range, _))| is_overlapping(trailing_range, range))
                {
                    // If it does, we'll instead want to render it below the
                    // carets along with the other hanging labels.
                    trailing_label = None;
                }
            }

            // Write a line of carets
            //
            // ```text
            //   │ ^^^^^^  -------^^^^^^^^^-------^^^^^----- ^^^^ trailing label message
            // ```
            self.outer_gutter(outer_padding)?;
            self.border_left()?;
            self.inner_gutter(severity, num_multi_labels, multi_labels)?;
            write!(self.writer, " ")?;

            let placeholder_metrics = Metrics {
                byte_index: TextSize::of(source),
                unicode_width: 1,
            };
            for (metrics, ch) in self
                .char_metrics(source.char_indices())
                // Add a placeholder source column at the end to allow for
                // printing carets at the end of lines, eg:
                //
                // ```text
                // 1 │ Hello world!
                //   │             ^
                // ```
                .chain(std::iter::once((placeholder_metrics, '\0')))
            {
                // Find the current label style at this column
                let column_range =
                    TextRange::new(metrics.byte_index, metrics.byte_index + TextSize::of(ch));
                let current_label_style = single_labels
                    .iter()
                    .filter(|(_, range, _)| is_overlapping(range, &column_range))
                    .map(|(label_style, _, _)| *label_style)
                    .max_by_key(label_priority_key);

                let caret_ch = match current_label_style {
                    Some(LabelStyle::Primary) => Some(SINGLE_PRIMARY_CARET),
                    Some(LabelStyle::Secondary) => Some(SINGLE_SECONDARY_CARET),
                    // Only print padding if we are before the end of the last single line caret
                    None if metrics.byte_index < max_label_end => Some(' '),
                    None => None,
                };

                match (current_label_style, caret_ch) {
                    (_, None) => {}
                    (None, Some(caret_ch)) => {
                        // FIXME: improve rendering of carets between character boundaries
                        (0..metrics.unicode_width)
                            .try_for_each(|_| write!(self.writer, "{}", caret_ch))?;
                    }
                    (Some(label_style), Some(caret_ch)) => {
                        for _ in 0..metrics.unicode_width {
                            self.writer.write_markup(markup! {
                                {WithSeverity(label_style, severity, &markup! { {caret_ch} })}
                            })?;
                        }
                    }
                }
            }
            // Write first trailing label message
            if let Some((_, (label_style, _, message))) = trailing_label {
                if !message.is_empty() {
                    write!(self.writer, " ")?;
                    self.writer.write_markup(markup! {
                        {WithSeverity(*label_style, severity, message)}
                    })?;
                }
            }
            writeln!(self.writer)?;

            // Write hanging labels pointing to carets
            //
            // ```text
            //   │     │ │
            //   │     │ first mutable borrow occurs here
            //   │     first borrow later used by call
            //   │     help: some help here
            // ```
            if num_messages > trailing_label.iter().count() {
                // Write first set of vertical lines before hanging labels
                //
                // ```text
                //   │     │ │
                // ```
                self.outer_gutter(outer_padding)?;
                self.border_left()?;
                self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                write!(self.writer, " ")?;
                self.caret_pointers(
                    severity,
                    max_label_start,
                    single_labels,
                    trailing_label,
                    source.char_indices(),
                )?;
                writeln!(self.writer)?;

                // Write hanging labels pointing to carets
                //
                // ```text
                //   │     │ first mutable borrow occurs here
                //   │     first borrow later used by call
                //   │     help: some help here
                // ```
                for (label_style, range, message) in
                    hanging_labels(single_labels, trailing_label).rev()
                {
                    self.outer_gutter(outer_padding)?;
                    self.border_left()?;
                    self.inner_gutter(severity, num_multi_labels, multi_labels)?;
                    write!(self.writer, " ")?;
                    self.caret_pointers(
                        severity,
                        max_label_start,
                        single_labels,
                        trailing_label,
                        source
                            .char_indices()
                            .take_while(|(byte_index, _)| *byte_index < range.start().into()),
                    )?;
                    self.writer.write_markup(markup! {
                        {WithSeverity(*label_style, severity, *message)}
                    })?;
                    writeln!(self.writer)?;
                }
            }
        }

        // Write top or bottom label carets underneath source
        //
        // ```text
        //     │ ╰───│──────────────────^ woops
        //     │   ╭─│─────────^
        // ```
        for (multi_label_index, (_, label_style, label)) in multi_labels.iter().enumerate() {
            let (label_style, range, bottom_message) = match label {
                MultiLabel::Left => continue, // no label caret needed
                // no label caret needed if this can be started in front of the line
                MultiLabel::Top(start)
                    if *start <= TextSize::of(source) - TextSize::of(source.trim_start()) =>
                {
                    continue
                }
                MultiLabel::Top(range) => (*label_style, range, None),
                MultiLabel::Bottom(range, message) => (*label_style, range, Some(message)),
            };

            self.outer_gutter(outer_padding)?;
            self.border_left()?;

            // Write inner gutter.
            //
            // ```text
            //  │ ╭─│───│
            // ```
            let mut underline = None;
            let mut multi_labels_iter = multi_labels.iter().enumerate().peekable();
            for label_column in 0..num_multi_labels {
                match multi_labels_iter.peek() {
                    Some((i, (label_index, ls, label))) if *label_index == label_column => {
                        match label {
                            MultiLabel::Left => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(..) if multi_label_index > *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Bottom(..) if multi_label_index < *i => {
                                self.label_multi_left(severity, *ls, underline.map(|(s, _)| s))?;
                            }
                            MultiLabel::Top(..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Top));
                                self.label_multi_top_left(severity, label_style)?
                            }
                            MultiLabel::Bottom(..) if multi_label_index == *i => {
                                underline = Some((*ls, VerticalBound::Bottom));
                                self.label_multi_bottom_left(severity, label_style)?;
                            }
                            MultiLabel::Top(..) | MultiLabel::Bottom(..) => {
                                self.inner_gutter_column(severity, underline)?;
                            }
                        }
                        multi_labels_iter.next();
                    }
                    Some((_, _)) | None => self.inner_gutter_column(severity, underline)?,
                }
            }

            // Finish the top or bottom caret
            match bottom_message {
                None => self.label_multi_top_caret(severity, label_style, source, *range)?,
                Some(message) => {
                    self.label_multi_bottom_caret(severity, label_style, source, *range, message)?
                }
            }
        }

        Ok(())
    }

    fn render_snippet_source_inside_of_long_line(
        &mut self,
        line_number: usize,
        line_range: TextRange,
        severity: Severity,
        single_labels: &mut Vec<(LabelStyle, TextRange, &MarkupBuf)>,
        outer_padding: usize,
        source: &str,
    ) -> Result<(), Error> {
        let labels_start = single_labels
            .first()
            .map_or(line_range.start(), |x| x.1.start());
        let labels_end = single_labels.last().map_or(line_range.end(), |x| x.1.end());

        // If labels width are larger then max_line_length, we will
        // trim the label
        let labels_width = (labels_end - labels_start).min(TextSize::from(MAX_LINE_LENGTH));

        let spacing = u32::from(TextSize::from(MAX_LINE_LENGTH) - labels_width) / 2;

        // We will try to center the interesting part of the line
        let interesting_part_start =
            TextSize::from(u32::from(labels_start).saturating_sub(spacing));
        let interesting_part_end = TextSize::from(u32::from(labels_end).saturating_add(spacing));
        let interesting_part_range = TextRange::new(interesting_part_start, interesting_part_end);

        // labels range are relative to the start of the line, now we
        // need the range relative to the file start.
        let mut new_code_range = TextRange::new(
            TextSize::from(
                u32::from(line_range.start()).saturating_add(interesting_part_range.start().into()),
            ),
            TextSize::from(
                u32::from(line_range.start()).saturating_add(interesting_part_range.end().into()),
            ),
        );

        // We need to adjust all labels ranges to be relative to the start
        // of the interesting part
        for label in single_labels.iter_mut() {
            label.1 = TextRange::new(
                label.1.start() - interesting_part_range.start(),
                // We need to limit the width of the range
                (label.1.end() - interesting_part_range.start())
                    .min(interesting_part_range.start() + TextSize::from(MAX_LINE_LENGTH)),
            );
        }

        // and the width of what we are going to print
        new_code_range = TextRange::new(
            new_code_range.start(),
            new_code_range
                .end()
                .min(new_code_range.start() + TextSize::from(MAX_LINE_LENGTH)),
        );

        let source = source
            .get(Range::<usize>::from(new_code_range))
            .unwrap_or_else(|| &source[line_range]);

        self.render_snippet_source_impl(
            outer_padding,
            line_number,
            source,
            severity,
            single_labels.as_slice(),
            0,
            &[],
        )?;

        Ok(())
    }

    /// A line of source code.
    ///
    /// ```text
    /// 10 │   │ muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
    ///    │ ╭─│─────────^
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub(super) fn render_snippet_source(
        &mut self,
        outer_padding: usize,
        line_number: usize,
        line_range: TextRange,
        source: &str,
        severity: Severity,
        single_labels: &[SingleLabel<'_>],
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        // if the line is smaller than max_line_length, we print it entirely...
        // we also print it entirely if there are multi_labels
        let line_candidate = &source[line_range];
        if (TextSize::of(line_candidate) < TextSize::from(MAX_LINE_LENGTH))
            || !multi_labels.is_empty()
        {
            return self.render_snippet_source_impl(
                outer_padding,
                line_number,
                line_candidate,
                severity,
                single_labels,
                num_multi_labels,
                multi_labels,
            );
        } else {
            // ... if not, we try to fit as many single_labels as possible
            // showing only the interesting part of the line.
            let mut candidates = vec![];
            for single_label in single_labels.iter() {
                candidates.push(*single_label);

                // We need to know which part of the long line we are going to display
                let labels_start = candidates
                    .first()
                    .map_or(line_range.start(), |x| x.1.start());
                let labels_end = candidates.last().map_or(line_range.end(), |x| x.1.end());
                let labels_width = labels_end - labels_start;

                if labels_width >= TextSize::from(MAX_LINE_LENGTH) {
                    self.render_snippet_source_inside_of_long_line(
                        line_number,
                        line_range,
                        severity,
                        &mut candidates,
                        outer_padding,
                        source,
                    )?;
                    candidates.clear();
                }
            }

            if !candidates.is_empty() {
                self.render_snippet_source_inside_of_long_line(
                    line_number,
                    line_range,
                    severity,
                    &mut candidates,
                    outer_padding,
                    source,
                )?;
            }
        }

        Ok(())
    }

    /// An empty source line, for providing additional whitespace to source snippets.
    ///
    /// ```text
    /// │ │ │
    /// ```
    pub(super) fn render_snippet_empty(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;
        self.border_left()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        writeln!(self.writer)?;
        Ok(())
    }

    /// A broken source line, for labeling skipped sections of source.
    ///
    /// ```text
    /// · │ │
    /// ```
    pub(super) fn render_snippet_break(
        &mut self,
        outer_padding: usize,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        self.outer_gutter(outer_padding)?;
        self.border_left_break()?;
        self.inner_gutter(severity, num_multi_labels, multi_labels)?;
        writeln!(self.writer)?;
        Ok(())
    }

    /// Adds tab-stop aware unicode-width computations to an iterator over
    /// character indices. Assumes that the character indices begin at the start
    /// of the line.
    fn char_metrics(
        &self,
        char_indices: impl Iterator<Item = (usize, char)> + Clone,
    ) -> impl Iterator<Item = (Metrics, char)> + Clone {
        use unicode_width::UnicodeWidthChar;

        let tab_width = 4;
        let mut unicode_column = 0;

        char_indices.map(move |(byte_index, ch)| {
            let metrics = Metrics {
                byte_index: TextSize::try_from(byte_index).expect("integer overflow"),
                unicode_width: match (ch, tab_width) {
                    ('\t', 0) => 0, // Guard divide-by-zero
                    ('\t', _) => tab_width - (unicode_column % tab_width),
                    (ch, _) => ch.width().unwrap_or(0),
                },
            };
            unicode_column += metrics.unicode_width;

            (metrics, ch)
        })
    }

    /// The outer gutter of a source line.
    fn outer_gutter(&mut self, outer_padding: usize) -> Result<(), Error> {
        write!(
            self.writer,
            "{space: >width$} ",
            space = "",
            width = outer_padding
        )?;
        Ok(())
    }

    /// The outer gutter of a source line, with line number.
    fn outer_gutter_number(
        &mut self,
        line_number: usize,
        outer_padding: usize,
    ) -> Result<(), Error> {
        self.writer.write_markup(markup! {
            <Info>
                {format_args!(
                    "{line_number: >width$}",
                    line_number = line_number,
                    width = outer_padding
                )}
            </Info>
        })?;
        write!(self.writer, " ")?;
        Ok(())
    }

    /// The left-hand border of a source line.
    fn border_left(&mut self) -> Result<(), Error> {
        self.writer.write_markup(markup! {
            <Info>{SOURCE_BORDER_LEFT}</Info>
        })?;
        Ok(())
    }

    /// The broken left-hand border of a source line.
    fn border_left_break(&mut self) -> Result<(), Error> {
        self.writer.write_markup(markup! {
            <Info>{SOURCE_BORDER_LEFT_BREAK}</Info>
        })?;
        Ok(())
    }

    /// Write vertical lines pointing to carets.
    fn caret_pointers(
        &mut self,
        severity: Severity,
        max_label_start: TextSize,
        single_labels: &[SingleLabel<'_>],
        trailing_label: Option<(usize, &SingleLabel<'_>)>,
        char_indices: impl Iterator<Item = (usize, char)> + Clone,
    ) -> Result<(), Error> {
        for (metrics, ch) in self.char_metrics(char_indices) {
            let column_range = metrics.byte_index..(metrics.byte_index + TextSize::of(ch));
            let label_style = hanging_labels(single_labels, trailing_label)
                .filter(|(_, range, _)| column_range.contains(&range.start()))
                .map(|(label_style, _, _)| *label_style)
                .max_by_key(label_priority_key);

            let mut spaces = match label_style {
                None => 0..metrics.unicode_width,
                Some(label_style) => {
                    self.writer.write_markup(markup! {
                        {WithSeverity(label_style, severity, &POINTER_LEFT)}
                    })?;
                    1..metrics.unicode_width
                }
            };
            // Only print padding if we are before the end of the last single line caret
            if metrics.byte_index <= max_label_start {
                spaces.try_for_each(|_| write!(self.writer, " "))?;
            }
        }

        Ok(())
    }

    /// The left of a multi-line label.
    ///
    /// ```text
    ///  │
    /// ```
    fn label_multi_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        underline: Option<LabelStyle>,
    ) -> Result<(), Error> {
        match underline {
            None => write!(self.writer, " ")?,
            // Continue an underline horizontally
            Some(label_style) => {
                self.writer.write_markup(markup! {
                    {WithSeverity(label_style, severity, &MULTI_TOP)}
                })?;
            }
        }
        self.writer.write_markup(markup! {
            {WithSeverity(label_style, severity, &MULTI_LEFT)}
        })?;
        Ok(())
    }

    /// The top-left of a multi-line label.
    ///
    /// ```text
    ///  ╭
    /// ```
    fn label_multi_top_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> Result<(), Error> {
        write!(self.writer, " ")?;
        self.writer.write_markup(markup! {
            {WithSeverity(label_style, severity, &MULTI_TOP_LEFT)}
        })?;
        Ok(())
    }

    /// The bottom left of a multi-line label.
    ///
    /// ```text
    ///  ╰
    /// ```
    fn label_multi_bottom_left(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
    ) -> Result<(), Error> {
        write!(self.writer, " ")?;
        self.writer.write_markup(markup! {
            {WithSeverity(label_style, severity, &MULTI_BOTTOM_LEFT)}
        })?;
        Ok(())
    }

    /// Multi-line label top.
    ///
    /// ```text
    /// ─────────────^
    /// ```
    fn label_multi_top_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        start: TextSize,
    ) -> Result<(), Error> {
        self.writer.write_markup(markup! {
            {WithSeverity(label_style, severity, &MultiCaret {
                label_style,
                message: &MarkupBuf(Vec::new()),
                caret: MULTI_TOP,
                char_metrics: self
                    .char_metrics(source.char_indices())
                    .take_while(|(metrics, _)| metrics.byte_index < start + TextSize::from(1u32)),
            })}
        })?;

        writeln!(self.writer)?;
        Ok(())
    }

    /// Multi-line label bottom, with a message.
    ///
    /// ```text
    /// ─────────────^ expected `Int` but found `String`
    /// ```
    fn label_multi_bottom_caret(
        &mut self,
        severity: Severity,
        label_style: LabelStyle,
        source: &str,
        start: TextSize,
        message: &MarkupBuf,
    ) -> Result<(), Error> {
        self.writer.write_markup(markup! {
            {WithSeverity(label_style, severity, &MultiCaret {
                label_style,
                message,
                caret: MULTI_BOTTOM,
                char_metrics: self
                    .char_metrics(source.char_indices())
                    .take_while(|(metrics, _)| metrics.byte_index < start),
            })}
        })?;

        writeln!(self.writer)?;
        Ok(())
    }

    /// Writes an empty gutter space, or continues an underline horizontally.
    fn inner_gutter_column(
        &mut self,
        severity: Severity,
        underline: Option<Underline>,
    ) -> Result<(), Error> {
        match underline {
            None => self.inner_gutter_space(),
            Some((label_style, vertical_bound)) => {
                let ch = match vertical_bound {
                    VerticalBound::Top => MULTI_TOP,
                    VerticalBound::Bottom => MULTI_BOTTOM,
                };
                self.writer.write_markup(markup! {
                    {WithSeverity(label_style, severity, &markup!{
                        {ch}{ch}
                    })}
                })?;
                Ok(())
            }
        }
    }

    /// Writes an empty gutter space.
    fn inner_gutter_space(&mut self) -> Result<(), Error> {
        write!(self.writer, "  ")?;
        Ok(())
    }

    /// Writes an inner gutter, with the left lines if necessary.
    fn inner_gutter(
        &mut self,
        severity: Severity,
        num_multi_labels: usize,
        multi_labels: &[(usize, LabelStyle, MultiLabel<'_>)],
    ) -> Result<(), Error> {
        let mut multi_labels_iter = multi_labels.iter().peekable();
        for label_column in 0..num_multi_labels {
            match multi_labels_iter.peek() {
                Some((label_index, ls, label)) if *label_index == label_column => match label {
                    MultiLabel::Left | MultiLabel::Bottom(..) => {
                        self.label_multi_left(severity, *ls, None)?;
                        multi_labels_iter.next();
                    }
                    MultiLabel::Top(..) => {
                        self.inner_gutter_space()?;
                        multi_labels_iter.next();
                    }
                },
                Some((_, _, _)) | None => self.inner_gutter_space()?,
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct MultiCaret<'a, I> {
    label_style: LabelStyle,
    message: &'a MarkupBuf,
    caret: char,
    char_metrics: I,
}

impl<'a, I> Display for MultiCaret<'a, I>
where
    I: Iterator<Item = (Metrics, char)> + Clone,
{
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        for (metrics, _) in self.char_metrics.clone() {
            // FIXME: improve rendering of carets between character boundaries
            (0..metrics.unicode_width).try_for_each(|_| write!(fmt, "{}", self.caret))?;
        }

        let caret_end = match self.label_style {
            LabelStyle::Primary => MULTI_PRIMARY_CARET_START,
            LabelStyle::Secondary => MULTI_SECONDARY_CARET_START,
        };

        write!(fmt, "{}", caret_end)?;

        if !self.message.is_empty() {
            fmt.write_markup(markup! { " "{self.message} })?;
        }

        Ok(())
    }
}
struct Metrics {
    byte_index: TextSize,
    unicode_width: usize,
}

/// Check if two ranges overlap
fn is_overlapping(range0: &TextRange, range1: &TextRange) -> bool {
    let start = std::cmp::max(range0.start(), range1.start());
    let end = std::cmp::min(range0.end(), range1.end());
    start < end
}

/// For prioritizing primary labels over secondary labels when rendering carets.
fn label_priority_key(label_style: &LabelStyle) -> u8 {
    match label_style {
        LabelStyle::Secondary => 0,
        LabelStyle::Primary => 1,
    }
}

/// Return an iterator that yields the labels that require hanging messages
/// rendered underneath them.
fn hanging_labels<'labels, 'diagnostic>(
    single_labels: &'labels [SingleLabel<'diagnostic>],
    trailing_label: Option<(usize, &'labels SingleLabel<'diagnostic>)>,
) -> impl 'labels + DoubleEndedIterator<Item = &'labels SingleLabel<'diagnostic>> {
    single_labels
        .iter()
        .enumerate()
        .filter(|(_, (_, _, message))| !message.is_empty())
        .filter(move |(i, _)| trailing_label.map_or(true, |(j, _)| *i != j))
        .map(|(_, label)| label)
}
