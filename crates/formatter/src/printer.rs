use crate::LineMode;
use crate::{FormatOptions, FormatTokens, IndentStyle};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrintResult {
	code: String,
}

impl PrintResult {
	pub fn code(&self) -> &String {
		return &self.code;
	}
}

/// Error returned if printing an item as a flat string fails because it either contains
/// explicit line breaks or would otherwise exceed the specified line width.
struct LineBreakRequiredError;

/// Prints the format tokens into a string
#[derive(Debug, Clone, Default)]
pub struct Printer {
	options: PrinterOptions,
}

impl Printer {
	pub fn new<T: Into<PrinterOptions>>(options: T) -> Self {
		Self {
			options: options.into(),
		}
	}

	/// Prints the passed in token as well as all its contained tokens
	pub fn print(&self, token: &FormatTokens) -> PrintResult {
		let mut state = PrinterState::default();
		let mut queue: Vec<PrintTokenCall> =
			vec![PrintTokenCall::new(token, PrintTokenArgs::default())];

		while let Some(print_token_call) = queue.pop() {
			queue.extend(self.print_token(
				print_token_call.token,
				print_token_call.args,
				&mut state,
			));
		}

		PrintResult { code: state.buffer }
	}

	/// Prints a single token and returns the tokens to queue (that should be printed next).
	fn print_token<'a>(
		&self,
		token: &'a FormatTokens,
		args: PrintTokenArgs,
		state: &mut PrinterState,
	) -> Vec<PrintTokenCall<'a>> {
		match token {
			FormatTokens::Space => {
				state.pending_spaces += 1;
				vec![]
			}
			FormatTokens::StringLiteral(content) => {
				if content != "" {
					// Print pending indention
					if state.pending_indent > 0 {
						self.print_str(
							self.options
								.indent_string
								.repeat(state.pending_indent as usize)
								.as_str(),
							state,
						);
						state.pending_indent = 0;
					}

					// Print pending spaces
					if state.pending_spaces > 0 {
						self.print_str(" ".repeat(state.pending_spaces as usize).as_str(), state);
						state.pending_spaces = 0;
					}

					self.print_str(content.as_str(), state);
				}
				vec![]
			}

			FormatTokens::Group(group) if !group.should_break => {
				match self.try_print_flat(token, args.clone(), state) {
					Err(_) => {
						// Flat printing didn't work, print with line breaks
						vec![PrintTokenCall::new(group.content.as_ref(), args)]
					}
					Ok(_) => vec![],
				}
			}

			FormatTokens::Group(group) => {
				vec![PrintTokenCall::new(group.content.as_ref(), args)]
			}

			FormatTokens::List { content } => content
				.iter()
				.rev()
				.map(|t| PrintTokenCall::new(t, args.clone()))
				.collect(),

			FormatTokens::Indent { content } => {
				vec![PrintTokenCall::new(content, args.with_incremented_indent())]
			}

			FormatTokens::IfBreak { break_contents, .. } => {
				vec![PrintTokenCall::new(break_contents, args)]
			}

			FormatTokens::Line { .. } => {
				self.print_str("\n", state);
				state.pending_spaces = 0;
				state.pending_indent = args.indent;
				vec![]
			}
		}
	}

	/// Tries to print a token without any line breaks. Reverts any made `state` changes (by this function)
	/// and returns with a [LineBreakRequiredError] if the `token` contains any hard line breaks
	/// or printing the group exceeds the configured maximal print width.
	fn try_print_flat(
		&self,
		token: &FormatTokens,
		args: PrintTokenArgs,
		state: &mut PrinterState,
	) -> Result<(), LineBreakRequiredError> {
		let mut queue = vec![PrintTokenCall::new(token, args)];

		let snapshot = state.snapshot();

		while let Some(call) = queue.pop() {
			match self.try_print_flat_token(call.token, call.args, state) {
				Ok(to_queue) => queue.extend(to_queue),
				Err(err) => {
					state.restore(snapshot);
					return Err(err);
				}
			}
		}

		Ok(())
	}

	fn try_print_flat_token<'a>(
		&self,
		token: &'a FormatTokens,
		args: PrintTokenArgs,
		state: &mut PrinterState,
	) -> Result<Vec<PrintTokenCall<'a>>, LineBreakRequiredError> {
		let next_calls = match token {
			FormatTokens::StringLiteral(_) => {
				let current_line = state.generated_line;

				// Delegate to generic string printing
				let calls = self.print_token(token, args, state);

				// If the line is too long, break the group
				if state.line_width > self.options.print_width {
					return Err(LineBreakRequiredError);
				}

				// If a new line was printed, break the group
				if current_line != state.generated_line {
					return Err(LineBreakRequiredError);
				}

				calls
			}
			FormatTokens::Line { mode } => {
				match mode {
					LineMode::Space => {
						state.pending_spaces += 1;
						vec![]
					}
					// We want a flat structure, so omit soft line wraps
					LineMode::Soft => vec![],
					LineMode::Hard => return Err(LineBreakRequiredError {}),
				}
			}
			FormatTokens::Group(group) if group.should_break => {
				return Err(LineBreakRequiredError {})
			}

			FormatTokens::Group(group) => vec![PrintTokenCall::new(group.content.as_ref(), args)],

			FormatTokens::IfBreak {
				flat_contents: Some(content),
				..
			} => vec![PrintTokenCall::new(content, args)],

			// Omit if there's no flat_contents
			FormatTokens::IfBreak {
				flat_contents: None,
				..
			} => vec![],

			_ => self.print_token(token, args, state),
		};

		Ok(next_calls)
	}

	fn print_str(&self, content: &str, state: &mut PrinterState) -> () {
		for char in content.chars() {
			state.generated_index += 1;

			if char == '\n' {
				// TODO support different line endings?
				state.generated_line += 1;
				state.generated_column = 0;
				state.line_width = 0;
			} else {
				state.generated_column += 1;

				let char_width = if char == '\t' {
					self.options.tab_width as u16
				} else {
					1
				};

				state.line_width += char_width;
			}
		}

		state.buffer.push_str(content);
	}
}

