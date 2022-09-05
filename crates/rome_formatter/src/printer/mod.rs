mod printer_options;

pub use printer_options::*;

use crate::format_element::{
    Align, ConditionalGroupContent, DedentMode, Group, IndentIfGroupBreaks, LineMode, PrintMode,
    VerbatimKind,
};
use crate::{FormatElement, GroupId, IndentStyle, Printed, SourceMarker, TextRange};

use rome_rowan::{TextLen, TextSize};
use std::iter::{once, Rev};
use std::num::NonZeroU8;

/// Prints the format elements into a string
#[derive(Debug, Default)]
pub struct Printer<'a> {
    options: PrinterOptions,
    state: PrinterState<'a>,
}

impl<'a> Printer<'a> {
    pub fn new(options: PrinterOptions) -> Self {
        Self {
            options,
            state: PrinterState::default(),
        }
    }

    /// Prints the passed in element as well as all its content
    pub fn print(self, element: &'a FormatElement) -> Printed {
        self.print_with_indent(element, 0)
    }

    /// Prints the passed in element as well as all its content,
    /// starting at the specified indentation level
    pub fn print_with_indent(mut self, element: &'a FormatElement, indent: u16) -> Printed {
        tracing::debug_span!("Printer::print").in_scope(move || {
            let mut queue = ElementCallQueue::default();

            queue.enqueue(PrintElementCall::new(
                element,
                PrintElementArgs::new(Indention::Level(indent)),
            ));

            while let Some(print_element_call) = queue.dequeue() {
                self.print_element(
                    &mut queue,
                    print_element_call.element,
                    print_element_call.args,
                );

                if queue.is_empty() && !self.state.line_suffixes.is_empty() {
                    queue.extend(self.state.line_suffixes.drain(..));
                }
            }

            Printed::new(
                self.state.buffer,
                None,
                self.state.source_markers,
                self.state.verbatim_markers,
            )
        })
    }

