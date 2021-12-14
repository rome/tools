//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::binding::*;
use super::expr::{expr, EXPR_RECOVERY_SET, STARTS_EXPR};
use super::program::export_decl;
use super::typescript::*;
use super::util::check_label_use;
use crate::parser::{ParseNodeList, ParsedSyntax, ParserProgress};
use crate::parser::{RecoveryError, RecoveryResult};
use crate::syntax::assignment::{expression_to_assignment_pattern, AssignmentExprPrecedence};
use crate::syntax::class::{parse_class_declaration, parse_initializer_clause};
use crate::syntax::expr::{expr_or_assignment, parse_identifier};
use crate::syntax::function::{is_at_async_function, parse_function_declaration, LineBreak};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::expected_binding;
use crate::syntax::module::parse_import;
use crate::CompletedNodeOrMissingMarker::NodeMarker;
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::SyntaxFeature;
use crate::{SyntaxKind::*, *};
use rslint_errors::Span;

pub const STMT_RECOVERY_SET: TokenSet = token_set![
	L_CURLY,
	VAR_KW,
	FUNCTION_KW,
	IF_KW,
	FOR_KW,
	DO_KW,
	WHILE_KW,
	CONTINUE_KW,
	BREAK_KW,
	RETURN_KW,
	WITH_KW,
	SWITCH_KW,
	THROW_KW,
	TRY_KW,
	DEBUGGER_KW,
	FUNCTION_KW,
	CLASS_KW,
	IMPORT_KW,
	EXPORT_KW,
	T![;]
];

pub const FOLLOWS_LET: TokenSet = token_set![T!['{'], T!['['], T![ident], T![yield], T![await]];

/// Consume an explicit semicolon, or try to automatically insert one,
/// or add an error to the parser if there was none and it could not be inserted
// test semicolons
// let foo = bar;
// let foo = b;
// let foo;
// let foo
// let foo
// function foo() { return true }
pub fn semi(p: &mut Parser, err_range: Range<usize>) -> bool {
	// test_err semicolons_err
	// let foo = bar throw foo

	if !optional_semi(p) {
		let err = p
			.err_builder(
				"Expected a semicolon or an implicit semicolon after a statement, but found none",
			)
			.primary(
				p.cur_tok().range,
				"An explicit or implicit semicolon is expected here...",
			)
			.secondary(err_range, "...Which is required to end this statement");

		p.error(err);
		false
	} else {
		true
	}
}

/// Eats a semicolon if present but doesn't add an error none is present and the automatic
/// semicolon insertion rule does not apply.
///
/// Returns false if neither a semicolon was present and the current position doesn't allow an automatic
/// semicolon insertion.
pub(crate) fn optional_semi(p: &mut Parser) -> bool {
	if p.eat_optional(T![;]) {
		return true;
	}

	is_semi(p, 0)
}

pub(super) fn is_semi(p: &Parser, offset: usize) -> bool {
	p.nth_at(offset, T![;])
		|| p.nth_at(offset, EOF)
		|| p.nth_at(offset, T!['}'])
		|| p.has_linebreak_before_n(offset)
}

