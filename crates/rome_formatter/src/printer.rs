use crate::format_element::{
    ConditionalGroupContent, Group, GroupPrintMode, LineMode, List, VerbatimKind,
};
use crate::intersperse::Intersperse;
use crate::{
    hard_line_break, space_token, FormatElement, FormatOptions, Formatted, IndentStyle,
    SourceMarker, TextRange,
};
use rome_rowan::TextSize;
use std::iter::once;

/// Options that affect how the [Printer] prints the format tokens
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrinterOptions {
    /// Width of a single tab character (does it equal 2, 4, ... spaces?)
    pub tab_width: u8,

    /// What's the max width of a line. Defaults to 80
    pub print_width: u16,

    /// The type of line ending to apply to the printed input
    pub line_ending: LineEnding,

    /// The never ending question whatever to use spaces or tabs, and if spaces, how many spaces
    /// to indent code.
    ///
    /// * Tab: Value is '\t'
    /// * Spaces: String containing the number of spaces per indention level, e.g. "  " for using two spaces
    pub indent_string: String,
}

impl From<FormatOptions> for PrinterOptions {
    fn from(options: FormatOptions) -> Self {
        let tab_width = 2;

        let indent_string = match options.indent_style {
            IndentStyle::Tab => String::from("\t"),
            IndentStyle::Space(width) => " ".repeat(width as usize),
        };

        PrinterOptions {
            indent_string,
            tab_width,
            print_width: options.line_width,
            ..PrinterOptions::default()
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    LineFeed,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    CarriageReturnLineFeed,

    /// Carriage Return character only (\r), used very rarely
    CarriageReturn,
}

impl LineEnding {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LineEnding::LineFeed => "\n",
            LineEnding::CarriageReturnLineFeed => "\r\n",
            LineEnding::CarriageReturn => "\r",
        }
    }
}

impl Default for PrinterOptions {
    fn default() -> Self {
        PrinterOptions {
            tab_width: 2,
            print_width: 80,
            indent_string: String::from("\t"),
            line_ending: LineEnding::LineFeed,
        }
    }
}

/// Error returned if printing an item as a flat string fails because it either contains
/// explicit line breaks or would otherwise exceed the specified line width.
struct LineBreakRequiredError;

/// Prints the format elements into a string
#[derive(Debug, Clone, Default)]
pub struct Printer<'a> {
    options: PrinterOptions,
    state: PrinterState<'a>,
}

impl<'a> Printer<'a> {
    pub fn new<T: Into<PrinterOptions>>(options: T) -> Self {
        Self {
            options: options.into(),
            state: PrinterState::default(),
        }
    }

    /// Prints the passed in element as well as all its content
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn print(self, element: &'a FormatElement) -> Formatted {
        self.print_with_indent(element, 0)
    }