    /// Prints a single element and push the following elements to queue
    fn print_element(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        element: &'a FormatElement,
        args: PrintElementArgs,
    ) {
        match element {
            FormatElement::Space => {
                if self.state.line_width > 0 {
                    self.state.pending_space = true;
                }
            }
            FormatElement::Text(token) => {
                if !self.state.pending_indent.is_empty() {
                    let (indent_char, repeat_count) = match self.options.indent_style() {
                        IndentStyle::Tab => ('\t', 1),
                        IndentStyle::Space(count) => (' ', count),
                    };

                    let indent = std::mem::take(&mut self.state.pending_indent);
                    let total_indent_char_count = indent.level() as usize * repeat_count as usize;

                    self.state
                        .buffer
                        .reserve(total_indent_char_count + indent.align() as usize);

                    for _ in 0..total_indent_char_count {
                        self.print_char(indent_char);
                    }

                    for _ in 0..indent.align() {
                        self.print_char(' ');
                    }
                }

                // Print pending spaces
                if self.state.pending_space {
                    self.print_str(" ");
                    self.state.pending_space = false;
                }

                // Insert source map markers before and after the token
                //
                // If the token has source position information the start marker
                // will use the start position of the original token, and the end
                // marker will use that position + the text length of the token
                //
                // If the token has no source position (was created by the formatter)
                // both the start and end marker will use the last known position
                // in the input source (from state.source_position)
                if let Some(source) = token.source_position() {
                    self.state.source_position = *source;
                }

                self.push_marker(SourceMarker {
                    source: self.state.source_position,
                    dest: self.state.buffer.text_len(),
                });

                self.print_str(token);

                if token.source_position().is_some() {
                    self.state.source_position += TextSize::of(&**token);
                }

                self.push_marker(SourceMarker {
                    source: self.state.source_position,
                    dest: self.state.buffer.text_len(),
                });
            }

            FormatElement::Group(Group { content, id }) => {
                let group_mode = match args.mode {
                    PrintMode::Flat if self.state.measured_group_fits => {
                        // A parent group has already verified that this group fits on a single line
                        // Thus, just continue in flat mode
                        queue.extend_with_args(content.iter(), args);
                        PrintMode::Flat
                    }
                    // The printer is either in expanded mode or it's necessary to re-measure if the group fits
                    // because the printer printed a line break
                    _ => {
                        // Measure to see if the group fits up on a single line. If that's the case,
                        // print the group in "flat" mode, otherwise continue in expanded mode

                        let flat_args = args.with_print_mode(PrintMode::Flat);
                        if fits_on_line(content.iter(), flat_args, queue, self) {
                            queue.extend_with_args(content.iter(), flat_args);
                            self.state.measured_group_fits = true;
                            PrintMode::Flat
                        } else {
                            queue.extend_with_args(
                                content.iter(),
                                args.with_print_mode(PrintMode::Expanded),
                            );
                            PrintMode::Expanded
                        }
                    }
                };

                if let Some(id) = id {
                    self.state.group_modes.insert_print_mode(*id, group_mode);
                }
            }

            FormatElement::Fill(content) => {
                self.print_fill(queue, content, args);
            }

            FormatElement::List(list) => {
                queue.extend_with_args(list.iter(), args);
            }

            FormatElement::Indent(content) => {
                queue.extend_with_args(
                    content.iter(),
                    args.increment_indent_level(self.options.indent_style()),
                );
            }

            FormatElement::Dedent { content, mode } => {
                let args = match mode {
                    DedentMode::Level => args.decrement_indent(),
                    DedentMode::Root => args.reset_indent(),
                };
                queue.extend_with_args(content.iter(), args);
            }

            FormatElement::Align(Align { content, count }) => {
                queue.extend_with_args(content.iter(), args.set_indent_align(*count))
            }

            FormatElement::ConditionalGroupContent(ConditionalGroupContent {
                mode,
                content,
                group_id,
            }) => {
                let group_mode = match group_id {
                    None => args.mode,
                    Some(id) => self.state.group_modes.unwrap_print_mode(*id, element),
                };

                if &group_mode == mode {
                    queue.extend_with_args(content.iter(), args);
                }
            }

            FormatElement::IndentIfGroupBreaks(IndentIfGroupBreaks { content, group_id }) => {
                let group_mode = self.state.group_modes.unwrap_print_mode(*group_id, element);

                match group_mode {
                    PrintMode::Flat => queue.extend_with_args(content.iter(), args),
                    PrintMode::Expanded => queue.extend_with_args(
                        content.iter(),
                        args.increment_indent_level(self.options.indent_style),
                    ),
                }
            }

            FormatElement::Line(line_mode) => {
                if args.mode.is_flat()
                    && matches!(line_mode, LineMode::Soft | LineMode::SoftOrSpace)
                {
                    if line_mode == &LineMode::SoftOrSpace && self.state.line_width > 0 {
                        self.state.pending_space = true;
                    }
                } else if !self.state.line_suffixes.is_empty() {
                    self.queue_line_suffixes(element, args, queue);
                } else {
                    // Only print a newline if the current line isn't already empty
                    if self.state.line_width > 0 {
                        self.print_str("\n");
                    }

                    // Print a second line break if this is an empty line
                    if line_mode == &LineMode::Empty && !self.state.has_empty_line {
                        self.print_str("\n");
                        self.state.has_empty_line = true;
                    }

                    self.state.pending_space = false;
                    self.state.pending_indent = args.indent;

                    // Fit's only tests if groups up to the first line break fit.
                    // The next group must re-measure if it still fits.
                    self.state.measured_group_fits = false;
                }
            }

            FormatElement::LineSuffix(suffix) => {
                self.state
                    .line_suffixes
                    .extend(suffix.iter().map(|e| PrintElementCall::new(e, args)));
            }
            FormatElement::LineSuffixBoundary => {
                const HARD_BREAK: &FormatElement = &FormatElement::Line(LineMode::Hard);
                self.queue_line_suffixes(HARD_BREAK, args, queue);
            }

            FormatElement::Comment(content) => {
                queue.extend_with_args(content.iter(), args);
            }

            FormatElement::Verbatim(verbatim) => {
                if let VerbatimKind::Verbatim { length } = &verbatim.kind {
                    self.state.verbatim_markers.push(TextRange::at(
                        TextSize::from(self.state.buffer.len() as u32),
                        *length,
                    ));
                }

                queue.extend_with_args(verbatim.content.iter(), args);
            }
            FormatElement::ExpandParent => {
                // No-op, only has an effect on `fits`
                debug_assert!(
                    !args.mode.is_flat(),
                    "Fits should always return false for `ExpandParent`"
                );
            }
            FormatElement::BestFitting(best_fitting) => {
                match args.mode {
                    PrintMode::Flat if self.state.measured_group_fits => {
                        queue.enqueue(PrintElementCall::new(best_fitting.most_flat(), args))
                    }
                    _ => {
                        let last_index = best_fitting.variants().len() - 1;
                        for (index, variant) in best_fitting.variants().iter().enumerate() {
                            if index == last_index {
                                // No variant fits, take the last (most expanded) as fallback
                                queue.enqueue(PrintElementCall::new(
                                    variant,
                                    args.with_print_mode(PrintMode::Expanded),
                                ));
                                break;
                            } else {
                                // Test if this variant fits and if so, use it. Otherwise try the next
                                // variant.

                                // Try to fit only the first variant on a single line
                                let mode = if index == 0 {
                                    PrintMode::Flat
                                } else {
                                    PrintMode::Expanded
                                };

                                if fits_on_line([variant], args.with_print_mode(mode), queue, self)
                                {
                                    self.state.measured_group_fits = true;
                                    queue.enqueue(PrintElementCall::new(
                                        variant,
                                        args.with_print_mode(mode),
                                    ));
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            FormatElement::Interned(content) => queue.enqueue(PrintElementCall::new(content, args)),
            FormatElement::Label(label) => queue.extend(
                label
                    .content
                    .iter()
                    .map(|element| PrintElementCall::new(element, args)),
            ),
        }
    }

    fn push_marker(&mut self, marker: SourceMarker) {
        if let Some(last) = self.state.source_markers.last() {
            if last != &marker {
                self.state.source_markers.push(marker)
            }
        } else {
            self.state.source_markers.push(marker);
        }
    }

    fn queue_line_suffixes(
        &mut self,
        line_break: &'a FormatElement,
        args: PrintElementArgs,
        queue: &mut ElementCallQueue<'a>,
    ) {
        if self.state.line_suffixes.is_empty() {
            return;
        }

        // If the indentation level has changed since these line suffixes were queued,
        // insert a line break before to push the comments into the new indent block
        // SAFETY: Indexing into line_suffixes is guarded by the above call to is_empty
        let has_line_break = self.state.line_suffixes[0].args.indent.level() < args.indent.level();

        // Print this line break element again once all the line suffixes have been flushed
        let call_self = PrintElementCall::new(line_break, args);

        let line_break = if has_line_break {
            // Duplicate this line break element before the line
            // suffixes if a line break is required
            Some(call_self.clone())
        } else {
            None
        };

        queue.extend(
            line_break
                .into_iter()
                .chain(self.state.line_suffixes.drain(..))
                .chain(once(call_self)),
        );
    }

    /// Tries to fit as much content as possible on a single line.
    /// Each item forms a virtual group that is either printed in flat or expanded mode.
    /// It handles three different cases:
    ///
    /// * The first and second content fit on a single line. It prints the content and separator in flat mode.
    /// * The first content fits on a single line, but the second doesn't. It prints the content in flat and the separator in expanded mode.
    /// * Neither the first nor the second content fit on the line. It brings the first content and the separator in expanded mode.
    fn print_fill(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        content: &'a [FormatElement],
        args: PrintElementArgs,
    ) {
        let empty_rest = ElementCallQueue::default();

        let mut items = content.iter();

        let current_content = match items.next() {
            None => {
                // Empty list
                return;
            }
            Some(item) => item,
        };

        let mut current_fits = fits_on_line(
            [current_content],
            args.with_print_mode(PrintMode::Flat),
            &empty_rest,
            self,
        );

        self.print_all(
            queue,
            &[current_content],
            args.with_print_mode(if current_fits {
                PrintMode::Flat
            } else {
                PrintMode::Expanded
            }),
        );

        // Process remaining items
        loop {
            match (items.next(), items.next()) {
                (Some(separator), Some(next_item)) => {
                    // A line break in expanded mode is always necessary if the current item didn't fit.
                    // otherwise see if both contents fit on the line.
                    let current_and_next_fit = current_fits
                        && fits_on_line(
                            [separator, next_item],
                            args.with_print_mode(PrintMode::Flat),
                            &empty_rest,
                            self,
                        );

                    if current_and_next_fit {
                        // Print Space and next item on the same line
                        self.print_all(
                            queue,
                            &[separator, next_item],
                            args.with_print_mode(PrintMode::Flat),
                        );
                    } else {
                        // Print the separator and then check again if the next item fits on the line now
                        self.print_all(
                            queue,
                            &[separator],
                            args.with_print_mode(PrintMode::Expanded),
                        );

                        let next_fits = fits_on_line(
                            [next_item],
                            args.with_print_mode(PrintMode::Flat),
                            &empty_rest,
                            self,
                        );

                        if next_fits {
                            self.print_all(
                                queue,
                                &[next_item],
                                args.with_print_mode(PrintMode::Flat),
                            );
                        } else {
                            self.print_all(
                                queue,
                                &[next_item],
                                args.with_print_mode(PrintMode::Expanded),
                            );
                        }

                        current_fits = next_fits;
                    }
                }
                // Trailing separator
                (Some(separator), _) => {
                    let print_mode = if current_fits
                        && fits_on_line(
                            [separator],
                            args.with_print_mode(PrintMode::Flat),
                            &empty_rest,
                            self,
                        ) {
                        PrintMode::Flat
                    } else {
                        PrintMode::Expanded
                    };

                    self.print_all(queue, &[separator], args.with_print_mode(print_mode));
                }
                (None, None) => {
                    break;
                }
                (None, Some(_)) => {
                    // Unreachable for iterators implementing [FusedIterator] which slice.iter implements.
                    // Reaching this means that the first `iter.next()` returned `None` but calling `iter.next()`
                    // again returns `Some`
                    unreachable!()
                }
            }
        }
    }

    /// Fully print an element (print the element itself and all its descendants)
    ///
    /// Unlike [print_element], this function ensures the entire element has
    /// been printed when it returns and the queue is back to its original state
    fn print_all(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        elements: &[&'a FormatElement],
        args: PrintElementArgs,
    ) {
        let min_queue_length = queue.0.len();

        queue.extend(elements.iter().map(|e| PrintElementCall::new(e, args)));

        while let Some(call) = queue.dequeue() {
            self.print_element(queue, call.element, call.args);

            if queue.0.len() == min_queue_length {
                return;
            }

            debug_assert!(queue.0.len() > min_queue_length);
        }
    }

    fn print_str(&mut self, content: &str) {
        for char in content.chars() {
            self.print_char(char);

            self.state.has_empty_line = false;
        }
    }

    fn print_char(&mut self, char: char) {
        if char == '\n' {
            self.state
                .buffer
                .push_str(self.options.line_ending.as_str());
            self.state.generated_line += 1;
            self.state.generated_column = 0;
            self.state.line_width = 0;
        } else {
            self.state.buffer.push(char);
            self.state.generated_column += 1;

            let char_width = if char == '\t' {
                self.options.tab_width as usize
            } else {
                1
            };

            self.state.line_width += char_width;
        }
    }
}

/// Printer state that is global to all elements.
/// Stores the result of the print operation (buffer and mappings) and at what
/// position the printer currently is.
#[derive(Default, Debug)]
struct PrinterState<'a> {
    buffer: String,
    source_markers: Vec<SourceMarker>,
    source_position: TextSize,
    pending_indent: Indention,
    pending_space: bool,
    measured_group_fits: bool,
    generated_line: usize,
    generated_column: usize,
    line_width: usize,
    has_empty_line: bool,
    line_suffixes: Vec<PrintElementCall<'a>>,
    verbatim_markers: Vec<TextRange>,
    group_modes: GroupModes,
    // Re-used queue to measure if a group fits. Optimisation to avoid re-allocating a new
    // vec everytime a group gets measured
    measure_queue: Vec<PrintElementCall<'a>>,
}

/// Tracks the mode in which groups with ids are printed. Stores the groups at `group.id()` index.
/// This is based on the assumption that the group ids for a single document are dense.
#[derive(Debug, Default)]
struct GroupModes(Vec<Option<PrintMode>>);

impl GroupModes {
    fn insert_print_mode(&mut self, group_id: GroupId, mode: PrintMode) {
        let index = u32::from(group_id) as usize;

        self.0.resize(index + 1, None);
        self.0[index] = Some(mode);
    }

    fn get_print_mode(&self, group_id: GroupId) -> Option<PrintMode> {
        let index = u32::from(group_id) as usize;
        self.0
            .get(index)
            .and_then(|option| option.as_ref().copied())
    }

    fn unwrap_print_mode(&self, group_id: GroupId, next_element: &FormatElement) -> PrintMode {
        self.get_print_mode(group_id).unwrap_or_else(||
            panic!("Expected group with id {group_id:?} to exist but it wasn't present in the document. Ensure that a group with such a document appears in the document before the element {next_element:?}.")
        )
    }
}

/// Stores arguments passed to `print_element` call, holding the state specific to printing an element.
/// E.g. the `indent` depends on the token the Printer's currently processing. That's why
/// it must be stored outside of the [PrinterState] that stores the state common to all elements.
///
/// The state is passed by value, which is why it's important that it isn't storing any heavy
/// data structures. Such structures should be stored on the [PrinterState] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PrintElementArgs {
    indent: Indention,
    mode: PrintMode,
}

impl PrintElementArgs {
    pub fn new(indent: Indention) -> Self {
        Self {
            indent,
            ..Self::default()
        }
    }

    pub fn increment_indent_level(mut self, indent_style: IndentStyle) -> Self {
        self.indent = self.indent.increment_level(indent_style);
        self
    }

    pub fn decrement_indent(mut self) -> Self {
        self.indent = self.indent.decrement();
        self
    }

    pub fn reset_indent(mut self) -> Self {
        self.indent = Indention::default();
        self
    }

    pub fn set_indent_align(mut self, count: NonZeroU8) -> Self {
        self.indent = self.indent.set_align(count);
        self
    }

    pub fn with_print_mode(mut self, mode: PrintMode) -> Self {
        self.mode = mode;
        self
    }
}

impl Default for PrintElementArgs {
    fn default() -> Self {
        Self {
            indent: Indention::Level(0),
            mode: PrintMode::Expanded,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Indention {
    /// Indent the content by `count` levels by using the indention sequence specified by the printer options.
    Level(u16),

    /// Indent the content by n-`level`s using the indention sequence specified by the printer options and `align` spaces.
    Align { level: u16, align: NonZeroU8 },
}

impl Indention {
    const fn is_empty(&self) -> bool {
        matches!(self, Indention::Level(0))
    }

    /// Creates a new indention level with a zero-indent.
    const fn new() -> Self {
        Indention::Level(0)
    }

    /// Returns the indention level
    fn level(&self) -> u16 {
        match self {
            Indention::Level(count) => *count,
            Indention::Align { level: indent, .. } => *indent,
        }
    }

    /// Returns the number of trailing align spaces or 0 if none
    fn align(&self) -> u8 {
        match self {
            Indention::Level(_) => 0,
            Indention::Align { align, .. } => (*align).into(),
        }
    }

    /// Increments the level by one.
    ///
    /// The behaviour depends on the [`indent_style`][IndentStyle] if this is an [Indent::Align]:
    /// * **Tabs**: `align` is converted into an indent. This results in `level` increasing by two: once for the align, once for the level increment
    /// * **Spaces**: Increments the `level` by one and keeps the `align` unchanged.
    /// Keeps any  the current value is [Indent::Align] and increments the level by one.
    fn increment_level(self, indent_style: IndentStyle) -> Self {
        match self {
            Indention::Level(count) => Indention::Level(count + 1),
            // Increase the indent AND convert the align to an indent
            Indention::Align { level, .. } if indent_style.is_tab() => Indention::Level(level + 2),
            Indention::Align {
                level: indent,
                align,
            } => Indention::Align {
                level: indent + 1,
                align,
            },
        }
    }

    /// Decrements the indent by one by:
    /// * Reducing the level by one if this is [Indent::Level]
    /// * Removing the `align` if this is [Indent::Align]
    ///
    /// No-op if the level is already zero.
    fn decrement(self) -> Self {
        match self {
            Indention::Level(level) => Indention::Level(level.saturating_sub(1)),
            Indention::Align { level, .. } => Indention::Level(level),
        }
    }

    /// Adds an `align` of `count` spaces to the current indention.
    ///
    /// It increments the `level` value if the current value is [Indent::IndentAlign].
    fn set_align(self, count: NonZeroU8) -> Self {
        match self {
            Indention::Level(indent_count) => Indention::Align {
                level: indent_count,
                align: count,
            },

            // Convert the existing align to an indent
            Indention::Align { level: indent, .. } => Indention::Align {
                level: indent + 1,
                align: count,
            },
        }
    }
}

impl Default for Indention {
    fn default() -> Self {
        Indention::new()
    }
}

/// The Printer uses a stack that emulates recursion. E.g. recursively processing the elements:
/// `indent(&concat(string, string))` would result in the following call stack:
///
/// ```plain
/// print_element(indent, indent = 0);
///   print_element(concat, indent = 1);
///     print_element(string, indent = 1);
///     print_element(string, indent = 1);
/// ```
/// The `PrintElementCall` stores the data for a single `print_element` call consisting of the element
/// and the `args` that's passed to `print_element`.
#[derive(Debug, Eq, PartialEq, Clone)]
struct PrintElementCall<'element> {
    element: &'element FormatElement,
    args: PrintElementArgs,
}

impl<'element> PrintElementCall<'element> {
    pub fn new(element: &'element FormatElement, args: PrintElementArgs) -> Self {
        Self { element, args }
    }
}

/// Small helper that manages the order in which the elements should be visited.
#[derive(Debug, Default)]
struct ElementCallQueue<'a>(Vec<PrintElementCall<'a>>);

impl<'a> ElementCallQueue<'a> {
    pub fn new(elements: Vec<PrintElementCall<'a>>) -> Self {
        Self(elements)
    }

    fn extend<T>(&mut self, calls: T)
    where
        T: IntoIterator<Item = PrintElementCall<'a>>,
        T::IntoIter: DoubleEndedIterator,
    {
        // Reverse the calls because elements are removed from the back of the vec
        // in reversed insertion order
        self.0.extend(calls.into_iter().rev());
    }

    fn extend_with_args<I>(&mut self, elements: I, args: PrintElementArgs)
    where
        I: IntoIterator<Item = &'a FormatElement>,
        I::IntoIter: DoubleEndedIterator,
    {
        self.extend(
            elements
                .into_iter()
                .map(|element| PrintElementCall::new(element, args)),
        )
    }

    pub fn enqueue(&mut self, call: PrintElementCall<'a>) {
        self.0.push(call);
    }

    pub fn dequeue(&mut self) -> Option<PrintElementCall<'a>> {
        self.0.pop()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn into_vec(self) -> Vec<PrintElementCall<'a>> {
        self.0
    }
}

/// Tests if it's possible to print the content of the queue up to the first hard line break
/// or the end of the document on a single line without exceeding the line width.
#[must_use = "Only determines if content fits on a single line but doesn't print it"]
fn fits_on_line<'a, I>(
    elements: I,
    args: PrintElementArgs,
    queue: &ElementCallQueue<'a>,
    printer: &mut Printer<'a>,
) -> bool
where
    I: IntoIterator<Item = &'a FormatElement>,
    I::IntoIter: DoubleEndedIterator,
{
    let shared_buffer = std::mem::take(&mut printer.state.measure_queue);
    debug_assert!(shared_buffer.is_empty());

    let mut measure_queue = MeasureQueue::new()
        .with_rest(queue)
        .with_queue(ElementCallQueue::new(shared_buffer));

    measure_queue.extend(elements.into_iter(), args);

    let mut measure_state = MeasureState {
        pending_indent: printer.state.pending_indent,
        pending_space: printer.state.pending_space,
        line_width: printer.state.line_width,
        has_line_suffix: !printer.state.line_suffixes.is_empty(),
        group_modes: &mut printer.state.group_modes,
    };

    let result = loop {
        match measure_queue.dequeue() {
            None => {
                break true;
            }

            Some((element, args)) => match fits_element_on_line(
                element,
                args,
                &mut measure_state,
                &mut measure_queue,
                &printer.options,
            ) {
                Fits::Yes => {
                    break true;
                }
                Fits::No => {
                    break false;
                }
                Fits::Maybe => {
                    continue;
                }
            },
        }
    };

    let mut shared_buffer = measure_queue.into_vec();
    // Clear out remaining items
    shared_buffer.clear();
    printer.state.measure_queue = shared_buffer;

    result
}

/// Tests if the passed element fits on the current line or not.
fn fits_element_on_line<'a, 'rest>(
    element: &'a FormatElement,
    args: PrintElementArgs,
    state: &mut MeasureState,
    queue: &mut MeasureQueue<'a, 'rest>,
    options: &PrinterOptions,
) -> Fits {
    match element {
        FormatElement::Space => {
            if state.line_width > 0 {
                state.pending_space = true;
            }
        }

        FormatElement::Line(line_mode) => {
            if args.mode.is_flat() {
                match line_mode {
                    LineMode::SoftOrSpace => {
                        state.pending_space = true;
                    }
                    LineMode::Soft => {}
                    LineMode::Hard | LineMode::Empty => {
                        return Fits::No;
                    }
                }
            } else {
                // Reachable if the restQueue contains an element with mode expanded because Expanded
                // is what the mode's initialized to by default
                // This means, the printer is outside of the current element at this point and any
                // line break should be printed as regular line break -> Fits
                return Fits::Yes;
            }
        }

        FormatElement::Indent(content) => queue.extend(
            content.iter(),
            args.increment_indent_level(options.indent_style()),
        ),

        FormatElement::Dedent { content, mode } => {
            let args = match mode {
                DedentMode::Level => args.decrement_indent(),
                DedentMode::Root => args.reset_indent(),
            };
            queue.extend(content.iter(), args)
        }

        FormatElement::Align(Align { content, count }) => {
            queue.extend(content.iter(), args.set_indent_align(*count))
        }

        FormatElement::Group(group) => {
            queue.extend(group.content.iter(), args);

            if let Some(id) = group.id {
                state.group_modes.insert_print_mode(id, args.mode);
            }
        }

        FormatElement::ConditionalGroupContent(conditional) => {
            let group_mode = match conditional.group_id {
                None => args.mode,
                Some(group_id) => state
                    .group_modes
                    .get_print_mode(group_id)
                    .unwrap_or(args.mode),
            };

            if group_mode == conditional.mode {
                queue.extend(conditional.content.iter(), args);
            }
        }

        FormatElement::IndentIfGroupBreaks(indent) => {
            let group_mode = state
                .group_modes
                .get_print_mode(indent.group_id)
                .unwrap_or(args.mode);

            match group_mode {
                PrintMode::Flat => queue.extend(indent.content.iter(), args),
                PrintMode::Expanded => queue.extend(
                    indent.content.iter(),
                    args.increment_indent_level(options.indent_style()),
                ),
            }
        }

        FormatElement::List(list) => queue.extend(list.iter(), args),

        FormatElement::Fill(content) => queue
            .queue
            .0
            .extend(content.iter().rev().map(|t| PrintElementCall::new(t, args))),

        FormatElement::Text(token) => {
            let indent = std::mem::take(&mut state.pending_indent);
            state.line_width +=
                indent.level() as usize * options.indent_width() as usize + indent.align() as usize;

            if state.pending_space {
                state.line_width += 1;
            }

            for c in token.chars() {
                let char_width = match c {
                    '\t' => options.tab_width,
                    '\n' => {
                        return match args.mode {
                            PrintMode::Flat => Fits::No,
                            PrintMode::Expanded => Fits::Yes,
                        }
                    }
                    _ => 1,
                };
                state.line_width += char_width as usize;
            }

            if state.line_width > options.print_width.into() {
                return Fits::No;
            }

            state.pending_space = false;
        }

        FormatElement::LineSuffix(_) => {
            state.has_line_suffix = true;
        }

        FormatElement::LineSuffixBoundary => {
            if state.has_line_suffix {
                return Fits::No;
            }
        }

        FormatElement::Comment(content) => queue.extend(content.iter(), args),

        FormatElement::Verbatim(verbatim) => queue.extend(verbatim.content.iter(), args),
        FormatElement::BestFitting(best_fitting) => {
            let content = match args.mode {
                PrintMode::Flat => best_fitting.most_flat(),
                PrintMode::Expanded => best_fitting.most_expanded(),
            };

            queue.enqueue(PrintElementCall::new(content, args))
        }
        FormatElement::ExpandParent => {
            if args.mode.is_flat() {
                return Fits::No;
            }
        }
        FormatElement::Interned(content) => queue.enqueue(PrintElementCall::new(content, args)),
        FormatElement::Label(label) => queue.extend(label.content.iter(), args),
    }

    Fits::Maybe
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Fits {
    // Element fits
    Yes,
    // Element doesn't fit
    No,
    // Element may fit, depends on the elements following it
    Maybe,
}

impl From<bool> for Fits {
    fn from(value: bool) -> Self {
        match value {
            true => Fits::Yes,
            false => Fits::No,
        }
    }
}

/// State used when measuring if a group fits on a single line
#[derive(Debug)]
struct MeasureState<'group> {
    pending_indent: Indention,
    pending_space: bool,
    has_line_suffix: bool,
    line_width: usize,
    group_modes: &'group mut GroupModes,
}

#[derive(Debug)]
struct MeasureQueue<'a, 'rest> {
    /// Queue that holds the elements that the `fits` operation inspects.
    /// Normally, these all the elements belonging to the group that is tested if it fits
    queue: ElementCallQueue<'a>,
    /// Queue that contains the remaining elements in the documents.
    rest_queue: Rev<std::slice::Iter<'rest, PrintElementCall<'a>>>,
}

impl<'a, 'rest> MeasureQueue<'a, 'rest> {
    fn new() -> Self {
        Self {
            rest_queue: [].iter().rev(),
            queue: ElementCallQueue::default(),
        }
    }

    fn with_rest(mut self, rest_queue: &'rest ElementCallQueue<'a>) -> Self {
        // Last element in the vector is the first element in the queue
        self.rest_queue = rest_queue.0.as_slice().iter().rev();
        self
    }

    fn with_queue(mut self, queue: ElementCallQueue<'a>) -> Self {
        debug_assert!(queue.is_empty());
        self.queue = queue;
        self
    }

    fn enqueue(&mut self, call: PrintElementCall<'a>) {
        self.queue.enqueue(call);
    }

    fn extend<T>(&mut self, elements: T, args: PrintElementArgs)
    where
        T: IntoIterator<Item = &'a FormatElement>,
        T::IntoIter: DoubleEndedIterator,
    {
        // Reverse the calls because elements are removed from the back of the vec
        // in reversed insertion order
        self.queue.0.extend(
            elements
                .into_iter()
                .rev()
                .map(|element| PrintElementCall::new(element, args)),
        );
    }

    fn dequeue(&mut self) -> Option<(&'a FormatElement, PrintElementArgs)> {
        let next = match self.queue.dequeue() {
            Some(call) => (call.element, call.args),
            None => {
                let rest_item = self.rest_queue.next()?;

                (rest_item.element, rest_item.args)
            }
        };

        Some(next)
    }

    fn into_vec(self) -> Vec<PrintElementCall<'a>> {
        self.queue.into_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::printer::{LineEnding, PrintWidth, Printer, PrinterOptions};
    use crate::{format_args, write, FormatState, IndentStyle, Printed, VecBuffer};

    fn format(root: &dyn Format<()>) -> Printed {
        format_with_options(
            root,
            PrinterOptions {
                indent_style: IndentStyle::Space(2),
                ..PrinterOptions::default()
            },
        )
    }

    fn format_with_options(root: &dyn Format<()>, options: PrinterOptions) -> Printed {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);

        write!(&mut buffer, [root]).unwrap();

        Printer::new(options).print(&buffer.into_element())
    }

    #[test]
    fn it_prints_a_group_on_a_single_line_if_it_fits() {
        let result = format(&FormatArrayElements {
            items: vec![
                &text("\"a\""),
                &text("\"b\""),
                &text("\"c\""),
                &text("\"d\""),
            ],
        });

        assert_eq!(r#"["a", "b", "c", "d"]"#, result.as_code())
    }

    #[test]
    fn it_tracks_the_indent_for_each_token() {
        let formatted = format(&format_args!(
            text("a"),
            soft_block_indent(&format_args!(
                text("b"),
                soft_block_indent(&format_args!(
                    text("c"),
                    soft_block_indent(&format_args!(text("d"), soft_line_break(), text("d"),)),
                    text("c"),
                )),
                text("b"),
            )),
            text("a")
        ));

        assert_eq!(
            r#"a
  b
    c
      d
      d
    c
  b
a"#,
            formatted.as_code()
        )
    }

    #[test]
    fn it_converts_line_endings() {
        let options = PrinterOptions {
            line_ending: LineEnding::CarriageReturnLineFeed,
            ..PrinterOptions::default()
        };

        let result = format_with_options(
            &format_args![
                text("function main() {"),
                block_indent(&text("let x = `This is a multiline\nstring`;")),
                text("}"),
                hard_line_break()
            ],
            options,
        );

        assert_eq!(
            "function main() {\r\n\tlet x = `This is a multiline\r\nstring`;\r\n}\r\n",
            result.as_code()
        );
    }

    #[test]
    fn it_breaks_a_group_if_a_string_contains_a_newline() {
        let result = format(&FormatArrayElements {
            items: vec![
                &text("`This is a string spanning\ntwo lines`"),
                &text("\"b\""),
            ],
        });

        assert_eq!(
            r#"[
  `This is a string spanning
two lines`,
  "b",
]"#,
            result.as_code()
        )
    }
    #[test]
    fn it_breaks_a_group_if_it_contains_a_hard_line_break() {
        let result = format(&group(&format_args![text("a"), block_indent(&text("b"))]));

        assert_eq!("a\n  b\n", result.as_code())
    }

    #[test]
    fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
        let result = format(&FormatArrayElements {
            items: vec![
                &text("\"a\""),
                &text("\"b\""),
                &text("\"c\""),
                &text("\"d\""),
                &FormatArrayElements {
                    items: vec![
                        &text("\"0123456789\""),
                        &text("\"0123456789\""),
                        &text("\"0123456789\""),
                        &text("\"0123456789\""),
                        &text("\"0123456789\""),
                    ],
                },
            ],
        });

        assert_eq!(
            r#"[
  "a",
  "b",
  "c",
  "d",
  ["0123456789", "0123456789", "0123456789", "0123456789", "0123456789"],
]"#,
            result.as_code()
        );
    }

    #[test]
    fn it_use_the_indent_character_specified_in_the_options() {
        let options = PrinterOptions {
            indent_style: IndentStyle::Tab,
            tab_width: 4,
            print_width: PrintWidth::new(19),
            ..PrinterOptions::default()
        };

        let result = format_with_options(
            &FormatArrayElements {
                items: vec![&text("'a'"), &text("'b'"), &text("'c'"), &text("'d'")],
            },
            options,
        );

        assert_eq!("[\n\t'a',\n\t\'b',\n\t\'c',\n\t'd',\n]", result.as_code());
    }

    #[test]
    fn it_prints_consecutive_hard_lines_as_one() {
        let result = format(&format_args![
            text("a"),
            hard_line_break(),
            hard_line_break(),
            hard_line_break(),
            text("b"),
        ]);

        assert_eq!("a\nb", result.as_code())
    }

    #[test]
    fn it_prints_consecutive_empty_lines_as_one() {
        let result = format(&format_args![
            text("a"),
            empty_line(),
            empty_line(),
            empty_line(),
            text("b"),
        ]);

        assert_eq!("a\n\nb", result.as_code())
    }

    #[test]
    fn it_prints_consecutive_mixed_lines_as_one() {
        let result = format(&format_args![
            text("a"),
            empty_line(),
            hard_line_break(),
            empty_line(),
            hard_line_break(),
            text("b"),
        ]);

        assert_eq!("a\n\nb", result.as_code())
    }

    #[test]
    fn test_fill_breaks() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);
        let mut formatter = Formatter::new(&mut buffer);

