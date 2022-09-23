mod call_stack;
mod line_suffixes;
mod printer_options;
mod queue;
mod stack;

pub use printer_options::*;

use crate::format_element::{BestFitting, LineMode, PrintMode};
use crate::{
    ActualStart, FormatElement, GroupId, IndentStyle, InvalidDocumentError, PrintError,
    PrintResult, Printed, SourceMarker, TextRange,
};

use crate::format_element::document::Document;
use crate::format_element::tag::Condition;
use crate::prelude::tag::{DedentMode, Tag, TagKind, VerbatimKind};
use crate::prelude::Tag::EndFill;
use crate::printer::call_stack::{
    CallStack, FitsCallStack, PrintCallStack, PrintElementArgs, StackFrame,
};
use crate::printer::line_suffixes::{LineSuffixEntry, LineSuffixes};
use crate::printer::queue::{
    AllPredicate, FitsPredicate, FitsQueue, PrintQueue, Queue, SeparatorItemPairPredicate,
    SingleEntryPredicate,
};
use rome_rowan::{TextLen, TextSize};
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
    pub fn print(self, document: &'a Document) -> PrintResult<Printed> {
        self.print_with_indent(document, 0)
    }

    /// Prints the passed in element as well as all its content,
    /// starting at the specified indentation level
    pub fn print_with_indent(
        mut self,
        document: &'a Document,
        indent: u16,
    ) -> PrintResult<Printed> {
        tracing::debug_span!("Printer::print").in_scope(move || {
            let mut stack = PrintCallStack::new(PrintElementArgs::new(Indention::Level(indent)));
            let mut queue: PrintQueue<'a> = PrintQueue::new(document.as_ref());

            while let Some(element) = queue.pop() {
                self.print_element(&mut stack, &mut queue, element)?;

                if queue.is_empty() {
                    self.flush_line_suffixes(&mut queue, &mut stack, None);
                }
            }

            Ok(Printed::new(
                self.state.buffer,
                None,
                self.state.source_markers,
                self.state.verbatim_markers,
            ))
        })
    }

    /// Prints a single element and push the following elements to queue
    fn print_element(
        &mut self,
        stack: &mut PrintCallStack,
        queue: &mut PrintQueue<'a>,
        element: &'a FormatElement,
    ) -> PrintResult<()> {
        use Tag::*;

        let args = stack.top();

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

            FormatElement::Line(line_mode) => {
                if args.mode().is_flat()
                    && matches!(line_mode, LineMode::Soft | LineMode::SoftOrSpace)
                {
                    if line_mode == &LineMode::SoftOrSpace && self.state.line_width > 0 {
                        self.state.pending_space = true;
                    }
                } else if self.state.line_suffixes.has_pending() {
                    self.flush_line_suffixes(queue, stack, Some(element));
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
                    self.state.pending_indent = args.indention();

                    // Fit's only tests if groups up to the first line break fit.
                    // The next group must re-measure if it still fits.
                    self.state.measured_group_fits = false;
                }
            }

            FormatElement::ExpandParent => {
                // No-op, only has an effect on `fits`
                debug_assert!(
                    !args.mode().is_flat(),
                    "Fits should always return false for `ExpandParent`"
                );
            }

            FormatElement::LineSuffixBoundary => {
                const HARD_BREAK: &FormatElement = &FormatElement::Line(LineMode::Hard);
                self.flush_line_suffixes(queue, stack, Some(HARD_BREAK));
            }

            FormatElement::BestFitting(best_fitting) => {
                self.print_best_fitting(best_fitting, queue, stack)?;
            }

            FormatElement::Interned(content) => {
                queue.extend_back(content);
            }

            FormatElement::Tag(StartGroup(id)) => {
                let group_mode = match args.mode() {
                    PrintMode::Flat if self.state.measured_group_fits => {
                        // A parent group has already verified that this group fits on a single line
                        // Thus, just continue in flat mode
                        stack.push(TagKind::Group, args);
                        PrintMode::Flat
                    }
                    // The printer is either in expanded mode or it's necessary to re-measure if the group fits
                    // because the printer printed a line break
                    _ => {
                        // Measure to see if the group fits up on a single line. If that's the case,
                        // print the group in "flat" mode, otherwise continue in expanded mode
                        stack.push(TagKind::Group, args.with_print_mode(PrintMode::Flat));
                        let fits = fits_on_line(AllPredicate, queue, stack, self)?;
                        stack.pop(TagKind::Group)?;

                        let mode = if fits {
                            self.state.measured_group_fits = true;
                            PrintMode::Flat
                        } else {
                            PrintMode::Expanded
                        };

                        stack.push(TagKind::Group, args.with_print_mode(mode));

                        mode
                    }
                };

                if let Some(id) = id {
                    self.state.group_modes.insert_print_mode(*id, group_mode);
                }
            }

            FormatElement::Tag(StartFill) => {
                stack.push(TagKind::Fill, args);
                self.print_fill_entries(queue, stack)?;
            }

            FormatElement::Tag(StartIndent) => {
                stack.push(
                    TagKind::Indent,
                    args.increment_indent_level(self.options.indent_style()),
                );
            }

            FormatElement::Tag(StartDedent(mode)) => {
                let args = match mode {
                    DedentMode::Level => args.decrement_indent(),
                    DedentMode::Root => args.reset_indent(),
                };
                stack.push(TagKind::Dedent, args);
            }

            FormatElement::Tag(StartAlign(align)) => {
                stack.push(TagKind::Align, args.set_indent_align(align.count()));
            }

            FormatElement::Tag(StartConditionalContent(Condition { mode, group_id })) => {
                let group_mode = match group_id {
                    None => args.mode(),
                    Some(id) => self.state.group_modes.unwrap_print_mode(*id, element),
                };

                if group_mode != *mode {
                    queue.skip_content(TagKind::ConditionalContent);
                } else {
                    stack.push(TagKind::ConditionalContent, args);
                }
            }

            FormatElement::Tag(StartIndentIfGroupBreaks(group_id)) => {
                let group_mode = self.state.group_modes.unwrap_print_mode(*group_id, element);

                let args = match group_mode {
                    PrintMode::Flat => args,
                    PrintMode::Expanded => args.increment_indent_level(self.options.indent_style),
                };

                stack.push(TagKind::IndentIfGroupBreaks, args);
            }

            FormatElement::Tag(StartLineSuffix) => {
                self.state
                    .line_suffixes
                    .extend(args, queue.iter_content(TagKind::LineSuffix));
            }

            FormatElement::Tag(StartVerbatim(kind)) => {
                if let VerbatimKind::Verbatim { length } = kind {
                    self.state.verbatim_markers.push(TextRange::at(
                        TextSize::from(self.state.buffer.len() as u32),
                        *length,
                    ));
                }

                stack.push(TagKind::Verbatim, args);
            }

            FormatElement::Tag(tag @ (StartLabelled(_) | StartEntry)) => {
                stack.push(tag.kind(), args);
            }
            FormatElement::Tag(
                tag @ (EndLabelled
                | EndEntry
                | EndGroup
                | EndIndent
                | EndDedent
                | EndAlign
                | EndConditionalContent
                | EndIndentIfGroupBreaks
                | EndVerbatim
                | EndLineSuffix
                | EndFill),
            ) => {
                stack.pop(tag.kind())?;
            }
        };

        Ok(())
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

    fn flush_line_suffixes(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        line_break: Option<&'a FormatElement>,
    ) {
        let suffixes = self.state.line_suffixes.take_pending();

        if suffixes.len() > 0 {
            // Print this line break element again once all the line suffixes have been flushed
            if let Some(line_break) = line_break {
                queue.push(line_break);
            }

            for entry in suffixes.rev() {
                match entry {
                    LineSuffixEntry::Suffix(suffix) => {
                        queue.push(suffix);
                    }
                    LineSuffixEntry::Args(args) => {
                        stack.push(TagKind::LineSuffix, args);
                        const LINE_SUFFIX_END: &FormatElement =
                            &FormatElement::Tag(Tag::EndLineSuffix);

                        queue.push(LINE_SUFFIX_END);
                    }
                }
            }
        }
    }

    fn print_best_fitting(
        &mut self,
        best_fitting: &'a BestFitting,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
    ) -> PrintResult<()> {
        let args = stack.top();

        if args.mode().is_flat() {
            queue.extend_back(best_fitting.most_flat());
            self.print_entry(queue, stack, args)
        } else {
            let normal_variants = &best_fitting.variants()[..best_fitting.variants().len() - 1];

            for (index, variant) in normal_variants.iter().enumerate() {
                // Test if this variant fits and if so, use it. Otherwise try the next
                // variant.

                // Try to fit only the first variant on a single line
                let mode = if index == 0 {
                    PrintMode::Flat
                } else {
                    PrintMode::Expanded
                };

                if !matches!(variant.first(), Some(&FormatElement::Tag(Tag::StartEntry))) {
                    return invalid_start_tag(TagKind::Entry, variant.first());
                }

                // Skip the first element because we want to override the args for the entry and the
                // args must be popped from the stack as soon as it sees the matching end entry.
                let content = &variant[1..];

                queue.extend_back(content);
                stack.push(TagKind::Entry, args.with_print_mode(mode));
                let variant_fits = fits_on_line(AllPredicate, queue, stack, self)?;
                stack.pop(TagKind::Entry)?;

                // Remove the content slice because printing needs the variant WITH the start entry
                let popped_slice = queue.pop_slice();
                debug_assert_eq!(popped_slice, Some(content));

                if variant_fits {
                    self.state.measured_group_fits = true;

                    queue.extend_back(variant);
                    return self.print_entry(queue, stack, args.with_print_mode(mode));
                }
            }

            // No variant fits, take the last (most expanded) as fallback
            let most_expanded = best_fitting.most_expanded();
            queue.extend_back(most_expanded);
            self.print_entry(queue, stack, args.with_print_mode(PrintMode::Expanded))
        }
    }

    /// Tries to fit as much content as possible on a single line.
    /// Each item forms a virtual group that is either printed in flat or expanded mode.
    /// It handles three different cases:
    ///
    /// * The first and second content fit on a single line. It prints the content and separator in flat mode.
    /// * The first content fits on a single line, but the second doesn't. It prints the content in flat and the separator in expanded mode.
    /// * Neither the first nor the second content fit on the line. It brings the first content and the separator in expanded mode.
    fn print_fill_entries(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
    ) -> PrintResult<()> {
        let args = stack.top();

        if matches!(queue.top(), Some(FormatElement::Tag(Tag::EndFill))) {
            // Empty fill
            return Ok(());
        }

        // Print the first item
        let mut current_fits = self.fits_fill_entry(
            SingleEntryPredicate::default(),
            queue,
            stack,
            PrintMode::Flat,
        )?;

        self.print_entry(
            queue,
            stack,
            args.with_print_mode(if current_fits {
                PrintMode::Flat
            } else {
                PrintMode::Expanded
            }),
        )?;

        // Process remaining items, it's a sequence of separator, item, separator, item...
        while matches!(queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
            // A line break in expanded mode is always necessary if the current item didn't fit.
            // otherwise see if both contents fit on the line.
            let all_fits = if current_fits {
                self.fits_fill_entry(
                    SeparatorItemPairPredicate::default(),
                    queue,
                    stack,
                    PrintMode::Flat,
                )?
            } else {
                false
            };

            let separator_mode = if all_fits {
                PrintMode::Flat
            } else {
                PrintMode::Expanded
            };

            // Separator
            self.print_entry(queue, stack, args.with_print_mode(separator_mode))?;

            // If this was a trailing separator, exit
            if !matches!(queue.top(), Some(FormatElement::Tag(Tag::StartEntry))) {
                break;
            }

            if all_fits {
                // Item
                self.print_entry(queue, stack, args.with_print_mode(PrintMode::Flat))?;
            } else {
                // Test if item fits now
                let next_fits = self.fits_fill_entry(
                    SingleEntryPredicate::default(),
                    queue,
                    stack,
                    PrintMode::Flat,
                )?;

                self.print_entry(
                    queue,
                    stack,
                    args.with_print_mode(if next_fits {
                        PrintMode::Flat
                    } else {
                        PrintMode::Expanded
                    }),
                )?;

                current_fits = next_fits;
            }
        }

        if queue.top() == Some(&FormatElement::Tag(EndFill)) {
            Ok(())
        } else {
            invalid_end_tag(TagKind::Fill, stack.top_kind())
        }
    }

    fn fits_fill_entry<P>(
        &mut self,
        predicate: P,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        mode: PrintMode,
    ) -> PrintResult<bool>
    where
        P: FitsPredicate,
    {
        let start_entry = queue.top();

        if !matches!(start_entry, Some(&FormatElement::Tag(Tag::StartEntry))) {
            return invalid_start_tag(TagKind::Entry, start_entry);
        }

        // Create a virtual group around each fill entry
        stack.push(TagKind::Group, stack.top().with_print_mode(mode));
        let fits = fits_on_line(predicate, queue, stack, self)?;
        stack.pop(TagKind::Group)?;

        Ok(fits)
    }

    /// Fully print an element (print the element itself and all its descendants)
    ///
    /// Unlike [print_element], this function ensures the entire element has
    /// been printed when it returns and the queue is back to its original state
    fn print_entry(
        &mut self,
        queue: &mut PrintQueue<'a>,
        stack: &mut PrintCallStack,
        args: PrintElementArgs,
    ) -> PrintResult<()> {
        let start_entry = queue.top();

        if !matches!(start_entry, Some(&FormatElement::Tag(Tag::StartEntry))) {
            return invalid_start_tag(TagKind::Entry, start_entry);
        }

        let mut depth = 0;

        while let Some(element) = queue.pop() {
            match element {
                FormatElement::Tag(Tag::StartEntry) => {
                    // Handle the start of the first element by pushing the args on the stack.
                    if depth == 0 {
                        depth = 1;
                        stack.push(TagKind::Entry, args);
                        continue;
                    }

                    depth += 1;
                }
                FormatElement::Tag(Tag::EndEntry) => {
                    depth -= 1;
                    // Reached the end entry, pop the entry from the stack and return.
                    if depth == 0 {
                        stack.pop(TagKind::Entry)?;
                        return Ok(());
                    }
                }
                _ => {
                    // Fall through
                }
            }

            self.print_element(stack, queue, element)?;
        }

        invalid_end_tag(TagKind::Entry, stack.top_kind())
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
    line_suffixes: LineSuffixes<'a>,
    verbatim_markers: Vec<TextRange>,
    group_modes: GroupModes,
    // Re-used queue to measure if a group fits. Optimisation to avoid re-allocating a new
    // vec everytime a group gets measured
    fits_stack: Vec<StackFrame>,
    fits_queue: Vec<&'a [FormatElement]>,
}