    /// Prints the passed in element as well as all its content,
    /// starting at the specified indentation level
    pub fn print_with_indent(mut self, element: &'a FormatElement, indent: u16) -> Formatted {
        let mut queue = ElementCallQueue::new();

        queue.enqueue(PrintElementCall::new(
            element,
            PrintElementArgs {
                indent,
                hard_group: false,
            },
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

        Formatted::new(
            self.state.buffer,
            None,
            self.state.source_markers,
            self.state.verbatim_markers,
        )
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
            FormatElement::Empty => {}
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

                if let Some(range) = token.source() {
                    self.state.source_markers.push(SourceMarker {
                        source: range.start(),
                        dest: TextSize::from(self.state.buffer.len() as u32),
                    });
                }

                self.print_str(token);
            }

            FormatElement::HardGroup(group) => queue.enqueue(PrintElementCall::new(
                group.content.as_ref(),
                args.with_hard_group(true),
            )),
            FormatElement::Group(Group { content }) => {
                let args = args.with_hard_group(false);
                if self.try_print_flat(queue, element, args.clone()).is_err() {
                    // Flat printing didn't work, print with line breaks
                    queue.enqueue(PrintElementCall::new(content.as_ref(), args));
                }
            }

            FormatElement::Fill(list) => {
                self.print_fill(queue, list, args);
            }

            FormatElement::List(list) => {
                queue.extend(list.iter().map(|t| PrintElementCall::new(t, args.clone())));
            }

            FormatElement::Indent(indent) => {
                queue.enqueue(PrintElementCall::new(
                    &indent.content,
                    args.with_incremented_indent(),
                ));
            }

            FormatElement::ConditionalGroupContent(ConditionalGroupContent { mode, content }) => {
                if args.hard_group == matches!(mode, GroupPrintMode::Flat) {
                    queue.enqueue(PrintElementCall::new(content, args));
                }
            }

            FormatElement::Line(line) => {
                if args.hard_group && matches!(line.mode, LineMode::Soft | LineMode::SoftOrSpace) {
                    self.state.pending_space |= line.mode == LineMode::SoftOrSpace;
                } else if !self.state.line_suffixes.is_empty() {
                    // If the indentation level has changed since these line suffixes were queued,
                    // insert a line break before to push the comments into the new indent block
                    // SAFETY: Indexing into line_suffixes is guarded by the above call to is_empty
                    let has_line_break = self.state.line_suffixes[0].args.indent < args.indent;

                    // Print this line break element again once all the line suffixes have been flushed
                    let call_self = PrintElementCall::new(element, args.clone());

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
                                call.args = args.clone();
                                call
                            }))
                            .chain(once(call_self)),
                    );
                } else {
                    // Only print a newline if the current line isn't already empty
                    if self.state.line_width > 0 {
                        self.print_str("\n");
                    }

                    // Print a second line break if this is an empty line
                    if line.mode == LineMode::Empty && !self.state.has_empty_line {
                        self.print_str("\n");
                        self.state.has_empty_line = true;
                    }

                    self.state.pending_space = false;
                    self.state.pending_indent = args.indent;
                }
            }

            FormatElement::LineSuffix(suffix) => {
                self.state
                    .line_suffixes
                    .push(PrintElementCall::new(&**suffix, args));
            }
            FormatElement::Comment(content) => {
                queue.enqueue(PrintElementCall::new(content.as_ref(), args));
            }

            FormatElement::Verbatim(verbatim) => {
                if let VerbatimKind::Verbatim { range, text } = &verbatim.kind {
                    self.state.verbatim_markers.push((text.clone(), *range));
                }

                queue.enqueue(PrintElementCall::new(&verbatim.element, args));
            }
        }
    }

    /// Tries to print an element without any line breaks. Reverts any made `state` changes (by this function)
    /// and returns with a [LineBreakRequiredError] if the `element` contains any hard line breaks
    /// or printing the group exceeds the configured maximal print width.
    fn try_print_flat(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        element: &'a FormatElement,
        args: PrintElementArgs,
    ) -> Result<(), LineBreakRequiredError> {
        let snapshot = self.state.snapshot();
        let min_queue_length = queue.0.len();

        queue.enqueue(PrintElementCall::new(element, args));

        while let Some(call) = queue.dequeue() {
            if let Err(err) = self.try_print_flat_element(queue, call.element, call.args) {
                self.state.restore(snapshot);
                queue.0.truncate(min_queue_length);
                return Err(err);
            }

            if queue.0.len() == min_queue_length {
                break;
            }

            debug_assert!(queue.0.len() > min_queue_length);
        }

        Ok(())
    }

    fn try_print_flat_element(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        element: &'a FormatElement,
        args: PrintElementArgs,
    ) -> Result<(), LineBreakRequiredError> {
        match element {
            FormatElement::Token(_) => {
                let current_line = self.state.generated_line;

                // Delegate to generic string printing
                self.print_element(queue, element, args);

                // If the line is too long, break the group
                if self.state.line_width > self.options.print_width as usize {
                    return Err(LineBreakRequiredError);
                }

                // If a new line was printed, break the group
                if current_line != self.state.generated_line {
                    return Err(LineBreakRequiredError);
                }
            }
            FormatElement::Line(line) => {
                match line.mode {
                    LineMode::SoftOrSpace => {
                        if self.state.line_width > 0 {
                            self.state.pending_space = true;
                        }
                    }
                    // We want a flat structure, so omit soft line wraps
                    LineMode::Soft => {}
                    LineMode::Hard | LineMode::Empty => return Err(LineBreakRequiredError),
                }
            }

            FormatElement::HardGroup(group) => queue.enqueue(PrintElementCall::new(
                group.content.as_ref(),
                args.with_hard_group(true),
            )),
            FormatElement::Group(group) => queue.enqueue(PrintElementCall::new(
                group.content.as_ref(),
                args.with_hard_group(false),
            )),

            // Fill elements are printed as space-separated lists in flat mode
            FormatElement::Fill(list) => {
                // Intersperse the list of elements with spaces before pushing
                // them to the queue, however elements in the queue are stored
                // as references so the space element must be allocated in a
                // static so its reference is bound to the static lifetime
                static SPACE: FormatElement = space_token();
                queue.0.extend(
                    Intersperse::new(list.iter().rev(), &SPACE)
                        .map(|t| PrintElementCall::new(t, args.clone())),
                );
            }

            FormatElement::ConditionalGroupContent(ConditionalGroupContent {
                mode: GroupPrintMode::Flat,
                content,
            }) => queue.enqueue(PrintElementCall::new(content, args)),

            // Omit if there's no flat_contents
            FormatElement::ConditionalGroupContent(ConditionalGroupContent {
                mode: GroupPrintMode::Multiline,
                ..
            }) => {}

            FormatElement::Comment(content) => {
                queue.enqueue(PrintElementCall::new(content.as_ref(), args));
            }

            FormatElement::LineSuffix { .. } => return Err(LineBreakRequiredError),

            FormatElement::Empty
            | FormatElement::Space
            | FormatElement::Indent { .. }
            | FormatElement::Verbatim { .. }
            | FormatElement::List { .. } => self.print_element(queue, element, args),
        }

        Ok(())
    }

    /// Print a list in fill mode.
    ///
    /// Prints the elements of the list separated by spaces, but backtrack if
    /// they go over the print width and insert a line break before resuming
    /// printing
    fn print_fill(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        content: &'a List,
        args: PrintElementArgs,
    ) {
        let mut snapshot = None;

        for item in content.iter() {
            if snapshot.is_some() {
                self.state.pending_space = true;
            }

            self.print_all(queue, item, args.clone());

            if self.state.line_width > self.options.print_width.into() {
                if let Some(snapshot) = snapshot.take() {
                    self.state.restore(snapshot);

                    static LINE: FormatElement = hard_line_break();
                    self.print_all(queue, &LINE, args.clone());

                    self.print_all(queue, item, args.clone());
                }
            }

            snapshot = Some(self.state.snapshot());
        }
    }

    /// Fully print an element (print the element itself and all its descendants)
    ///
    /// Unlike [print_element], this function ensures the entire element has
    /// been printed when it returns and the queue is back to its original state
    fn print_all(
        &mut self,
        queue: &mut ElementCallQueue<'a>,
        element: &'a FormatElement,
        args: PrintElementArgs,
    ) {
        let min_queue_length = queue.0.len();

        queue.enqueue(PrintElementCall::new(element, args));

        while let Some(call) = queue.dequeue() {
            self.print_element(queue, call.element, call.args);

            if queue.0.len() == min_queue_length {
                return;
            }

            debug_assert!(queue.0.len() > min_queue_length);
        }
    }

    fn print_str(&mut self, content: &str) {
        self.state.buffer.reserve(content.len());

        for char in content.chars() {
            if char == '\n' {
                for char in self.options.line_ending.as_str().chars() {
                    self.state.buffer.push(char);
                }

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
#[derive(Default, Debug, Clone)]
struct PrinterState<'a> {
    buffer: String,
    source_markers: Vec<SourceMarker>,
    pending_indent: u16,
    pending_space: bool,
    generated_line: usize,
    generated_column: usize,
    line_width: usize,
    has_empty_line: bool,
    // mappings: Mapping[];
    line_suffixes: Vec<PrintElementCall<'a>>,
    verbatim_markers: Vec<(String, TextRange)>,
}

impl<'a> PrinterState<'a> {
    /// Allows creating a snapshot of the state that can be restored using [restore]
    pub fn snapshot(&self) -> PrinterStateSnapshot {
        PrinterStateSnapshot {
            pending_space: self.pending_space,
            pending_indents: self.pending_indent,
            generated_line: self.generated_line,
            generated_column: self.generated_column,
            line_width: self.line_width,
            has_empty_line: self.has_empty_line,
            buffer_position: self.buffer.len(),
            tokens_position: self.source_markers.len(),
            verbatim_markers: self.verbatim_markers.len(),
        }
    }

    /// Restores the printer state to the state stored in the snapshot.
    pub fn restore(&mut self, snapshot: PrinterStateSnapshot) {
        self.pending_space = snapshot.pending_space;
        self.pending_indent = snapshot.pending_indents;
        self.generated_column = snapshot.generated_column;
        self.generated_line = snapshot.generated_line;
        self.line_width = snapshot.line_width;
        self.has_empty_line = snapshot.has_empty_line;
        self.buffer.truncate(snapshot.buffer_position);
        self.source_markers.truncate(snapshot.tokens_position);
        self.verbatim_markers.truncate(snapshot.verbatim_markers);
    }
}

/// Snapshot of a printer state.
struct PrinterStateSnapshot {
    pending_indents: u16,
    pending_space: bool,
    generated_column: usize,
    generated_line: usize,
    line_width: usize,
    has_empty_line: bool,
    buffer_position: usize,
    tokens_position: usize,
    verbatim_markers: usize,
}

/// Stores arguments passed to `print_element` call, holding the state specific to printing an element.
/// E.g. the `indent` depends on the token the Printer's currently processing. That's why
/// it must be stored outside of the [PrinterState] that stores the state common to all elements.
///
/// The state is passed by value, which is why it's important that it isn't storing any heavy
/// data structures. Such structures should be stored on the [PrinterState] instead.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct PrintElementArgs {
    indent: u16,
    hard_group: bool,
}

impl PrintElementArgs {
    pub fn new(indent: u16) -> Self {
        Self {
            indent,
            hard_group: false,
        }
    }

    pub fn with_incremented_indent(self) -> Self {
        Self::new(self.indent + 1)
    }

    pub fn with_hard_group(self, hard_group: bool) -> Self {
        Self { hard_group, ..self }
    }
}

/// The Printer uses a stack that emulates recursion. E.g. recursively processing the elements:
/// `indent(concat(string, string))` would result in the following call stack:
///
/// ```plain
/// print_element(indent, indent = 0);
///   print_element(concat, indent = 1);
///     print_element(string, indent = 1);
///     print_element(string, indent = 1);
/// ```
/// The `PrintElementCall` stores the data for a single `print_element` call consisting of the element
/// and the `args` that's passed to `print_element`.
///
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
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn extend<T>(&mut self, calls: T)
    where
        T: IntoIterator<Item = PrintElementCall<'a>>,
        T::IntoIter: DoubleEndedIterator,
    {
        // Reverse the calls because elements are removed from the back of the vec
        // in reversed insertion order
        self.0.extend(calls.into_iter().rev());
    }

    #[inline]
    pub fn enqueue(&mut self, call: PrintElementCall<'a>) {
        self.0.push(call);
    }

    #[inline]
    pub fn dequeue(&mut self) -> Option<PrintElementCall<'a>> {
        self.0.pop()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::format_element::join_elements;
    use crate::printer::{LineEnding, Printer, PrinterOptions};
    use crate::{
        block_indent, empty_line, format_elements, group_elements, hard_line_break,
        if_group_breaks, soft_block_indent, soft_line_break, soft_line_break_or_space, token,
        FormatElement, Formatted,
    };

    /// Prints the given element with the default printer options
    fn print_element<T: Into<FormatElement>>(element: T) -> Formatted {
        let options = PrinterOptions {
            indent_string: String::from("  "),
            ..PrinterOptions::default()
        };

        Printer::new(options).print(&element.into())
    }

    #[test]
    fn it_prints_a_group_on_a_single_line_if_it_fits() {
        let result = print_element(create_array_element(vec![
            token("\"a\""),
            token("\"b\""),
            token("\"c\""),
            token("\"d\""),
        ]));

        assert_eq!(r#"["a", "b", "c", "d"]"#, result.as_code())
    }

    #[test]
    fn it_tracks_the_indent_for_each_token() {
        let element = format_elements![
            token("a"),
            soft_block_indent(format_elements![
                token("b"),
                soft_block_indent(format_elements![
                    token("c"),
                    soft_block_indent(format_elements![token("d"), soft_line_break(), token("d"),],),
                    token("c"),
                ],),
                token("b"),
            ],),
            token("a"),
        ];

        assert_eq!(
            r#"a
  b
    c
      d
      d
    c
  b
a"#,
            print_element(element).as_code()
        )
    }

    #[test]
    fn it_breaks_a_group_if_a_string_contains_a_newline() {
        let result = print_element(create_array_element(vec![
            token("`This is a string spanning\ntwo lines`"),
            token("\"b\""),
        ]));

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
    fn it_converts_line_endings_in_strings() {
        let options = PrinterOptions {
            line_ending: LineEnding::CarriageReturnLineFeed,
            ..PrinterOptions::default()
        };

        let program = format_elements![
            token("function main() {"),
            block_indent(token("let x = `This is a multiline\nstring`;"),),
            token("}"),
            hard_line_break(),
        ];

        let result = Printer::new(options).print(&program);

        assert_eq!(
            "function main() {\r\n\tlet x = `This is a multiline\r\nstring`;\r\n}\r\n",
            result.as_code()
        );
    }

    #[test]
    fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
        let result = print_element(create_array_element(vec![
            token("\"a\""),
            token("\"b\""),
            token("\"c\""),
            token("\"d\""),
            create_array_element(vec![
                token("\"0123456789\""),
                token("\"0123456789\""),
                token("\"0123456789\""),
                token("\"0123456789\""),
                token("\"0123456789\""),
            ]),
        ]));

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
        let printer = Printer::new(PrinterOptions {
            indent_string: String::from("\t"),
            tab_width: 4,
            print_width: 19,
            ..PrinterOptions::default()
        });

        let element =
            create_array_element(vec![token("'a'"), token("'b'"), token("'c'"), token("'d'")]);

        let result = printer.print(&element);

        assert_eq!("[\n\t'a',\n\t\'b',\n\t\'c',\n\t'd',\n]", result.as_code());
    }

    fn create_array_element(items: Vec<FormatElement>) -> FormatElement {
        let separator = format_elements![token(","), soft_line_break_or_space(),];

        let elements =
            format_elements![join_elements(separator, items), if_group_breaks(token(","))];

        group_elements(format_elements![
            token("["),
            soft_block_indent(elements),
            token("]"),
        ])
    }

    #[test]
    fn it_prints_consecutive_hard_lines_as_one() {
        let result = print_element(format_elements![
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
        let result = print_element(format_elements![
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
        let result = print_element(format_elements![
            token("a"),
            empty_line(),
            hard_line_break(),
            empty_line(),
            hard_line_break(),
            token("b"),
        ]);

        assert_eq!("a\n\nb", result.as_code())
    }
}