        formatter
            .fill()
            // These all fit on the same line together
            .entry(
                &soft_line_break_or_space(),
                &format_args!(text("1"), text(",")),
            )
            .entry(
                &soft_line_break_or_space(),
                &format_args!(text("2"), text(",")),
            )
            .entry(
                &soft_line_break_or_space(),
                &format_args!(text("3"), text(",")),
            )
            // This one fits on a line by itself,
            .entry(
                &soft_line_break_or_space(),
                &format_args!(text("723493294"), text(",")),
            )
            // fits without breaking
            .entry(
                &soft_line_break_or_space(),
                &group(&format_args!(
                    text("["),
                    soft_block_indent(&text("5")),
                    text("],")
                )),
            )
            // this one must be printed in expanded mode to fit
            .entry(
                &soft_line_break_or_space(),
                &group(&format_args!(
                    text("["),
                    soft_block_indent(&text("123456789")),
                    text("]"),
                )),
            )
            .finish()
            .unwrap();

        let document = buffer.into_element();

        let printed = Printer::new(PrinterOptions::default().with_print_width(PrintWidth::new(10)))
            .print(&document);

        assert_eq!(
            printed.as_code(),
            "1, 2, 3,\n723493294,\n[5],\n[\n\t123456789\n]"
        )
    }

    #[test]
    fn line_suffix_printed_at_end() {
        let printed = format(&format_args![
            group(&format_args![
                text("["),
                soft_block_indent(&format_with(|f| {
                    f.fill()
                        .entry(
                            &soft_line_break_or_space(),
                            &format_args!(text("1"), text(",")),
                        )
                        .entry(
                            &soft_line_break_or_space(),
                            &format_args!(text("2"), text(",")),
                        )
                        .entry(
                            &soft_line_break_or_space(),
                            &format_args!(text("3"), if_group_breaks(&text(","))),
                        )
                        .finish()
                })),
                text("]")
            ]),
            text(";"),
            comment(&line_suffix(&format_args![
                space(),
                text("// trailing"),
                space()
            ]),)
        ]);

        assert_eq!(printed.as_code(), "[1, 2, 3]; // trailing")
    }

    #[test]
    fn conditional_with_group_id_in_fits() {
        let content = format_with(|f| {
            let group_id = f.group_id("test");
            write!(
                f,
                [
                    group(&format_args![
                        text("The referenced group breaks."),
                        hard_line_break()
                    ])
                    .with_group_id(Some(group_id)),
                    group(&format_args![
                        text("This group breaks because:"),
                        soft_line_break_or_space(),
                        if_group_fits_on_line(&text("This content fits but should not be printed.")).with_group_id(Some(group_id)),
                        if_group_breaks(&text("It measures with the 'if_group_breaks' variant because the referenced group breaks and that's just way too much text.")).with_group_id(Some(group_id)),
                    ])
                ]
            )
        });

        let printed = format(&content);

        assert_eq!(printed.as_code(), "The referenced group breaks.\nThis group breaks because:\nIt measures with the 'if_group_breaks' variant because the referenced group breaks and that's just way too much text.");
    }

    struct FormatArrayElements<'a> {
        items: Vec<&'a dyn Format<()>>,
    }

    impl Format<()> for FormatArrayElements<'_> {
        fn fmt(&self, f: &mut Formatter<()>) -> FormatResult<()> {
            write!(
                f,
                [group(&format_args!(
                    text("["),
                    soft_block_indent(&format_args!(
                        format_with(|f| f
                            .join_with(format_args!(text(","), soft_line_break_or_space()))
                            .entries(&self.items)
                            .finish()),
                        if_group_breaks(&text(",")),
                    )),
                    text("]")
                ))]
            )
        }
    }
}
