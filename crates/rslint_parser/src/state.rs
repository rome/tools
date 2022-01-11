use std::collections::HashMap;
use std::ops::Range;

/// State kept by the parser while parsing.
/// It is required for things such as strict mode or async functions
#[derive(Debug, PartialEq)]
pub struct ParserState {
	/// If false, object expressions are not allowed to be parsed
	/// inside an expression.
	///
	/// Also applies for object patterns
	pub allow_object_expr: bool,
	/// Whether `in` should be counted in a binary expression
	/// this is for `for...in` statements to prevent ambiguity.
	pub include_in: bool,
	/// Whether the parser is in an iteration statement and `continue` is allowed.
	pub continue_allowed: bool,
	/// Whether the parser is in an iteration or switch statement and
	/// `break` is allowed.
	pub break_allowed: bool,
	/// A list of labels for labelled statements used to report undefined label errors
	/// for break and continue, as well as duplicate labels
	pub labels: HashMap<String, Range<usize>>,
	/// Whether the parser is in a generator function like `function* a() {}`
	pub in_generator: bool,
	/// Whether the parser is inside of a function
	pub in_function: bool,
	/// Whatever the parser is inside of a constructor
	pub in_constructor: bool,
	/// Whether we potentially are in a place to parse an arrow expression
	pub potential_arrow_start: bool,
	/// Whether we are in an async function
	pub in_async: bool,
	/// Whether we are in strict mode code
	pub strict: Option<StrictMode>,
	/// Whether the code we are parsing is a module
	pub is_module: bool,
	/// The exported default item, used for checking duplicate defaults
	pub default_item: Option<Range<usize>>,
	/// If set, the parser reports bindings with identical names. The option stores the name of the
	/// node that disallows duplicate bindings, for example `let`, `const` or `import`.
	pub duplicate_binding_parent: Option<&'static str>,
	pub name_map: HashMap<String, Range<usize>>,
	/// Whether the parser is in a conditional expr (ternary expr)
	pub in_cond_expr: bool,
	pub in_case_cond: bool,
	pub(crate) no_recovery: bool,
	pub in_declare: bool,
	pub in_binding_list_for_signature: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictMode {
	Module,
	Explicit(Range<usize>),
	Class(Range<usize>),
}

impl Default for ParserState {
	fn default() -> Self {
		Self {
			allow_object_expr: true,
			include_in: true,
			continue_allowed: false,
			break_allowed: false,
			labels: HashMap::new(),
			in_generator: false,
			in_function: false,
			in_constructor: false,
			potential_arrow_start: false,
			in_async: false,
			strict: None,
			is_module: false,
			default_item: None,
			name_map: HashMap::with_capacity(3),
			duplicate_binding_parent: None,
			in_cond_expr: false,
			in_case_cond: false,
			no_recovery: false,
			in_declare: false,
			in_binding_list_for_signature: false,
		}
	}
}