/// A generic statement such as a block, if, while, with, etc
///
/// Error handling and recovering happens inside this function, so the
/// caller has to pass a recovery set.
///
/// If not passed, [STMT_RECOVERY_SET] will be used as recovery set
// TODO: change return type to `ParsedSyntax` once `expr_or_assignment` returns `ParsedSyntax`
// TODO: move error recovery to callers once `expr_with_semi_recovery` is refactored/gone
pub fn parse_statement(
	p: &mut Parser,
	recovery_set: impl Into<Option<TokenSet>>,
) -> Option<CompletedMarker> {
	let res = match p.cur() {
		// test_err import_decl_not_top_level
		// {
		//  import foo from "bar";
		// }

		// make sure we dont try parsing import.meta or import() as declarations
		T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
			let mut import = parse_import(p).unwrap();
			import.change_kind(p, JS_UNKNOWN_STATEMENT);

			if p.syntax.file_kind == FileKind::Script {
				let err = p
					.err_builder("Illegal use of an import declaration outside of a module")
					.primary(import.range(p), "not allowed inside scripts");

				p.error(err);
			} else {
				let err = p
					.err_builder("Illegal use of an import declaration not at the top level")
					.primary(import.range(p), "move this declaration to the top level");

				p.error(err);
			}
			Present(import)
		}
		// test_err export_decl_not_top_level
		// {
		//  export { pain } from "life";
		// }
		T![export] => {
			let mut m = export_decl(p);
			if !p.state.is_module && !p.typescript() {
				let err = p
					.err_builder("Illegal use of an export declaration outside of a module")
					.primary(m.range(p), "not allowed inside scripts");

				p.error(err);
			} else {
				let err = p
					.err_builder("Illegal use of an import declaration not at the top level")
					.primary(m.range(p), "move this declaration to the top level");

				p.error(err);
			}
			m.change_kind(p, JS_UNKNOWN_STATEMENT);
			Present(m)
		}
		T![;] => parse_empty_statement(p), // It is only ever Err if there's no ;
		T!['{'] => parse_block_stmt(p),    // It is only ever None if there is no `{`,
		T![if] => parse_if_statement(p),   // It is only ever Err if there's no if
		T![with] => parse_with_statement(p), // ever only Err if there's no with keyword
		T![while] => parse_while_statement(p), // It is only ever Err if there's no while keyword
		t if (t == T![const] && p.nth_at(1, T![enum])) || t == T![enum] => {
			let mut res = ts_enum(p);
			res.err_if_not_ts(p, "enums can only be declared in TypeScript files");
			Present(res)
		}
		T![var] | T![const] => variable_declaration_statement(p),
		T![for] => parse_for_statement(p),
		T![do] => parse_do_statement(p),
		T![switch] => parse_switch_statement(p),
		T![try] => parse_try_statement(p), // it is only ever Err if there's no try
		T![return] => parse_return_statement(p),
		T![break] => parse_break_statement(p),
		T![continue] => parse_continue_statement(p), // It is only ever Err if there's no continue keyword
		T![throw] => parse_throw_statement(p),
		T![debugger] => parse_debugger_statement(p),
		T![function] => {
			parse_function_declaration(p).or_invalid_to_unknown(p, JS_UNKNOWN_STATEMENT)
		}
		T![class] => parse_class_declaration(p),
		T![ident] if is_at_async_function(p, LineBreak::DoCheck) => {
			parse_function_declaration(p).or_invalid_to_unknown(p, JS_UNKNOWN_STATEMENT)
		}

		T![ident] if p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)) => {
			variable_declaration_statement(p)
		}
		// TODO: handle `<T>() => {};` with less of a hack
		_ if p.at_ts(STARTS_EXPR) || p.at(T![<]) => parse_expression_statement(p),
		_ => Absent,
	};

	match res {
		Absent => {
			// We must explicitly handle this case or else infinite recursion can happen
			if p.at_ts(token_set![T!['}'], T![export]]) {
				let err = p
					.err_builder("Expected a statement or declaration, but found none")
					.primary(
						p.cur_tok().range,
						"Expected a statement or declaration here",
					);
				p.err_and_bump(err, JS_UNKNOWN_STATEMENT);
			}
			res.or_recover(
				p,
				&ParseRecovery::new(
					JS_UNKNOWN_STATEMENT,
					recovery_set.into().unwrap_or(STMT_RECOVERY_SET),
				),
				js_parse_error::expected_statement,
			)
			.ok()
		}
		Present(marker) => Some(marker),
	}
}

// test_err double_label
// label1: {
// 	label2: {
// 		label1: {}
// 	}
// }

#[allow(deprecated)]
fn parse_expression_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	let start = p.cur_tok().range.start;
	// this is *technically* wrong because it would be an expr stmt in js but for our purposes
	// we treat these as always being ts declarations since ambiguity is inefficient in this style of
	// parsing and it results in better errors usually
	if matches!(
		p.cur_src(),
		"declare" | "abstract" | "enum" | "interface" | "namespace" | "type"
	) && !p.nth_at(1, T![:])
		&& !p.nth_at(1, T![.])
	{
		if let Some(mut res) = try_parse_ts(p, ts_expr_stmt) {
			res.err_if_not_ts(
				p,
				"TypeScript declarations can only be used in TypeScript files",
			);
			return Present(res);
		}
	}

	// module and global are special because its used normally in js a lot so we cant assume its a ts module decl
	if p.cur_src() == "module" || (p.cur_src() == "global" && p.nth_at(1, T!['{'])) {
		if let Some(mut res) = try_parse_ts(p, ts_expr_stmt) {
			res.err_if_not_ts(
				p,
				"TypeScript declarations can only be used in TypeScript files",
			);
			return Present(res);
		}
	}
	if p.typescript()
		&& matches!(p.cur_src(), "public" | "private" | "protected")
		&& p.nth_src(1) == "interface"
	{
		let err = p
			.err_builder("interface declarations cannot have accessibility modifiers")
			.primary(p.cur_tok().range, "")
			.secondary(p.nth_tok(1).range, "");

		p.error(err);
		let m = p.start();
		p.bump_any();
		ts_interface(p);
		m.complete(p, ERROR);
	}

	// Labelled statement
	if p.nth_at(1, T![:]) {
		if let Present(identifier) = parse_identifier(p, JS_LABELED_STATEMENT) {
			if let Valid(identifier) = &identifier {
				let range = identifier.range(p);
				let label = p.source(range);

				if let Some(first_range) = p.state.labels.get(label) {
					let err = p
						.err_builder("Duplicate statement labels are not allowed")
						.secondary(
							first_range.to_owned(),
							&format!("`{}` is first used as a label here", label),
						)
						.primary(
							range,
							&format!("a second use of `{}` here is not allowed", label),
						);

					p.error(err);
				} else {
					let string = label.to_string();
					p.state.labels.insert(string, range.into());
				}
			}

			let kind = if identifier.is_valid() {
				JS_LABELED_STATEMENT
			} else {
				JS_UNKNOWN_STATEMENT
			};

			let labelled_statement = identifier.undo_completion(p);
			p.bump_any();
			parse_statement(p, None);

			return Present(labelled_statement.complete(p, kind));
		}
	}

	let expr = p.expr_with_semi_recovery(false);
	if let Some(expr) = expr {
		let m = expr.precede(p);
		semi(p, start..p.cur_tok().range.end);
		Present(m.complete(p, JS_EXPRESSION_STATEMENT))
	} else {
		Absent
	}
}