/// Tracks the mode in which groups with ids are printed. Stores the groups at `group.id()` index.
/// This is based on the assumption that the group ids for a single document are dense.
#[derive(Debug, Default)]
struct GroupModes(Vec<Option<PrintMode>>);

impl GroupModes {
    fn insert_print_mode(&mut self, group_id: GroupId, mode: PrintMode) {
        let index = u32::from(group_id) as usize;

        if self.0.len() <= index {
            self.0.resize(index + 1, None);
        }

        self.0[index] = Some(mode);
    }

    fn get_print_mode(&self, group_id: GroupId) -> Option<PrintMode> {
        let index = u32::from(group_id) as usize;
        self.0
            .get(index)
            .and_then(|option| option.as_ref().copied())
    }

    fn unwrap_print_mode(&self, group_id: GroupId, next_element: &FormatElement) -> PrintMode {
        self.get_print_mode(group_id).unwrap_or_else(|| {
            panic!("Expected group with id {group_id:?} to exist but it wasn't present in the document. Ensure that a group with such a document appears in the document before the element {next_element:?}.")
        })
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

/// Tests if it's possible to print the content of the queue up to the first hard line break
/// or the end of the document on a single line without exceeding the line width.
fn fits_on_line<'a, 'print, P>(
    predicate: P,
    print_queue: &'print PrintQueue<'a>,
    stack: &'print PrintCallStack,
    printer: &mut Printer<'a>,
) -> PrintResult<bool>
where
    P: FitsPredicate,
{
    let saved_stack = std::mem::take(&mut printer.state.fits_stack);
    let saved_queue = std::mem::take(&mut printer.state.fits_queue);
    debug_assert!(saved_stack.is_empty());
    debug_assert!(saved_queue.is_empty());

    let mut fits_queue = FitsQueue::new(print_queue, saved_queue);
    let mut fits_stack = FitsCallStack::new(stack, saved_stack);

    let mut fits_state = FitsState {
        pending_indent: printer.state.pending_indent,
        pending_space: printer.state.pending_space,
        line_width: printer.state.line_width,
        has_line_suffix: printer.state.line_suffixes.has_pending(),
        group_modes: &mut printer.state.group_modes,
    };

    let result = all_fit(
        predicate,
        &mut fits_state,
        &mut fits_queue,
        &mut fits_stack,
        &printer.options,
    );

    printer.state.fits_stack = fits_stack.finish();
    printer.state.fits_queue = fits_queue.finish();

    printer.state.fits_stack.clear();
    printer.state.fits_queue.clear();

    result.map(|fits| match fits {
        Fits::Maybe | Fits::Yes => true,
        Fits::No => false,
    })
}

/// Tests if it's possible to print the content of the queue up to the first hard line break
/// or the end of the document on a single line without exceeding the line width.
fn all_fit<'a, 'print, P>(
    mut predicate: P,
    fits_state: &mut FitsState,
    queue: &mut FitsQueue<'a, 'print>,
    stack: &mut FitsCallStack<'print>,
    options: &PrinterOptions,
) -> PrintResult<Fits>
where
    P: FitsPredicate,
{
    while let Some(element) = queue.pop() {
        if !predicate.apply(element)? {
            break;
        }

        match fits_element_on_line(element, fits_state, queue, stack, options)? {
            Fits::Yes => {
                return Ok(Fits::Yes);
            }
            Fits::No => {
                return Ok(Fits::No);
            }
            Fits::Maybe => {
                continue;
            }
        }
    }

    Ok(Fits::Maybe)
}

/// Tests if the passed element fits on the current line or not.
fn fits_element_on_line<'a, 'rest>(
    element: &'a FormatElement,
    state: &mut FitsState,
    queue: &mut FitsQueue<'a, 'rest>,
    stack: &mut FitsCallStack<'rest>,
    options: &PrinterOptions,
) -> PrintResult<Fits> {
    use Tag::*;

    let args = stack.top();

    match element {
        FormatElement::Space => {
            if state.line_width > 0 {
                state.pending_space = true;
            }
        }

        FormatElement::Line(line_mode) => {
            if args.mode().is_flat() {
                match line_mode {
                    LineMode::SoftOrSpace => {
                        state.pending_space = true;
                    }
                    LineMode::Soft => {}
                    LineMode::Hard | LineMode::Empty => {
                        return Ok(Fits::No);
                    }
                }
            } else {
                // Reachable if the restQueue contains an element with mode expanded because Expanded
                // is what the mode's initialized to by default
                // This means, the printer is outside of the current element at this point and any
                // line break should be printed as regular line break -> Fits
                return Ok(Fits::Yes);
            }
        }

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
                        return Ok(match args.mode() {
                            PrintMode::Flat => Fits::No,
                            PrintMode::Expanded => Fits::Yes,
                        })
                    }
                    _ => 1,
                };
                state.line_width += char_width as usize;
            }

