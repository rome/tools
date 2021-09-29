use crate::printer::cst_builder::ParentNodeId;
use crate::FormatToken;

/// Stores arguments passed to `print_token` call, holding the state specific to printing a token.
/// E.g. the `indent` depends on the token the Printer's currently processing. That's why
/// it must be stored outside of the [PrinterState] that stores the state common to all tokens.
///
/// The state is passed by value, which is why it's important that it isn't storing any heavy
/// data structures. Such structures should be stored on the [PrinterState] instead.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct PrintTokenArgs {
	/// ID of the parent node to which a new token or node should be appended to.
	/// This information must be stored on the "stack" because the [Printer] only performs
	/// a pre-order traversal of the [FormatToken]s and, thus, can't explicitly call `finish_node` to
	/// terminate a opened node.
	///
	/// ## Example:
	///
	/// ```text
	/// node(program,
	///   node(array,      <- parent = 0
	///     token("["),    <- parent = 1
	///     token("5"),    <- parent = 1
	///     token("]"),    <- parent = 1
	///   ),
	///   string(          <- parent = 0
	///     token("'"),    <- parent = 2
	///     token("abcd"), <- parent = 2
	///     token("'"),    <- parent = 2
	/// )                  
	/// ```
	///
	/// Keeping track of the position allows the [Printer] to know exactly where the token must be
	/// inserted, especially in the situation when the current item is further up in the call-stack
	/// (e.g. string after the "]" token)
	parent_id: ParentNodeId,

	/// The indention level
	indent: u16,
}

impl PrintTokenArgs {
	#[inline]
	pub fn indent(&self) -> u16 {
		self.indent
	}

	#[inline]
	pub fn parent_id(&self) -> ParentNodeId {
		self.parent_id
	}

	#[must_use]
	pub fn with_incremented_indent(self) -> Self {
		Self {
			indent: self.indent + 1,
			..self
		}
	}

	#[must_use]
	pub fn with_parent_pos(self, parent_pos: ParentNodeId) -> Self {
		Self {
			parent_id: parent_pos,
			..self
		}
	}
}

/// The Printer uses a stack that emulates recursion. E.g. recursively processing the tokens:
/// `concat(indent(concat(string, string), string)` would result in the following call stack:
///
/// ```plain
/// print_token(concat, indent = 0);
///   print_token(indent, indent = 0);
///      print_token(concat, indent = 1);
///      print_token(string, indent = 1);
///      print_token(string, indent = 1);
///   print_token(string, indent = 0);
/// ```
/// The `PrintTokenCall` stores the data for a single `print_token` call consisting of the token
/// and the `args` that's passed to `print_token`.
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PrintTokenCall<'token> {
	pub token: &'token FormatToken,
	pub args: PrintTokenArgs,
}

impl<'token> PrintTokenCall<'token> {
	pub fn new(token: &'token FormatToken, args: PrintTokenArgs) -> Self {
		Self { token, args }
	}
}

/// Small helper that manages the order in which the tokens should be visited.
#[derive(Debug, Default)]
pub(crate) struct TokenCallStack<'a>(Vec<PrintTokenCall<'a>>);

impl<'a> TokenCallStack<'a> {
	#[inline]
	pub fn new() -> Self {
		Self(Vec::new())
	}

	#[inline]
	pub fn extend(&mut self, calls: Vec<PrintTokenCall<'a>>) {
		let mut calls = calls;
		// Reverse the calls because elements are removed from the back of the vec
		// in reversed insertion order
		calls.reverse();

		self.0.extend(calls);
	}

	#[inline]
	pub fn enqueue(&mut self, call: PrintTokenCall<'a>) {
		self.0.push(call);
	}

	#[inline]
	pub fn dequeue(&mut self) -> Option<PrintTokenCall<'a>> {
		self.0.pop()
	}
}