// test debugger_stmt
// debugger;

// test_err debugger_stmt
// function foo() {
// 	debugger {
// 		var something = "lorem";
// 	}
// }

/// A debugger statement such as `debugger;`
pub fn parse_debugger_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![debugger]) {
		return Absent;
	}
	let m = p.start();
	let range = p.cur_tok().range;
	p.bump_any(); // debugger keyword
	semi(p, range);
	Present(m.complete(p, JS_DEBUGGER_STATEMENT))
}

/// A throw statement such as `throw new Error("uh oh");`
// test throw_stmt
// throw new Error("foo");
// throw "foo"
#[allow(deprecated)]
pub fn parse_throw_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err throw_stmt_err
	// throw
	// new Error("oh no :(")
	if !p.at(T![throw]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any(); // throw keyword
	if p.has_linebreak_before_n(0) {
		let mut err = p
			.err_builder(
				"Linebreaks between a throw statement and the error to be thrown are not allowed",
			)
			.primary(p.cur_tok().range, "A linebreak is not allowed here");

		if p.at_ts(STARTS_EXPR) {
			err = err.secondary(p.cur_tok().range, "Help: did you mean to throw this?");
		}

		p.error(err);
		p.missing();
	} else {
		p.expr_with_semi_recovery(false);
	}
	semi(p, start..p.cur_tok().range.end);
	Present(m.complete(p, JS_THROW_STATEMENT))
}

// test break_stmt
// foo: {}
// rust: {}
// break;
// break foo;
// break rust

// test_err break_stmt
// function foo() { break; }
/// A break statement with an optional label such as `break a;`
pub fn parse_break_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![break]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range;
	p.bump_any(); // break keyword
	let end = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
		let label_token = p.cur_tok();
		p.bump_any();
		check_label_use(p, &label_token);

		label_token.range.end
	} else {
		p.missing();
		start.end
	};

	semi(p, start.start..p.cur_tok().range.end);

	if !p.state.break_allowed && p.state.labels.is_empty() {
		let err = p
			.err_builder("Invalid break not inside of a switch, loop, or labelled statement")
			.primary(start.start..end, "");

		p.error(err);
		Present(m.complete(p, JS_UNKNOWN_STATEMENT))
	} else {
		Present(m.complete(p, JS_BREAK_STATEMENT))
	}
}

// test continue_stmt
// foo: {}
// while (true) {
//   continue;
//   continue foo;
//   continue
// }

// test_err continue_stmt
// function foo() { continue; }
/// A continue statement with an optional label such as `continue a;`
pub fn parse_continue_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![continue]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range;
	p.bump_any(); // continue keyword

	let end = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
		let label_token = p.cur_tok();
		p.bump_any();
		check_label_use(p, &label_token);

		label_token.range.end
	} else {
		p.missing();
		start.end
	};

	semi(p, start.start..p.cur_tok().range.end);

	if !p.state.break_allowed && p.state.labels.is_empty() {
		let err = p
			.err_builder("Invalid continue not inside of a loop")
			.primary(start.start..end, "");

		p.error(err);
		Present(m.complete(p, JS_UNKNOWN_STATEMENT))
	} else {
		Present(m.complete(p, JS_CONTINUE_STATEMENT))
	}
}

// test return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
/// A return statement with an optional value such as `return a;`
#[allow(deprecated)]
pub fn parse_return_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err return_stmt_err
	// return;
	// return foo;
	if !p.at(T![return]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any(); // return keyword
	if !p.has_linebreak_before_n(0) && p.at_ts(STARTS_EXPR) {
		// TODO: review this part and make sure it plays well with the new recovery logic
		p.expr_with_semi_recovery(false);
	} else {
		p.missing();
	}
	semi(p, start..p.cur_tok().range.end);
	let mut complete = m.complete(p, JS_RETURN_STATEMENT);

	if !p.state.in_function && !p.syntax.global_return {
		let err = p
			.err_builder("Illegal return statement outside of a function")
			.primary(complete.range(p), "");

		p.error(err);
		complete.change_kind(p, JS_UNKNOWN_STATEMENT);
	}
	Present(complete)
}

// test empty_stmt
// ;
/// An empty statement denoted by a single semicolon.
pub fn parse_empty_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if p.at(T![;]) {
		let m = p.start();
		p.bump_any(); // bump ;
		m.complete(p, JS_EMPTY_STATEMENT).into()
	} else {
		Absent
	}
}

// test block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
/// A block statement consisting of statements wrapped in curly brackets.
pub(crate) fn parse_block_stmt(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	parse_block_impl(p, JS_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn parse_block_impl(
	p: &mut Parser,
	block_kind: SyntaxKind,
) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['{']) {
		return Absent;
	}

	let m = p.start();
	p.bump(T!['{']);

	let old_parser_state = if block_kind == JS_FUNCTION_BODY {
		directives(p)
	} else {
		None
	};

	parse_statements(p, true, None);

	p.expect_required(T!['}']);

	if let Some(old_parser_state) = old_parser_state {
		p.state = old_parser_state;
	}

	Present(m.complete(p, block_kind))
}