/// Printer state that is global to all tokens.
/// Stores the result of the print operation (buffer and mappings) and at what
/// position the printer currently is.
#[derive(Default, Debug)]
struct PrinterState {
	buffer: String,
	pending_indent: u16,
	pending_spaces: u16,
	generated_index: usize,
	generated_line: u16,
	generated_column: u16,
	line_width: u16,
	// mappings: Mapping[];
	// We'll need to clone the line suffixes tokens into the state.
	// I guess that's fine. They're only used for comments and should, therefore, be very limited
	// in size.
	// lineSuffixes: [FormatTokens, PrintTokenArgs][];
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
	generated_column: u16,
	generated_line: u16,
	line_width: u16,
	buffer_position: usize,
}

/// Stores arguments passed to `print_token` call, holding the state specific to printing a token.
/// E.g. the `indent` depends on the token the Printer's currently processing. That's why
/// it must be stored outside of the [PrinterState] that stores the state common to all tokens.
///
/// The state is passed by value, which is why it's important that it isn't storing any heavy
/// data structures. Such structures should be stored on the [PrinterState] instead.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct PrintTokenArgs {
	indent: u16,
}

impl PrintTokenArgs {
	pub fn new(indent: u16) -> Self {
		Self { indent }
	}

	pub fn with_incremented_indent(self) -> Self {
		Self::new(self.indent + 1)
	}
}

/// The Printer uses a stack that emulates recursion. E.g. recursively processing the tokens:
/// `indent(concat(string, string))` would result in the following call stack:
///
/// ```plain
/// print_token(indent, indent = 0);
///   print_token(concat, indent = 1);
///     print_token(string, indent = 1);
///     print_token(string, indent = 1);
/// ```
/// The `PrintTokenCall` stores the data for a single `print_token` call consisting of the token
/// and the `args` that's passed to `print_token`.
///
#[derive(Debug, Eq, PartialEq, Clone)]
struct PrintTokenCall<'token> {
	token: &'token FormatTokens,
	args: PrintTokenArgs,
}

impl<'token> PrintTokenCall<'token> {
	pub fn new(token: &'token FormatTokens, state: PrintTokenArgs) -> Self {
		Self { token, args: state }
	}
}

/// Options that affect how the [Printer] prints the format tokens
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrinterOptions {
	/// Width of a single tab character (does it equal 2, 4, ... spaces?)
	pub tab_width: u8,

	/// What's the max width of a line. Defaults to 80
	pub print_width: u16,

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
			..PrinterOptions::default()
		}
	}
}

impl Default for PrinterOptions {
	fn default() -> Self {
		PrinterOptions {
			tab_width: 2,
			print_width: 80,
			indent_string: String::from("  "),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::printer::Printer;
	use crate::FormatTokens;

	#[test]
	fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
		let array = create_array_tokens(vec![
			FormatTokens::string("\"a\""),
			FormatTokens::string("\"b\""),
			FormatTokens::string("\"c\""),
			FormatTokens::string("\"d\""),
			create_array_tokens(vec![
				FormatTokens::string("\"0123456789\""),
				FormatTokens::string("\"0123456789\""),
				FormatTokens::string("\"0123456789\""),
				FormatTokens::string("\"0123456789\""),
				FormatTokens::string("\"0123456789\""),
			]),
		]);

		let printer = Printer::default();
		let result = printer.print(&array);

		assert_eq!("[\n  \"a\",\n  \"b\",\n  \"c\",\n  \"d\",\n  [\"0123456789\", \"0123456789\", \"0123456789\", \"0123456789\", \"0123456789\"],\n]", result.code);
	}

	fn create_array_tokens(items: Vec<FormatTokens>) -> FormatTokens {
		let separator = FormatTokens::concat(vec![
			FormatTokens::string(","),
			FormatTokens::new_line_or_space(),
		]);

		let elements = vec![
			FormatTokens::softline(),
			FormatTokens::join(separator, items),
			FormatTokens::IfBreak {
				break_contents: Box::new(FormatTokens::string(",")),
				flat_contents: None,
			},
		];

		FormatTokens::group(FormatTokens::concat(vec![
			FormatTokens::string("["),
			FormatTokens::indent(FormatTokens::concat(elements)),
			FormatTokens::softline(),
			FormatTokens::string("]"),
		]))
	}
}
