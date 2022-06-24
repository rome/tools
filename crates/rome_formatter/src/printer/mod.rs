mod printer_options;

pub use printer_options::*;

use crate::format_element::{
    ConditionalGroupContent, Group, LineMode, List, PrintMode, VerbatimKind,
};
use crate::intersperse::Intersperse;
use crate::{FormatElement, GroupId, Printed, SourceMarker, TextRange};

use rome_rowan::TextSize;
use std::iter::{once, Rev};

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
                PrintElementArgs::new(indent),
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
            FormatElement::Token(token) => {
                // Print pending indention
                if self.state.pending_indent > 0 {
                    self.print_str(
                        self.options
                            .indent_string
                            .repeat(self.state.pending_indent as usize)
                            .as_str(),
                    );
                    self.state.pending_indent = 0;
                }

                // Print pending spaces
                if self.state.pending_space {
                    self.print_str(" ");
                    self.state.pending_space = false;
                }

                // Insert source map markers before and after the token
                //
                // If the token has source position informations the start marker
                // will use the start position of the original token, and the end
                // marker will use that position + the text length of the token
                //
                // If the token has no source position (was created by the formatter)
                // both the start and end marker will use the last known position
                // in the input source (from state.source_position)
                if let Some(source) = token.source_position() {
                    self.state.source_position = *source;
                }

                self.state.source_markers.push(SourceMarker {
                    source: self.state.source_position,
                    dest: TextSize::of(&self.state.buffer),
                });

                self.print_str(token);

                if token.source_position().is_some() {
                    self.state.source_position += TextSize::of(&**token);
                }

                self.state.source_markers.push(SourceMarker {
                    source: self.state.source_position,
                    dest: TextSize::of(&self.state.buffer),
                });
            }

            FormatElement::Group(Group { content, id }) => {
                let group_mode = match args.mode {
                    PrintMode::Flat if self.state.measured_group_fits => {
                        // A parent group has already verified that this group fits on a single line
                        // Thus, just continue in flat mode
                        queue.extend(
                            content
                                .iter()
                                .map(|element| PrintElementCall::new(element, args)),
                        );
                        PrintMode::Flat
                    }
                    // The printer is either in expanded mode or it's necessary to re-measure if the group fits
                    // because the printer printed a line break
                    _ => {
                        // Measure to see if the group fits up on a single line. If that's the case,
                        // print the group in "flat" mode, otherwise continue in expanded mode

                        let flat_args = args.with_print_mode(PrintMode::Flat);
                        if fits_on_line(content.iter(), flat_args, queue, self) {
                            queue.extend(
                                content.iter().map(|e| PrintElementCall::new(e, flat_args)),
                            );
                            self.state.measured_group_fits = true;
                            PrintMode::Flat
                        } else {
                            queue.extend(content.iter().map(|e| {
                                PrintElementCall::new(e, args.with_print_mode(PrintMode::Expanded))
                            }));
                            PrintMode::Expanded
                        }
                    }
                };

                if let Some(id) = id {
                    self.state.group_modes.insert_print_mode(*id, group_mode);
                }
            }

            FormatElement::Fill(fill) => {
                self.print_fill(queue, fill.list(), fill.separator(), args);
            }

            FormatElement::List(list) => {
                queue.extend(list.iter().map(|t| PrintElementCall::new(t, args)));
            }

            FormatElement::Indent(content) => {
                queue.enqueue(PrintElementCall::new(
                    content,
                    args.with_incremented_indent(),
                ));
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
                    queue.enqueue(PrintElementCall::new(content, args));
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
                    .push(PrintElementCall::new(&**suffix, args));
            }
            FormatElement::LineSuffixBoundary => {
                const HARD_BREAK: &FormatElement = &FormatElement::Line(LineMode::Hard);
                self.queue_line_suffixes(HARD_BREAK, args, queue);
            }

            FormatElement::Comment(content) => {
                queue.extend(content.iter().map(|e| PrintElementCall::new(e, args)))
            }

            FormatElement::Verbatim(verbatim) => {
                if let VerbatimKind::Verbatim { length } = &verbatim.kind {
                    self.state.verbatim_markers.push(TextRange::at(
                        TextSize::from(self.state.buffer.len() as u32),
                        *length,
                    ));
                }

                queue.enqueue(PrintElementCall::new(&verbatim.element, args));
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
        let has_line_break = self.state.line_suffixes[0].args.indent < args.indent;

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
                .chain(self.state.line_suffixes.drain(..).map(move |mut call| {
                    // Overwrite the arguments for the PrintElementCalls in the queue with the current arguments
                    call.args = args;
                    call
                }))
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
        content: &'a List,
        separator: &'a FormatElement,
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
        for next_item in items {
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
                    self.print_all(queue, &[next_item], args.with_print_mode(PrintMode::Flat));
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

        queue.extend(
            elements
                .iter()
                .map(|element| PrintElementCall::new(element, args)),
        );

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

            self.state.has_empty_line = false;
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
    pending_indent: u16,
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
    indent: u16,
    mode: PrintMode,
}

impl PrintElementArgs {
    pub fn new(indent: u16) -> Self {
        Self {
            indent,
            ..Self::default()
        }
    }

    pub fn with_incremented_indent(mut self) -> Self {
        self.indent += 1;
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
            indent: 0,
            mode: PrintMode::Expanded,
        }
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

    measure_queue.extend(
        elements
            .into_iter()
            .map(|element| PrintElementCall::new(element, args)),
    );

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

        FormatElement::Indent(content) => queue.enqueue(PrintElementCall::new(
            content,
            args.with_incremented_indent(),
        )),

        FormatElement::Group(group) => {
            queue.extend(group.content.iter().map(|e| PrintElementCall::new(e, args)));

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
                queue.enqueue(PrintElementCall::new(&conditional.content, args))
            }
        }

        FormatElement::List(list) => {
            queue.extend(list.iter().map(|t| PrintElementCall::new(t, args)))
        }

        FormatElement::Fill(fill) => queue.queue.0.extend(
            Intersperse::new(fill.list().iter().rev(), fill.separator())
                .map(|t| PrintElementCall::new(t, args)),
        ),

        FormatElement::Token(token) => {
            state.line_width += state.pending_indent as usize * options.indent_string.len();
            state.pending_indent = 0;

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

            if state.line_width > options.print_width.value().into() {
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

        FormatElement::Comment(content) => {
            queue.extend(content.iter().map(|e| PrintElementCall::new(e, args)))
        }

        FormatElement::Verbatim(verbatim) => {
            queue.enqueue(PrintElementCall::new(&verbatim.element, args))
        }
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
        FormatElement::Label(label) => queue.extend(
            label
                .content
                .iter()
                .map(|element| PrintElementCall::new(element, args)),
        ),
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
    pending_indent: u16,
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

    fn extend<T>(&mut self, calls: T)
    where
        T: IntoIterator<Item = PrintElementCall<'a>>,
        T::IntoIter: DoubleEndedIterator,
    {
        // Reverse the calls because elements are removed from the back of the vec
        // in reversed insertion order
        self.queue.0.extend(calls.into_iter().rev());
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
    use crate::printer::{LineEnding, Printer, PrinterOptions};
    use crate::{format_args, write, FormatState, LineWidth, Printed, VecBuffer};

    fn format(root: &dyn Format<()>) -> Printed {
        format_with_options(
            root,
            PrinterOptions {
                indent_string: String::from("  "),
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
                &token("\"a\""),
                &token("\"b\""),
                &token("\"c\""),
                &token("\"d\""),
            ],
        });

        assert_eq!(r#"["a", "b", "c", "d"]"#, result.as_code())
    }

    #[test]
    fn it_tracks_the_indent_for_each_token() {
        let formatted = format(&format_args!(
            token("a"),
            soft_block_indent(&format_args!(
                token("b"),
                soft_block_indent(&format_args!(
                    token("c"),
                    soft_block_indent(&format_args!(token("d"), soft_line_break(), token("d"),)),
                    token("c"),
                )),
                token("b"),
            )),
            token("a")
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
                token("function main() {"),
                block_indent(&token("let x = `This is a multiline\nstring`;")),
                token("}"),
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
                &token("`This is a string spanning\ntwo lines`"),
                &token("\"b\""),
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
        let result = format(&group_elements(&format_args![
            token("a"),
            block_indent(&token("b"))
        ]));

        assert_eq!("a\n  b\n", result.as_code())
    }

    #[test]
    fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
        let result = format(&FormatArrayElements {
            items: vec![
                &token("\"a\""),
                &token("\"b\""),
                &token("\"c\""),
                &token("\"d\""),
                &FormatArrayElements {
                    items: vec![
                        &token("\"0123456789\""),
                        &token("\"0123456789\""),
                        &token("\"0123456789\""),
                        &token("\"0123456789\""),
                        &token("\"0123456789\""),
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
            indent_string: String::from("\t"),
            tab_width: 4,
            print_width: LineWidth::try_from(19).unwrap(),
            ..PrinterOptions::default()
        };

        let result = format_with_options(
            &FormatArrayElements {
                items: vec![&token("'a'"), &token("'b'"), &token("'c'"), &token("'d'")],
            },
            options,
        );

        assert_eq!("[\n\t'a',\n\t\'b',\n\t\'c',\n\t'd',\n]", result.as_code());
    }

    #[test]
    fn it_prints_consecutive_hard_lines_as_one() {
        let result = format(&format_args![
            token("a"),
            hard_line_break(),
            hard_line_break(),
            hard_line_break(),
            token("b"),
        ]);

        assert_eq!("a\nb", result.as_code())
    }

    #[test]
    fn it_prints_consecutive_empty_lines_as_one() {
        let result = format(&format_args![
            token("a"),
            empty_line(),
            empty_line(),
            empty_line(),
            token("b"),
        ]);

        assert_eq!("a\n\nb", result.as_code())
    }

    #[test]
    fn it_prints_consecutive_mixed_lines_as_one() {
        let result = format(&format_args![
            token("a"),
            empty_line(),
            hard_line_break(),
            empty_line(),
            hard_line_break(),
            token("b"),
        ]);

        assert_eq!("a\n\nb", result.as_code())
    }

    #[test]
    fn test_fill_breaks() {
        let mut state = FormatState::new(());
        let mut buffer = VecBuffer::new(&mut state);
        let mut formatter = Formatter::new(&mut buffer);

        formatter
            .fill(&soft_line_break_or_space())
            // These all fit on the same line together
            .entry(&format_args!(token("1"), token(",")))
            .entry(&format_args!(token("2"), token(",")))
            .entry(&format_args!(token("3"), token(",")))
            // This one fits on a line by itself,
            .entry(&format_args!(token("723493294"), token(",")))
            // fits without breaking
            .entry(&group_elements(&format_args!(
                token("["),
                soft_block_indent(&token("5")),
                token("],")
            )))
            // this one must be printed in expanded mode to fit
            .entry(&group_elements(&format_args!(
                token("["),
                soft_block_indent(&token("123456789")),
                token("]"),
            )))
            .finish()
            .unwrap();

        let document = buffer.into_element();

        let printed = Printer::new(PrinterOptions::default().with_print_width(LineWidth(10)))
            .print(&document);

        assert_eq!(
            printed.as_code(),
            "1, 2, 3,\n723493294,\n[5],\n[\n\t123456789\n]"
        )
    }

    #[test]
    fn line_suffix_printed_at_end() {
        let printed = format(&format_args![
            group_elements(&format_args![
                token("["),
                soft_block_indent(&format_with(|f| {
                    f.fill(soft_line_break_or_space())
                        .entry(&format_args!(token("1"), token(",")))
                        .entry(&format_args!(token("2"), token(",")))
                        .entry(&format_args!(token("3"), if_group_breaks(&token(","))))
                        .finish()
                })),
                token("]")
            ]),
            token(";"),
            comment(&line_suffix(&format_args![
                space_token(),
                token("// trailing"),
                space_token()
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
                    group_elements(&format_args![
                        token("The referenced group breaks."),
                        hard_line_break()
                    ])
                    .with_group_id(Some(group_id)),
                    group_elements(&format_args![
                        token("This group breaks because:"),
                        soft_line_break_or_space(),
                        if_group_fits_on_line(&token("This content fits but should not be printed.")).with_group_id(Some(group_id)),
                        if_group_breaks(&token("It measures with the 'if_group_breaks' variant because the referenced group breaks and that's just way too much text.")).with_group_id(Some(group_id)),
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
                [group_elements(&format_args!(
                    token("["),
                    soft_block_indent(&format_args!(
                        format_with(|f| f
                            .join_with(format_args!(token(","), soft_line_break_or_space()))
                            .entries(&self.items)
                            .finish()),
                        if_group_breaks(&token(",")),
                    )),
                    token("]")
                ))]
            )
        }
    }
}
