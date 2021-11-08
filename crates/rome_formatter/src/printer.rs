use crate::format_element::{ConditionalGroupContent, Group, GroupPrintMode, LineMode};
use crate::{FormatElement, FormatOptions, Formatted, IndentStyle};

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
		let indent_string: String;
		let tab_width = 2;

		match options.indent_style {
			IndentStyle::Tab => indent_string = String::from("\t"),
			IndentStyle::Space(width) => indent_string = " ".repeat(width as usize),
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
pub struct Printer {
	options: PrinterOptions,
	state: PrinterState,
}

impl Printer {
	pub fn new<T: Into<PrinterOptions>>(options: T) -> Self {
		Self {
			options: options.into(),
			state: PrinterState::default(),
		}
	}

	/// Prints the passed in element as well as all its content
	pub fn print(mut self, element: &FormatElement) -> Formatted {
		let mut queue = ElementCallQueue::new();

		queue.enqueue(PrintElementCall::new(element, PrintElementArgs::default()));

		while let Some(print_element_call) = queue.dequeue() {
			queue.extend(self.print_element(print_element_call.element, print_element_call.args));
		}

		Formatted::new(self.state.buffer.as_str())
	}

	/// Prints a single element and returns the elements to queue (that should be printed next).
	fn print_element<'a>(
		&mut self,
		element: &'a FormatElement,
		args: PrintElementArgs,
	) -> Vec<PrintElementCall<'a>> {
		match element {
			FormatElement::Space => {
				self.state.pending_spaces += 1;
				vec![]
			}
			FormatElement::Empty => vec![],
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
				if self.state.pending_spaces > 0 {
					self.print_str(" ".repeat(self.state.pending_spaces as usize).as_str());
					self.state.pending_spaces = 0;
				}

				self.print_str(token);
				vec![]
			}

			FormatElement::Group(Group { content }) => {
				match self.try_print_flat(element, args.clone()) {
					Err(_) => {
						// Flat printing didn't work, print with line breaks
						vec![PrintElementCall::new(content.as_ref(), args)]
					}
					Ok(_) => vec![],
				}
			}

			FormatElement::List(list) => list
				.iter()
				.map(|t| PrintElementCall::new(t, args.clone()))
				.collect(),

			FormatElement::Indent(indent) => {
				vec![PrintElementCall::new(
					&indent.content,
					args.with_incremented_indent(),
				)]
			}

			FormatElement::ConditionalGroupContent(ConditionalGroupContent {
				mode: GroupPrintMode::Multiline,
				content,
			}) => {
				vec![PrintElementCall::new(content, args)]
			}

			FormatElement::ConditionalGroupContent(ConditionalGroupContent {
				mode: GroupPrintMode::Flat,
				..
			}) => {
				vec![]
			}

			FormatElement::Line { .. } => {
				self.print_str("\n");
				self.state.pending_spaces = 0;
				self.state.pending_indent = args.indent;
				vec![]
			}
		}
	}

	/// Tries to print an element without any line breaks. Reverts any made `state` changes (by this function)
	/// and returns with a [LineBreakRequiredError] if the `element` contains any hard line breaks
	/// or printing the group exceeds the configured maximal print width.
	fn try_print_flat(
		&mut self,
		element: &FormatElement,
		args: PrintElementArgs,
	) -> Result<(), LineBreakRequiredError> {
		let snapshot = self.state.snapshot();

		let mut queue = ElementCallQueue::new();
		queue.enqueue(PrintElementCall::new(element, args));

		while let Some(call) = queue.dequeue() {
			match self.try_print_flat_element(call.element, call.args) {
				Ok(to_queue) => queue.extend(to_queue),
				Err(err) => {
					self.state.restore(snapshot);
					return Err(err);
				}
			}
		}

		Ok(())
	}

	fn try_print_flat_element<'a>(
		&mut self,
		element: &'a FormatElement,
		args: PrintElementArgs,
	) -> Result<Vec<PrintElementCall<'a>>, LineBreakRequiredError> {
		let next_calls = match element {
			FormatElement::Token(_) => {
				let current_line = self.state.generated_line;

				// Delegate to generic string printing
				let calls = self.print_element(element, args);

				// If the line is too long, break the group
				if self.state.line_width > self.options.print_width as usize {
					return Err(LineBreakRequiredError);
				}

				// If a new line was printed, break the group
				if current_line != self.state.generated_line {
					return Err(LineBreakRequiredError);
				}

				calls
			}
			FormatElement::Line(line) => {
				match line.mode {
					LineMode::SoftOrSpace => {
						self.state.pending_spaces += 1;
						vec![]
					}
					// We want a flat structure, so omit soft line wraps
					LineMode::Soft => vec![],
					LineMode::Hard => return Err(LineBreakRequiredError),
				}
			}

			FormatElement::Group(group) => {
				vec![PrintElementCall::new(group.content.as_ref(), args)]
			}

			FormatElement::ConditionalGroupContent(ConditionalGroupContent {
				mode: GroupPrintMode::Flat,
				content,
			}) => vec![PrintElementCall::new(content, args)],

			// Omit if there's no flat_contents
			FormatElement::ConditionalGroupContent(ConditionalGroupContent {
				mode: GroupPrintMode::Multiline,
				..
			}) => vec![],

			FormatElement::Empty
			| FormatElement::Space
			| FormatElement::Indent { .. }
			| FormatElement::List { .. } => self.print_element(element, args),
		};

		Ok(next_calls)
	}

	fn print_str(&mut self, content: &str) {
		self.state.buffer.reserve(content.len());

		for char in content.chars() {
			if char == '\n' {
				for char in self.options.line_ending.as_str().chars() {
					self.state.generated_index += 1;
					self.state.buffer.push(char);
				}

				self.state.generated_line += 1;
				self.state.generated_column = 0;
				self.state.line_width = 0;
			} else {
				self.state.buffer.push(char);
				self.state.generated_index += 1;
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
}

/// Printer state that is global to all elements.
/// Stores the result of the print operation (buffer and mappings) and at what
/// position the printer currently is.
#[derive(Default, Debug, Clone)]
struct PrinterState {
	buffer: String,
	pending_indent: u16,
	pending_spaces: u16,
	generated_index: usize,
	generated_line: usize,
	generated_column: usize,
	line_width: usize,
	// mappings: Mapping[];
	// We'll need to clone the line suffixes elements into the state.
	// I guess that's fine. They're only used for comments and should, therefore, be very limited
	// in size.
	// lineSuffixes: [FormatElement, PrintElementArgs][];
}

impl PrinterState {
	/// Allows creating a snapshot of the state that can be restored using [restore]
	pub fn snapshot(&self) -> PrinterStateSnapshot {
		PrinterStateSnapshot {
			pending_spaces: self.pending_spaces,
			pending_indents: self.pending_indent,
			generated_index: self.generated_index,
			generated_line: self.generated_line,
			generated_column: self.generated_column,
			line_width: self.line_width,
			buffer_position: self.buffer.len(),
		}
	}

	/// Restores the printer state to the state stored in the snapshot.
	pub fn restore(&mut self, snapshot: PrinterStateSnapshot) {
		self.pending_spaces = snapshot.pending_spaces;
		self.pending_indent = snapshot.pending_indents;
		self.generated_index = snapshot.generated_index;
		self.generated_column = snapshot.generated_column;
		self.generated_line = snapshot.generated_line;
		self.line_width = snapshot.line_width;
		self.buffer.truncate(snapshot.buffer_position);
	}
}

/// Snapshot of a printer state.
struct PrinterStateSnapshot {
	pending_indents: u16,
	pending_spaces: u16,
	generated_index: usize,
	generated_column: usize,
	generated_line: usize,
	line_width: usize,
	buffer_position: usize,
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
}

impl PrintElementArgs {
	pub fn new(indent: u16) -> Self {
		Self { indent }
	}

	pub fn with_incremented_indent(self) -> Self {
		Self::new(self.indent + 1)
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
	fn extend(&mut self, calls: Vec<PrintElementCall<'a>>) {
		let mut calls = calls;
		// Reverse the calls because elements are removed from the back of the vec
		// in reversed insertion order
		calls.reverse();

		self.0.extend(calls);
	}

	#[inline]
	pub fn enqueue(&mut self, call: PrintElementCall<'a>) {
		self.0.push(call);
	}

	#[inline]
	pub fn dequeue(&mut self) -> Option<PrintElementCall<'a>> {
		self.0.pop()
	}
}

#[cfg(test)]
mod tests {
	use crate::format_element::join_elements;
	use crate::printer::{LineEnding, Printer, PrinterOptions};
	use crate::{
		block_indent, format_elements, group_elements, hard_line_break, if_group_breaks,
		soft_indent, soft_line_break, soft_line_break_or_space, token, FormatElement, Formatted,
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

		assert_eq!(r#"["a", "b", "c", "d"]"#, result.code())
	}

	#[test]
	fn it_tracks_the_indent_for_each_token() {
		let element = format_elements![
			token("a"),
			soft_indent(format_elements![
				token("b"),
				soft_indent(format_elements![
					token("c"),
					soft_indent(format_elements![token("d"), soft_line_break(), token("d"),],),
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
			print_element(element).code()
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
			result.code()
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
			result.code()
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
			result.code()
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

		let result = printer.print(&create_array_element(vec![
			token("'a'"),
			token("'b'"),
			token("'c'"),
			token("'d'"),
		]));

		assert_eq!("[\n\t'a',\n\t\'b',\n\t\'c',\n\t'd',\n]", result.code());
	}

	fn create_array_element(items: Vec<FormatElement>) -> FormatElement {
		let separator = format_elements![token(","), soft_line_break_or_space(),];

		let elements =
			format_elements![join_elements(separator, items), if_group_breaks(token(","))];

		group_elements(format_elements![
			token("["),
			soft_indent(elements),
			token("]"),
		])
	}
}