struct DirectivesList {
	old_state: Option<ParserState>,
}

impl Default for DirectivesList {
	fn default() -> Self {
		Self { old_state: None }
	}
}

impl DirectivesList {
	fn is_at_directives(&self, p: &mut Parser) -> bool {
		if !p.at(JS_STRING_LITERAL) {
			false
		} else {
			let next = p.nth(1);

			matches!(next, T![;] | EOF | T!['}']) || p.has_linebreak_before_n(1)
		}
	}
}

impl ParseNodeList for DirectivesList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		let directive_token = p.cur_tok();

		let directive = p.start();
		// bump string token
		p.bump_any();

		// eat semicolon if present, correct termination guaranteed by is_directive
		p.eat_optional(SyntaxKind::SEMICOLON);

		let completed_marker = directive.complete(p, JS_DIRECTIVE);

		let directive_text = p.token_src(&directive_token);

		if directive_text == "\"use strict\"" || directive_text == "'use strict'" {
			if self.old_state == None {
				self.old_state = Some(p.state.clone());
			}

			let mut new_state = p.state.clone();
			new_state.strict(p, directive_token.range);
			p.state = new_state;
		}

		Present(completed_marker)
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		!self.is_at_directives(p)
	}

	fn recover(
		&mut self,
		_p: &mut Parser,
		_parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult {
		// directives don't need proper error recovery
		Err(RecoveryError::AlreadyRecovered)
	}
}

#[must_use]
pub(crate) fn directives(p: &mut Parser) -> Option<ParserState> {
	let mut list = DirectivesList::default();
	list.parse_list(p);
	list.old_state
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn parse_statements(
	p: &mut Parser,
	stop_on_r_curly: bool,
	recovery_set: impl Into<Option<TokenSet>>,
) {
	let recovery_set = recovery_set.into();

	let list_start = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) {
		progress.assert_progressing(p);
		if stop_on_r_curly && p.at(T!['}']) {
			break;
		}

		parse_statement(p, recovery_set);
	}

	list_start.complete(p, LIST);
}

/// An expression wrapped in parentheses such as `()`
pub fn parenthesized_expression(p: &mut Parser) {
	p.state.allow_object_expr = p.expect_required(T!['(']);
	if expr(p).is_none() {
		p.missing();
	}
	p.expect_required(T![')']);
	p.state.allow_object_expr = true;
}

/// An if statement such as `if (foo) { bar(); }`
// test if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
pub fn parse_if_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err if_stmt_err
	// if (true) else {}
	// if (true) else
	// if else {}
	// if () {} else {}
	// if (true)}}}} {}
	if !p.at(T![if]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // bump if

	// (test)
	parenthesized_expression(&mut *p.with_state(ParserState {
		expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![else]]),
		..p.state.clone()
	}));

	// body
	// allows us to recover from `if (true) else {}`
	if parse_statement(p, STMT_RECOVERY_SET.union(token_set![T![else]])).is_none() {
		p.missing();
	}

	// else clause
	if p.at(T![else]) {
		let else_clause = p.start();
		p.bump_any(); // bump else
			  // stmt(p).into_required();
		parse_statement(p, None);
		else_clause.complete(p, JS_ELSE_CLAUSE);
	} else {
		p.missing();
	}

	Present(m.complete(p, JS_IF_STATEMENT))
}

/// A with statement such as `with (foo) something()`
pub fn parse_with_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![with]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // with
	parenthesized_expression(p);

	parse_statement(p, None);

	let with_stmt = m.complete(p, JS_WITH_STATEMENT);

	// or SloppyMode.exclusive_syntax(...) but this reads better with the error message, saying that
	// it's only forbidden in strict mode
	let conditional = StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
		p.err_builder("`with` statements are not allowed in strict mode")
			.primary(marker.range(p), "")
	});

	conditional.or_invalid_to_unknown(p, JS_UNKNOWN_STATEMENT)
}

/// A while statement such as `while(true) { do_something() }`
// test while_stmt
// while (true) {}
// while (5) {}
pub fn parse_while_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err while_stmt_err
	// while true {}
	// while {}
	// while (true {}
	// while true) }
	if !p.at(T![while]) {
		return Absent;
	}
	let m = p.start();
	p.bump_any(); // while
	parenthesized_expression(p);

	if parse_statement(
		&mut *p.with_state(ParserState {
			break_allowed: true,
			continue_allowed: true,
			..p.state.clone()
		}),
		None,
	)
	.is_none()
	{
		p.missing();
	}

	Present(m.complete(p, JS_WHILE_STATEMENT))
}

