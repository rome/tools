mod cst_builder;
mod printer_state;
mod token_call_stack;

use crate::format_token::{IfBreakToken, TokenToken};
use crate::printer::cst_builder::ParentNodeId;
use crate::printer::printer_state::PrinterState;
use crate::printer::token_call_stack::{PrintTokenArgs, PrintTokenCall, TokenCallStack};
use crate::{FormatOptions, FormatToken, IndentStyle, Tokens};
use crate::{GroupToken, LineMode};
use rslint_parser::{GreenNode, SyntaxKind, SyntaxNode};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrintResult {
	root: SyntaxNode,
}

impl PrintResult {
	pub fn root(&self) -> &SyntaxNode {
		&self.root
	}
}

/// Error returned if printing an item as a flat string fails because it either contains
/// explicit line breaks or would otherwise exceed the specified line width.
struct LineBreakRequiredError;

/// Prints the format tokens into a string
#[derive(Debug, Default)]
pub struct Printer {
	options: PrinterOptions,
	state: PrinterState,
	tokens: Tokens,
}

impl Printer {
	pub fn new<T: Into<PrinterOptions>>(options: T) -> Self {
		Self {
			options: options.into(),
			..Printer::default()
		}
	}

	/// Prints the passed in token as well as all its contained tokens
	pub fn print(mut self, token: &FormatToken) -> PrintResult {
		let mut stack = TokenCallStack::default();

		let root_call = match token {
			FormatToken::Node(_) => PrintTokenCall::new(token, PrintTokenArgs::default()),
			_ => {
				// Ensure that there's always a root node.
				// Create an artificial root node and insert into the CST
				let root = create_green_node(SyntaxKind::SCRIPT);
				let root_pos = self.state.cst.append_node(ParentNodeId::root(), root);
				PrintTokenCall::new(token, PrintTokenArgs::default().with_parent_pos(root_pos))
			}
		};

		stack.enqueue(root_call);

		while let Some(print_token_call) = stack.dequeue() {
			stack.extend(self.print_token(print_token_call.token, print_token_call.args));
		}

		PrintResult {
			root: self.state.cst.root_node(),
		}
	}