            if state.line_width > options.print_width.into() {
                return Ok(Fits::No);
            }

            state.pending_space = false;
        }

        FormatElement::LineSuffixBoundary => {
            if state.has_line_suffix {
                return Ok(Fits::No);
            }
        }

        FormatElement::ExpandParent => {
            if args.mode().is_flat() {
                return Ok(Fits::No);
            }
        }

        FormatElement::BestFitting(best_fitting) => match args.mode() {
            PrintMode::Flat => {
                queue.extend_back(best_fitting.most_flat());
            }
            PrintMode::Expanded => queue.extend_back(best_fitting.most_expanded()),
        },

        FormatElement::Interned(content) => queue.extend_back(content),

        FormatElement::Tag(StartIndent) => {
            stack.push(
                TagKind::Indent,
                args.increment_indent_level(options.indent_style()),
            );
        }

        FormatElement::Tag(StartDedent(mode)) => {
            let args = match mode {
                DedentMode::Level => args.decrement_indent(),
                DedentMode::Root => args.reset_indent(),
            };
            stack.push(TagKind::Dedent, args);
        }

        FormatElement::Tag(StartAlign(align)) => {
            stack.push(TagKind::Align, args.set_indent_align(align.count()));
        }

        FormatElement::Tag(StartGroup(id)) => {
            stack.push(TagKind::Group, args);

            if let Some(id) = id {
                state.group_modes.insert_print_mode(*id, args.mode());
            }
        }

        FormatElement::Tag(StartConditionalContent(condition)) => {
            let group_mode = match condition.group_id {
                None => args.mode(),
                Some(group_id) => state
                    .group_modes
                    .get_print_mode(group_id)
                    .unwrap_or_else(|| args.mode()),
            };

            if group_mode != condition.mode {
                queue.skip_content(TagKind::ConditionalContent);
            } else {
                stack.push(TagKind::ConditionalContent, args);
            }
        }

        FormatElement::Tag(StartIndentIfGroupBreaks(id)) => {
            let group_mode = state
                .group_modes
                .get_print_mode(*id)
                .unwrap_or_else(|| args.mode());

            match group_mode {
                PrintMode::Flat => {
                    stack.push(TagKind::IndentIfGroupBreaks, args);
                }
                PrintMode::Expanded => {
                    stack.push(
                        TagKind::IndentIfGroupBreaks,
                        args.increment_indent_level(options.indent_style()),
                    );
                }
            }
        }

        FormatElement::Tag(StartLineSuffix) => {
            queue.skip_content(TagKind::LineSuffix);
            state.has_line_suffix = true;
        }

        FormatElement::Tag(EndLineSuffix) => {
            return invalid_end_tag(TagKind::LineSuffix, stack.top_kind());
        }

        FormatElement::Tag(
            tag @ (StartFill | StartVerbatim(_) | StartLabelled(_) | StartEntry),
        ) => {
            stack.push(tag.kind(), args);
        }
        FormatElement::Tag(
            tag @ (EndFill
            | EndVerbatim
            | EndLabelled
            | EndEntry
            | EndGroup
            | EndIndentIfGroupBreaks
            | EndConditionalContent
            | EndAlign
            | EndDedent
            | EndIndent),
        ) => {
            stack.pop(tag.kind())?;
        }
    }

    Ok(Fits::Maybe)
}