/// A var, const, or let declaration statement such as `var a = 5, b;` or `let {a, b} = foo;`
// test var_decl
// var a = 5;
// let { foo, bar } = 5;
// let bar, foo;
// const a = 5;
// const { foo: [bar], baz } = {};
// let foo = "lorem", bar = "ipsum", third = "value", fourth = 6;
// var a, a, a, a, a;
//
// test_err variable_declaration_statement_err
// let a, { a } = { a: 10 }
// const a = 1, { a } = { a: 10 }
// const a;
// let [a];
// const { b };
pub fn variable_declaration_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err var_decl_err
	// var a =;
	// const a = 5 let b = 5;
	let start = p.cur_tok().range.start;

	let declaration =
		parse_variable_declaration_list(p, VariableDeclarationParent::VariableStatement)
			.or_missing_with_error(p, js_parse_error::expected_variable);
	if let NodeMarker(declaration) = declaration {
		let m = declaration.precede(p);
		semi(p, start..p.cur_tok().range.start);
		Present(m.complete(p, JS_VARIABLE_STATEMENT))
	} else {
		Absent
	}
}

fn parse_variable_declaration_list(
	p: &mut Parser,
	declaration_context: VariableDeclarationParent,
) -> ParsedSyntax<CompletedMarker> {
	let m = p.start();

	match parse_variable_declarations(p, declaration_context) {
		Absent => {
			m.abandon(p);
			Absent
		}
		Present(_) => Present(m.complete(p, JS_VARIABLE_DECLARATION_LIST)),
	}
}

/// What's the parent node of the variable declaration
#[repr(u8)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum VariableDeclarationParent {
	/// Declaration inside a `for...of` or `for...in` or `for (;;)` loop
	For,

	/// Declaration as part of a variable statement (`let a`, `const b`, or `var c`).
	VariableStatement,
}

/// Parses a list of JS_VARIABLE_DECLARATION_LIST
/// Returns a tuple where
/// * the first element is the marker to the not yet completed list
/// * the second element is the range of all variable declarations except the first one. Is [None] if
///   there's only one declaration.
fn parse_variable_declarations(
	p: &mut Parser,
	declaration_parent: VariableDeclarationParent,
) -> ParsedSyntax<(Option<CompletedMarker>, Option<Range<usize>>)> {
	let mut context = VariableDeclarationContext::new(declaration_parent);

	match p.cur() {
		T![var] => p.bump_any(),
		T![const] => {
			context.is_const = Some(p.cur_tok().range);
			p.bump_any()
		}
		T![ident] if p.cur_src() == "let" => {
			// let is a valid identifier name that's why the returns an ident for let.
			// remap it here because we know from the context that this is the let keyword.
			p.bump_remap(T![let]);
			context.is_let = true;
		}
		_ => {
			return Absent;
		}
	}

	let mut parse_declarations = ParseVariableDeclarations {
		declaration_context: context,
		remaining_declaration_range: None,
	};

	let list = parse_declarations.parse_list(p);

	p.state.name_map.clear();
	Present((list, parse_declarations.remaining_declaration_range))
}

struct ParseVariableDeclarations {
	declaration_context: VariableDeclarationContext,
	// Range of the declarations succeeding the first declaration
	// None until this hits the second declaration
	remaining_declaration_range: Option<Range<usize>>,
}

impl ParseSeparatedList for ParseVariableDeclarations {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		parse_variable_declaration(p, &self.declaration_context).map(|declaration| {
			if self.declaration_context.is_first {
				self.declaration_context.is_first = false;
			} else if let Some(range) = self.remaining_declaration_range.as_mut() {
				range.end = declaration.range(p).as_range().end;
			} else {
				self.remaining_declaration_range = Some(declaration.range(p).as_range());
			}
			declaration
		})
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		if self.declaration_context.is_first {
			false
		} else {
			!p.at(T![,])
		}
	}

	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(
				// TODO this isn't really valid. Either `parse_variable_declarations` should return `ConditionalSyntax`
				// or we use an `Unknown` node here
				JS_VARIABLE_DECLARATION,
				STMT_RECOVERY_SET.union(token_set!(T![,])),
			)
			.enable_recovery_on_line_break(),
			expected_binding,
		)
	}

	fn separating_element_kind(&mut self) -> SyntaxKind {
		T![,]
	}
}

struct VariableDeclarationContext {
	/// The range of the `const` keyword if this is `const` variable declaration.
	is_const: Option<Range<usize>>,
	/// `true` if this is a let variable declaration
	is_let: bool,
	/// Is this the first declaration in the declaration list (a first, b second in `let a, b`)
	is_first: bool,
	/// What's the parent of the variable declaration
	parent: VariableDeclarationParent,
}

impl VariableDeclarationContext {
	fn new(parent: VariableDeclarationParent) -> Self {
		Self {
			parent,
			is_const: None,
			is_let: false,
			is_first: true,
		}
	}

	fn duplicate_binding_parent_name(&self) -> Option<&'static str> {
		if self.is_const.is_some() {
			Some("const")
		} else if self.is_let {
			Some("let")
		} else {
			None
		}
	}
}