	/// Prints a single token and returns the tokens to queue (that should be printed next).
	fn print_token<'a>(
		&mut self,
		token: &'a FormatToken,
		args: PrintTokenArgs,
	) -> Vec<PrintTokenCall<'a>> {
		match token {
			FormatToken::Space => {
				self.state.pending_spaces += 1;
				vec![]
			}
			FormatToken::Token(TokenToken { token }) => {
				// Print pending indention and spaces
				if self.state.pending_indent > 0 || self.state.pending_spaces > 0 {
					let whitespace = self
						.options
						.indent_string
						.repeat(self.state.pending_indent as usize)
						+ " ".repeat(self.state.pending_spaces as usize).as_str();

					self.state.cst.append_token(
						args.parent_id(),
						self.tokens.whitespace(whitespace.as_str()),
					);

					self.state.line_width += self.state.pending_spaces as usize
						+ self.state.pending_indent as usize * self.options.tab_width as usize;
					self.state.pending_spaces = 0;
					self.state.pending_indent = 0;
				}

				self.state.cst.append_token(args.parent_id(), token.clone());

				let token_size: usize = token.text_len().into();
				self.state.line_width += token_size;

				vec![]
			}

			FormatToken::Group(GroupToken {
				should_break: false,
				content,
			}) => {
				match self.try_print_flat(token, args.clone()) {
					Err(_) => {
						// Flat printing didn't work, print with line breaks
						vec![PrintTokenCall::new(content.as_ref(), args)]
					}
					Ok(_) => vec![],
				}
			}

			FormatToken::Group(group) => {
				vec![PrintTokenCall::new(group.content.as_ref(), args)]
			}

			FormatToken::List(list) => list
				.iter()
				.map(|t| PrintTokenCall::new(t, args.clone()))
				.collect(),

			FormatToken::Indent(indent) => {
				vec![PrintTokenCall::new(
					&indent.content,
					args.with_incremented_indent(),
				)]
			}

			FormatToken::IfBreak(if_break) => {
				vec![PrintTokenCall::new(&if_break.break_contents, args)]
			}

			FormatToken::Line { .. } => {
				self.state.cst.append_token(
					args.parent_id(),
					self.tokens.whitespace(self.options.line_ending.as_str()),
				);
				self.state.line_width = 0;
				self.state.pending_spaces = 0;
				self.state.pending_indent = args.indent();
				vec![]
			}

			FormatToken::Node(node) => {
				let node_pos = self
					.state
					.cst
					.append_node(args.parent_id(), node.node.clone());

				vec![PrintTokenCall::new(
					&node.content,
					args.with_parent_pos(node_pos),
				)]
			}

			FormatToken::RawNode(node) => {
				self.state
					.cst
					.append_raw_node(args.parent_id(), node.node.clone());
				vec![]
			}
		}
	}

	/// Tries to print a token without any line breaks. Reverts any made `state` changes (by this function)
	/// and returns with a [LineBreakRequiredError] if the `token` contains any hard line breaks
	/// or printing the group exceeds the configured maximal print width.
	fn try_print_flat(
		&mut self,
		token: &FormatToken,
		args: PrintTokenArgs,
	) -> Result<(), LineBreakRequiredError> {
		let snapshot = self.state.snapshot();

		let mut stack = TokenCallStack::new();
		stack.enqueue(PrintTokenCall::new(token, args));

		while let Some(call) = stack.dequeue() {
			match self.try_print_flat_token(call.token, call.args) {
				Ok(to_queue) => stack.extend(to_queue),
				Err(err) => {
					self.state.restore(snapshot);
					return Err(err);
				}
			}
		}

		Ok(())
	}

	fn try_print_flat_token<'a>(
		&mut self,
		token: &'a FormatToken,
		args: PrintTokenArgs,
	) -> Result<Vec<PrintTokenCall<'a>>, LineBreakRequiredError> {
		let next_calls = match token {
			FormatToken::Token(_) => {
				// Delegate to generic string printing
				let calls = self.print_token(token, args);

				// If the line is too long, break the group
				if self.state.line_width > self.options.print_width as usize {
					return Err(LineBreakRequiredError);
				}

				calls
			}
			FormatToken::Line(line) => {
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
			FormatToken::Group(GroupToken {
				should_break: true, ..
			}) => return Err(LineBreakRequiredError),

			FormatToken::Group(group) => vec![PrintTokenCall::new(group.content.as_ref(), args)],

			FormatToken::IfBreak(IfBreakToken {
				flat_contents: Some(content),
				..
			}) => vec![PrintTokenCall::new(content, args)],

			// Omit if there's no flat_contents
			FormatToken::IfBreak(_) => vec![],

			FormatToken::Space
			| FormatToken::Indent(_)
			| FormatToken::List(_)
			| FormatToken::RawNode(_)
			| FormatToken::Node(_) => self.print_token(token, args),
		};

		Ok(next_calls)
	}
}

#[inline]
fn create_green_node(kind: SyntaxKind) -> GreenNode {
	GreenNode::new(rslint_rowan::SyntaxKind(kind.into()), vec![])
}

#[cfg(test)]
mod tests {
	use crate::format_token::{
		GroupToken, IfBreakToken, IndentToken, LineToken, NodeToken, TokenToken,
	};
	use crate::printer::{create_green_node, LineEnding, PrintResult, Printer, PrinterOptions};
	use crate::{format_tokens, FormatToken, Tokens};
	use rslint_parser::SyntaxKind;

	#[test]
	fn it_prints_a_group_on_a_single_line_if_it_fits() {
		let mut tokens = Tokens::default();
		let items: Vec<FormatToken> = vec![
			tokens.get(SyntaxKind::NUMBER, "1").into(),
			tokens.get(SyntaxKind::NUMBER, "2").into(),
			tokens.get(SyntaxKind::NUMBER, "3").into(),
			tokens.get(SyntaxKind::NUMBER, "4").into(),
		];

		let array_expression = create_array_tokens(items, &mut tokens);

		assert_eq!("[1, 2, 3, 4]", print(array_expression).root().text());
	}