#[cold]
fn invalid_end_tag<R>(end_tag: TagKind, start_tag: Option<TagKind>) -> PrintResult<R> {
    Err(PrintError::InvalidDocument(match start_tag {
        None => InvalidDocumentError::StartTagMissing { kind: end_tag },
        Some(kind) => InvalidDocumentError::StartEndTagMismatch {
            start_kind: end_tag,
            end_kind: kind,
        },
    }))
}

#[cold]
fn invalid_start_tag<R>(expected: TagKind, actual: Option<&FormatElement>) -> PrintResult<R> {
    let start = match actual {
        None => ActualStart::EndOfDocument,
        Some(FormatElement::Tag(tag)) => {
            if tag.is_start() {
                ActualStart::Start(tag.kind())
            } else {
                ActualStart::End(tag.kind())
            }
        }
        Some(_) => ActualStart::Content,
    };

    Err(PrintError::InvalidDocument(
        InvalidDocumentError::ExpectedStart {
            actual: start,
            expected_start: expected,
        },
    ))
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
struct FitsState<'group> {
    pending_indent: Indention,
    pending_space: bool,
    has_line_suffix: bool,
    line_width: usize,
    group_modes: &'group mut GroupModes,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::printer::{LineEnding, PrintWidth, Printer, PrinterOptions};
    use crate::{format_args, write, Document, FormatState, IndentStyle, Printed, VecBuffer};

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

        Printer::new(options)
            .print(&Document::from(buffer.into_vec()))
            .expect("Document to be valid")
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

        let document = Document::from(buffer.into_vec());

        let printed = Printer::new(PrinterOptions::default().with_print_width(PrintWidth::new(10)))
            .print(&document)
            .unwrap();

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
            &line_suffix(&format_args![space(), text("// trailing"), space()])
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

    #[test]
    fn out_of_order_group_ids() {
        let content = format_with(|f| {
            let id_1 = f.group_id("id-1");
            let id_2 = f.group_id("id-2");

            write!(
                f,
                [
                    group(&text("Group with id-2")).with_group_id(Some(id_2)),
                    hard_line_break()
                ]
            )?;

            write!(
                f,
                [
                    group(&text("Group with id-1 does not fit on the line because it exceeds the line width of 80 characters by")).with_group_id(Some(id_1)),
                    hard_line_break()
                ]
            )?;

            write!(
                f,
                [
                    if_group_fits_on_line(&text("Group 2 fits")).with_group_id(Some(id_2)),
                    hard_line_break(),
                    if_group_breaks(&text("Group 1 breaks")).with_group_id(Some(id_1))
                ]
            )
        });

        let printed = format(&content);

        assert_eq!(
            printed.as_code(),
            r#"Group with id-2
Group with id-1 does not fit on the line because it exceeds the line width of 80 characters by
Group 2 fits
Group 1 breaks"#
        );
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