// A single declarator, either `ident` or `ident = assign_expr`
fn parse_variable_declaration(
	p: &mut Parser,
	context: &VariableDeclarationContext,
) -> ParsedSyntax<CompletedMarker> {
	p.state.duplicate_binding_parent = context.duplicate_binding_parent_name();
	let id = parse_binding_pattern(p);
	p.state.duplicate_binding_parent = None;

	id.map(|id| {
		let m = id.precede(p);

		let cur = p.cur_tok().range;
		let opt = p.eat_optional(T![!]);
		if opt && !p.typescript() {
			let err = p
				.err_builder("definite assignment assertions can only be used in TypeScript files")
				.primary(cur, "");

			p.error(err);
		}

		let type_annotation = maybe_ts_type_annotation(p);

		if type_annotation.is_none() {
			p.missing();
		}

		let initializer = parse_initializer_clause(p).or_missing(p);

		// Heuristic to determine if we're in a for of or for in loop. This may be off if
		// the user uses a for of/in with multiple declarations but this isn't allowed anyway.
		let is_in_for_of_or_in = context.parent == VariableDeclarationParent::For
			&& context.is_first
			&& (p.cur_src() == "of" || p.at(T![in]));

		if is_in_for_of_or_in {
			if p.typescript() {
				if let Some(type_annotation) = type_annotation {
					let err = p
						.err_builder("`for` statement declarators cannot have a type annotation")
						.primary(type_annotation.start..type_annotation.end, "");

					p.error(err);
				}
			}
		} else if initializer.is_missing_marker()
			&& matches!(
				id.kind(),
				JS_ARRAY_BINDING_PATTERN | JS_OBJECT_BINDING_PATTERN
			) && !p.state.in_declare
		{
			let err = p
				.err_builder("Object and Array patterns require initializers")
				.primary(
					id.range(p),
					"this pattern is declared, but it is not given an initialized value",
				);

			p.error(err);
		// FIXME: does ts allow const var declarations without initializers in .d.ts files?
		} else if initializer.is_missing_marker()
			&& context.is_const.is_some()
			&& !p.state.in_declare
		{
			let err = p
				.err_builder("Const var declarations must have an initialized value")
				.primary(id.range(p), "this variable needs to be initialized");

			p.error(err);
		}

		m.complete(p, JS_VARIABLE_DECLARATION)
	})
}

// A do.. while statement, such as `do {} while (true)`
// test do_while_stmtR
// do { } while (true)
// do throw Error("foo") while (true)
// do { break; } while (true)
pub fn parse_do_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err do_while_stmt_err
	// do while (true)
	// do while ()
	// do while true

	// test_err do_while_no_continue_break
	// do { } break (continue)
	// do { } continue (break)
	if !p.at(T![do]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any(); // do keyword

	if parse_statement(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
	)
	.is_none()
	{
		p.missing();
	}

	p.expect_required(T![while]);
	parenthesized_expression(p);
	let end_range = p.cur_tok().range.end;
	if !semi(p, start..end_range) {
		p.missing();
	}
	Present(m.complete(p, JS_DO_WHILE_STATEMENT))
}

/// Parses the header of a for statement into the current node and returns whatever it is a for in/of or "regular" for statement
fn parse_for_head(p: &mut Parser) -> SyntaxKind {
	// for (;...
	if p.at(T![;]) {
		p.missing();
		parse_normal_for_head(p);
		return FOR_STMT;
	}

	// `for (let...` | `for (const...` | `for (var...`

	if p.at(T![const]) || p.at(T![var]) || (p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)))
	{
		let m = p.start();

		let (declarations, additional_declarations) = {
			let guard = &mut *p.with_state(ParserState {
				include_in: false,
				..p.state.clone()
			});

			parse_variable_declarations(&mut *guard, VariableDeclarationParent::For).unwrap()
		};

		let is_in = p.at(T![in]);
		let is_of = p.cur_src() == "of";

		if is_in || is_of {
			if let Some(declarations_list) = declarations {
				// remove the intermediate list node created by parse variable declarations that is not needed
				// for a ForInOrOfInitializer where the variable declaration is a direct child.
				declarations_list.undo_completion(p).abandon(p);
			}

			if let Some(additional_declarations_range) = additional_declarations {
				p.error(
					p.err_builder(&format!(
						"Only a single declaration is allowed in a `for...{}` statement.",
						if is_of { "of" } else { "in" },
					))
					.primary(additional_declarations_range, "additional declarations"),
				);
				// TODO change kind to UNKNOWN
			}

			m.complete(p, JS_FOR_VARIABLE_DECLARATION);

			parse_for_of_or_in_head(p)
		} else {
			m.complete(p, JS_VARIABLE_DECLARATION_LIST);
			parse_normal_for_head(p);
			FOR_STMT
		}
	} else {
		// for (some_expression`
		let checkpoint = p.checkpoint();
		let init_expr = {
			let guard = &mut *p.with_state(ParserState {
				include_in: false,
				..p.state.clone()
			});
			expr(guard)
		};

		if p.at(T![in]) || p.cur_src() == "of" {
			// for (assignment_pattern in ...
			if let Some(assignment_expr) = init_expr {
				let mut assignment = expression_to_assignment_pattern(
					p,
					assignment_expr,
					checkpoint,
					AssignmentExprPrecedence::Any,
				);

				if p.typescript()
					&& p.at(T![in]) && matches!(
					assignment.kind(),
					JS_ARRAY_ASSIGNMENT_PATTERN | JS_OBJECT_ASSIGNMENT_PATTERN
				) {
					let err = p.err_builder("the left hand side of a `for..in` statement cannot be a destructuring pattern")
							.primary(assignment.range(p), "");
					p.error(err);
					assignment.change_kind(p, JS_UNKNOWN_ASSIGNMENT);
				}
			} else {
				p.missing();
			}

			return parse_for_of_or_in_head(p);
		}

		if init_expr.is_none() {
			p.missing();
		}

		parse_normal_for_head(p);
		FOR_STMT
	}
}