	#[test]
	fn it_breaks_parent_groups_if_they_dont_fit_on_a_single_line() {
		let mut tokens = Tokens::default();

		let result = print(create_array_tokens(
			vec![
				create_string("a", &mut tokens),
				create_string("b", &mut tokens),
				create_string("c", &mut tokens),
				create_string("d", &mut tokens),
				create_array_tokens(
					vec![
						create_string("0123456789", &mut tokens),
						create_string("0123456789", &mut tokens),
						create_string("0123456789", &mut tokens),
						create_string("0123456789", &mut tokens),
						create_string("0123456789", &mut tokens),
					],
					&mut tokens,
				),
			],
			&mut tokens,
		));

		assert_eq!(
			r#"[
	"a",
	"b",
	"c",
	"d",
	["0123456789", "0123456789", "0123456789", "0123456789", "0123456789"],
]"#,
			result.root().text()
		);
	}

	#[test]
	fn it_tracks_the_indent_for_each_token() {
		let mut tokens = Tokens::default();

		let root = format_tokens![
			create_number(0, &mut tokens),
			IndentToken::new(format_tokens![
				LineToken::soft(),
				create_number(1, &mut tokens),
				IndentToken::new(format_tokens![
					LineToken::soft(),
					create_number(2, &mut tokens),
					IndentToken::new(format_tokens![
						LineToken::soft(),
						create_number(3, &mut tokens),
						LineToken::soft(),
						create_number(3, &mut tokens)
					]),
					LineToken::soft(),
					create_number(2, &mut tokens),
				]),
				LineToken::soft(),
				create_number(1, &mut tokens),
			]),
			LineToken::soft(),
			create_number(0, &mut tokens),
		];

		assert_eq!(
			r#"0
	1
		2
			3
			3
		2
	1
0"#,
			print(root).root().text()
		)
	}

	#[test]
	fn it_converts_line_endings_in_strings() {
		let mut tokens = Tokens::default();

		let options = PrinterOptions {
			line_ending: LineEnding::CarriageReturnLineFeed,
			..printer_options()
		};

		let array = create_array_tokens(
			vec![
				create_string("abcd", &mut tokens),
				create_string("efgh", &mut tokens),
				create_string("ijkl", &mut tokens),
				create_string("mnop", &mut tokens),
				create_string("qrst", &mut tokens),
				create_string("uvwx", &mut tokens),
				create_string("yz01", &mut tokens),
				create_string("2345", &mut tokens),
				create_string("6789", &mut tokens),
				create_string("abcd", &mut tokens),
				create_string("ef", &mut tokens),
			],
			&mut tokens,
		);

		let result = Printer::new(options).print(&array);

		assert_eq!(
				"[\r\n\t\"abcd\",\r\n\t\"efgh\",\r\n\t\"ijkl\",\r\n\t\"mnop\",\r\n\t\"qrst\",\r\n\t\"uvwx\",\r\n\t\"yz01\",\r\n\t\"2345\",\r\n\t\"6789\",\r\n\t\"abcd\",\r\n\t\"ef\",\r\n]",
				result.root().text()
			);
	}

	#[test]
	fn it_use_the_indent_character_specified_in_the_options() {
		let mut tokens = Tokens::default();

		let printer = Printer::new(PrinterOptions {
			indent_string: String::from("    "),
			print_width: 19,
			..printer_options()
		});

		let result = printer.print(&create_array_tokens(
			vec![
				create_string("a", &mut tokens),
				create_string("b", &mut tokens),
				create_string("c", &mut tokens),
				create_string("d", &mut tokens),
			],
			&mut tokens,
		));

		assert_eq!(
			"[\n    \"a\",\n    \"b\",\n    \"c\",\n    \"d\",\n]",
			result.root().text()
		);
	}

	fn create_array_tokens(items: Vec<FormatToken>, tokens: &mut Tokens) -> FormatToken {
		let separator = format_tokens![tokens.comma(), LineToken::soft_or_space(),];

		let elements = format_tokens![
			LineToken::soft(),
			FormatToken::join(separator, items),
			IfBreakToken::new(tokens.comma())
		];

		GroupToken::new(format_tokens![
			tokens.left_bracket(),
			FormatToken::indent(elements),
			FormatToken::Line(LineToken::soft()),
			tokens.right_bracket(),
		])
		.into()
	}

	fn create_string(str: &str, tokens: &mut Tokens) -> FormatToken {
		NodeToken::new(
			create_green_node(SyntaxKind::STRING),
			TokenToken::new(tokens.double_quoted_string(str)),
		)
		.into()
	}

	fn create_number(num: u32, tokens: &mut Tokens) -> FormatToken {
		FormatToken::from(tokens.get(SyntaxKind::NUMBER, num.to_string().as_str()))
	}

	fn printer_options() -> PrinterOptions {
		PrinterOptions {
			line_ending: LineEnding::LineFeed,
			tab_width: 2,
			print_width: 80,
			indent_string: String::from("\t"),
		}
	}

	/// Prints the given token with a fixed set of options to ensure the tests are independent of the default printer options
	fn print<T: Into<FormatToken>>(token: T) -> PrintResult {
		Printer::new(printer_options()).print(&token.into())
	}
}
