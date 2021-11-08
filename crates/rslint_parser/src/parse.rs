//! Utilities for high level parsing of js code.

use crate::{
	ast::{Expr, Module, Script},
	*,
};
use rslint_errors::Severity;
use std::marker::PhantomData;

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct Parse<T> {
	root: SyntaxNode,
	errors: Vec<ParserError>,
	_ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
	fn clone(&self) -> Parse<T> {
		Parse {
			root: self.root.clone(),
			errors: self.errors.clone(),
			_ty: PhantomData,
		}
	}
}

impl<T> Parse<T> {
	pub fn new(root: SyntaxNode, errors: Vec<ParserError>) -> Parse<T> {
		Parse {
			root,
			errors,
			_ty: PhantomData,
		}
	}

	/// The syntax node represented by this Parse result
	///
	/// ```
	/// use rslint_parser::{parse_text, ast::IfStmt, SyntaxNodeExt, SyntaxKind, AstNode};
	///
	/// let parse = parse_text(
	/// "
	///     if (a > 5) {
	///         /* something */
	///     }
	/// ", 0);
	///
	/// // The first stmt in the root syntax node (Script) is the if statement.
	/// let if_stmt = parse.tree().items().first().unwrap();
	///
	/// assert_eq!(if_stmt.syntax().kind(), SyntaxKind::IF_STMT);
	/// ```
	pub fn syntax(&self) -> SyntaxNode {
		self.root.clone()
	}

	/// Get the errors which ocurred when parsing
	pub fn errors(&self) -> &[ParserError] {
		&*self.errors
	}
}

impl<T: AstNode> Parse<T> {
	/// Convert the result to an untyped SyntaxNode parse.
	#[allow(clippy::wrong_self_convention)]
	pub fn to_syntax(self) -> Parse<SyntaxNode> {
		Parse {
			root: self.root,
			errors: self.errors,
			_ty: PhantomData,
		}
	}

	/// Convert this parse result into a typed AST node.
	///
	/// # Panics
	/// Panics if the node represented by this parse result mismatches.
	pub fn tree(&self) -> T {
		self.try_tree().unwrap()
	}

	/// Try to convert this parse's untyped syntax node into an AST node.
	pub fn try_tree(&self) -> Option<T> {
		T::cast(self.syntax())
	}

	/// Convert this parse into a result
	pub fn ok(self) -> Result<T, Vec<ParserError>> {
		if !self.errors.iter().any(|d| d.severity == Severity::Error) {
			Ok(self.tree())
		} else {
			Err(self.errors)
		}
	}
}

/// Run the rslint_lexer lexer to turn source code into tokens and errors produced by the lexer
pub fn tokenize(text: &str, file_id: usize) -> (Vec<rslint_lexer::Token>, Vec<ParserError>) {
	let mut tokens = Vec::new();
	let mut errors = Vec::new();
	for (tok, error) in rslint_lexer::Lexer::from_str(text, file_id) {
		tokens.push(tok);
		if let Some(err) = error {
			errors.push(err)
		}
	}
	(tokens, errors)
}

fn parse_common(
	text: &str,
	file_id: usize,
	syntax: Syntax,
) -> (Vec<Event>, Vec<ParserError>, Vec<rslint_lexer::Token>) {
	let (tokens, mut errors) = tokenize(text, file_id);

	let tok_source = TokenSource::new(text, &tokens);

	let mut parser = crate::Parser::new(tok_source, file_id, syntax);
	crate::syntax::program::parse(&mut parser);

	let (events, p_errs) = parser.finish();
	errors.extend(p_errs);

	(events, errors, tokens)
}

/// Parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Script`](Script) with [`tree`](Parse::tree).
///
/// ```
/// use rslint_parser::{ast::BracketExpr, parse_text, AstNode, SyntaxToken, SyntaxNodeExt, util, SyntaxList};
///
/// let parse = parse_text("foo.bar[2]", 0);
/// // Parse returns a list, get the first statement
/// let stmt = parse.syntax().first_child().unwrap();
/// // The untyped syntax node of `foo.bar[2]`, the root node is `Script`.
/// let untyped_expr_node = stmt.first_child().unwrap();
///
/// // SyntaxNodes can be turned into a nice string representation.
/// println!("{:#?}", untyped_expr_node);
///
/// // You can then cast syntax nodes into a typed AST node.
/// let typed_ast_node = BracketExpr::cast(untyped_expr_node.first_child().unwrap()).unwrap();
///
/// // Everything on every ast node is optional because of error recovery.
/// let prop = dbg!(typed_ast_node.prop()).unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.tokens();
///
/// assert_eq!(&util::concat_tokens(&tokens), "foo.bar[2]")
/// ```
pub fn parse_text(text: &str, file_id: usize) -> Parse<Script> {
	let (events, errors, tokens) = parse_common(text, file_id, Syntax::default());
	let mut tree_sink = LosslessTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}

/// Lossly parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Script`](Script) with [`tree`](Parse::tree).
///
/// Unlike [`parse_text`], the final parse result includes no whitespace, it does however include errors.
///
/// Note however that the ranges and text of nodes still includes whitespace! Therefore you should trim text before rendering it.
/// The [`util`](crate::util) module has utility functions for dealing with this easily.
///
/// ```
/// use rslint_parser::{ast::BracketExpr, parse_text_lossy, AstNode, SyntaxToken, SyntaxNodeExt, util, SyntaxList};
///
/// let parse = parse_text_lossy("foo.bar[2]", 0);
/// let stmt = parse.syntax().first_child().unwrap();
/// // The untyped syntax node of `foo.bar[2]`, the root node is `Script`.
/// let untyped_expr_node = stmt.first_child().unwrap();
///
/// // SyntaxNodes can be turned into a nice string representation.
/// println!("{:#?}", untyped_expr_node);
///
/// // You can then cast syntax nodes into a typed AST node.
/// let typed_ast_node = BracketExpr::cast(untyped_expr_node.first_child().unwrap()).unwrap();
///
/// // Everything on every ast node is optional because of error recovery.
/// let prop = typed_ast_node.prop().unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.tokens();
///
/// // End result does not include whitespace because the parsing is lossy in this case
/// assert_eq!(&util::concat_tokens(&tokens), "foo.bar[2]")
/// ```
pub fn parse_text_lossy(text: &str, file_id: usize) -> Parse<Script> {
	let (events, errors, tokens) = parse_common(text, file_id, Syntax::default());
	let mut tree_sink = LossyTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}

/// Same as [`parse_text_lossy`] but configures the parser to parse an ECMAScript module instead of a Script
pub fn parse_module_lossy(text: &str, file_id: usize) -> Parse<Module> {
	let (events, errors, tokens) = parse_common(text, file_id, Syntax::default().module());
	let mut tree_sink = LossyTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}

/// Same as [`parse_text`] but configures the parser to parse an ECMAScript module instead of a script
pub fn parse_module(text: &str, file_id: usize) -> Parse<Module> {
	let (events, errors, tokens) = parse_common(text, file_id, Syntax::default().module());
	let mut tree_sink = LosslessTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}

/// Losslessly Parse text into an expression [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Expr`](Expr) with [`tree`](Parse::tree).
pub fn parse_expr(text: &str, file_id: usize) -> Parse<Expr> {
	let (tokens, mut errors) = tokenize(text, file_id);
	let tok_source = TokenSource::new(text, &tokens);
	let mut parser = crate::Parser::new(tok_source, file_id, Syntax::default());
	crate::syntax::expr::expr(&mut parser);
	let (events, p_diags) = parser.finish();
	errors.extend(p_diags);
	let mut tree_sink = LosslessTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}

pub fn parse_with_syntax(text: &str, file_id: usize, syntax: Syntax) -> Parse<()> {
	let (events, errors, tokens) = parse_common(text, file_id, syntax);
	let mut tree_sink = LosslessTreeSink::new(text, &tokens);
	crate::process(&mut tree_sink, events, errors);
	let (green, parse_errors) = tree_sink.finish();
	Parse::new(green, parse_errors)
}