/// Parses the parenthesized part of a non for in or for of statement
/// Expects to be positioned right after the initializer
fn parse_normal_for_head(p: &mut Parser) {
	p.expect_required(T![;]);

	if p.at(T![;]) {
		p.missing(); // missing test
	} else {
		let m = p.start();
		expr(p);
		m.complete(p, FOR_STMT_TEST);
	}

	p.expect_required(T![;]);

	if p.at(T![')']) {
		p.missing(); // Missing update
	} else {
		let m = p.start();
		expr(p);
		m.complete(p, FOR_STMT_UPDATE);
	}
}

/// Expects to be positioned right before the of or in keyword
fn parse_for_of_or_in_head(p: &mut Parser) -> SyntaxKind {
	let is_in = p.at(T![in]);

	if is_in {
		p.bump_any();
		if expr(p).is_none() {
			p.missing();
		}

		JS_FOR_IN_STATEMENT
	} else {
		p.bump_remap(T![of]);

		if expr_or_assignment(p).is_none() {
			p.missing();
		}

		JS_FOR_OF_STATEMENT
	}
}

/// Either a traditional for statement or a for.. in statement
// test for_stmt
// for (let i = 5; i < 10; i++) {}
// for (let { foo, bar } of {}) {}
// for (foo in {}) {}
// for (;;) {}
// for (let foo of []) {}
// for (let i = 5, j = 6; i < j; ++j) {}
// for await (let a of []) {}
pub fn parse_for_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err for_stmt_err
	// for ;; {}
	// for let i = 5; i < 10; i++ {}
	// for let i = 5; i < 10; ++i {}
	// for (in []) {}
	// for (let i, j = 6 of []) {}
	// for await (let a in []) {}
	// for await (let i = 0; i < 10; ++i) {}
	// for (let [a];;) {}
	if !p.at(T![for]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // for keyword

	let mut await_range = None;
	let mut await_missing_marker = None;
	if p.at(T![await]) {
		await_range = Some(p.cur_tok().range);
		p.bump_any();
	} else {
		// Add the missing await for for...of but we may need to undo it later if the for loop turns out
		// to be a for...in or normal for loop
		await_missing_marker = Some(p.missing());
	}

	p.expect_required(T!['(']);
	let kind = parse_for_head(p);
	p.expect_required(T![')']);
	parse_statement(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
	);

	let mut completed = m.complete(p, kind);

	if kind != JS_FOR_OF_STATEMENT {
		if let Some(await_range) = await_range {
			p.error(
				p.err_builder("await can only be used in conjunction with `for...of` statements")
					.primary(await_range, "Remove the await here")
					.secondary(
						completed.range(p),
						"or convert this to a `for...of` statement",
					),
			);
			completed.change_kind(p, JS_UNKNOWN_STATEMENT)
		}

		if let Some(await_missing_marker) = await_missing_marker {
			await_missing_marker.undo(p);
		}
	}

	Present(completed)
}

struct SwitchClausesList;

impl ParseNodeList for SwitchClausesList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
		match parse_statement(p, None) {
			Some(stmt) => Present(stmt),
			None => Absent,
		}
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at_ts(token_set![T![default], T![case], T!['}']])
	}

	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> parser::RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
			js_parse_error::expected_case,
		)
	}
}

struct ConsList;
impl ParseNodeList for ConsList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		match parse_statement(p, None) {
			None => Absent,
			Some(marker) => Present(marker),
		}
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at_ts(token_set![T![default], T![case], T!['}']])
	}

	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_STATEMENT, token_set!()),
			js_parse_error::expected_case,
		)
	}
}
// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn parse_switch_clause(
	p: &mut Parser,
	first_default: &mut Option<CompletedMarker>,
) -> ParsedSyntax<CompletedMarker> {
	let m = p.start();
	match p.cur() {
		T![default] => {
			// in case we have two `default` expression, we mark the second one
			// as `JS_CASE_CLAUSE` where the the "default" keyword is an Unknown node
			let syntax_kind = if first_default.is_some() {
				// missing "case" keyword
				p.missing();
				let discriminant = p.start();
				p.bump_any(); // interpret `default` as the test of the case
				discriminant.complete(p, JS_UNARY_EXPRESSION);
				JS_CASE_CLAUSE
			} else {
				p.bump_any();
				JS_DEFAULT_CLAUSE
			};

			p.expect_required(T![:]);
			ConsList.parse_list(p);
			let default = m.complete(p, syntax_kind);
			if first_default.is_some() {
				let err = p
					.err_builder(
						"Multiple default clauses inside of a switch statement are not allowed",
					)
					.secondary(
						first_default.unwrap().range(p),
						"the first default clause is defined here",
					)
					.primary(default.range(p), "a second clause here is not allowed");

				p.error(err);
			}

			Present(default)
		}
		T![case] => {
			p.bump_any();
			expr(p);
			p.expect_required(T![:]);

			SwitchClausesList.parse_list(p);
			Present(m.complete(p, JS_CASE_CLAUSE))
		}
		_ => {
			m.abandon(p);
			Absent
		}
	}
}
#[derive(Default)]
struct SwitchCasesList {
	first_default: Option<CompletedMarker>,
}

impl ParseNodeList for SwitchCasesList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		let clause = parse_switch_clause(p, &mut self.first_default);

		if let Present(marker) = clause {
			if marker.kind() == JS_DEFAULT_CLAUSE && self.first_default == None {
				self.first_default = Some(marker);
			}
		}

		clause
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at(T!['}'])
	}

	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult {
		if let Present(marker) = parsed_element {
			Ok(marker)
		} else {
			let m = p.start();
			p.missing(); // case
			p.missing(); // discriminant
			p.missing(); // colon

			let statements = p.start();

			let recovered_element = parsed_element.or_recover(
				p,
				&ParseRecovery::new(
					JS_UNKNOWN_STATEMENT,
					token_set![T![default], T![case], T!['}']],
				)
				.enable_recovery_on_line_break(),
				js_parse_error::expected_case_or_default,
			);

			match recovered_element {
				Ok(marker) => {
					statements.complete(p, LIST);
					m.complete(p, JS_CASE_CLAUSE);
					Ok(marker)
				}
				Err(err) => {
					statements.abandon(p);
					m.abandon(p);
					Err(err)
				}
			}
		}
	}
}

/// A switch statement such as
///
/// ```js
/// switch (a) {
///     case foo:
///         bar();
/// }
/// ```
// test switch_stmt
// switch (foo) {
//  case bar:
//  default:
// }
pub fn parse_switch_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// test_err switch_stmt_err
	// switch foo {}
	// switch {}
	// switch { var i = 0 }
	// switch { var i = 0; case "bar": {} }
	// switch (foo) {
	// 	default: {}
	// 	default: {}
	// }

	if !p.at(T![switch]) {
		return Absent;
	}
	let m = p.start();
	p.bump_any(); // switch keyword
	parenthesized_expression(p);
	p.expect_required(T!['{']);

	{
		let mut temp = p.with_state(ParserState {
			break_allowed: true,
			..p.state.clone()
		});
		SwitchCasesList::default().parse_list(&mut *temp);
	}
	p.expect_required(T!['}']);
	Present(m.complete(p, JS_SWITCH_STATEMENT))
}

fn parse_catch_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![catch]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // bump catch

	parse_catch_declaration(p).or_missing(p);
	parse_block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);

	Present(m.complete(p, JS_CATCH_CLAUSE))
}

fn parse_catch_declaration(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['(']) {
		return Absent;
	}

	let declaration_marker = p.start();

	p.bump_any(); // bump (

	let pattern_marker = parse_binding_pattern(p).or_missing_with_error(p, expected_binding);
	let pattern_kind = pattern_marker.kind();

	if p.at(T![:]) {
		let error_marker = match pattern_marker {
			NodeMarker(pattern_node) => pattern_node.precede(p),
			_ => p.start(),
		};
		let start = p.cur_tok().range.start;
		p.bump_any();
		let ty = ts_type(p);
		if !matches!(
			ty.as_ref().map(|x| p.span_text(x.range(p))),
			Some("unknown") | Some("any")
		) && p.typescript()
			&& ty.is_some()
		{
			let err = p.err_builder("type annotations for catch parameters can only be `unknown` or `any` if specified")
					.primary(ty.as_ref().unwrap().range(p), "");

			p.error(err);
		}

		let end = ty
			.map(|x| usize::from(x.range(p).end()))
			.unwrap_or(p.cur_tok().range.start);
		error_marker.complete(p, pattern_kind.filter(|_| p.typescript()).unwrap_or(ERROR));

		if !p.typescript() {
			let err = p
				.err_builder("type annotations can only be used in TypeScript files")
				.primary(start..end, "");

			p.error(err);
		}
	}
	p.expect_required(T![')']);

	Present(declaration_marker.complete(p, JS_CATCH_DECLARATION))
}

/// A try statement such as
///
/// ```js
/// try {
///     something();
/// } catch (a) {
///
/// }
/// ```
// test try_stmt
// try {} catch {}
// try {} catch (e) {}
// try {} catch {} finally {}
// try {} catch (e) {} finally {}
// try {} finally {}
pub fn parse_try_statement(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	// TODO: recover from `try catch` and `try finally`. The issue is block_items
	// will cause infinite recursion because parsing a stmt would not consume the catch token
	// and block_items would not exit, and if we exited on any error that would greatly limit
	// block_items error recovery

	if !p.at(T![try]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // eat try

	parse_block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);

	let catch = parse_catch_clause(p);

	if p.at(T![finally]) {
		catch.or_missing(p);

		let finalizer = p.start();
		p.bump_any();
		parse_block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);
		finalizer.complete(p, JS_FINALLY_CLAUSE);
		Present(m.complete(p, JS_TRY_FINALLY_STATEMENT))
	} else {
		catch.or_missing_with_error(p, js_parse_error::expected_catch_clause);
		Present(m.complete(p, JS_TRY_STATEMENT))
	}
}
