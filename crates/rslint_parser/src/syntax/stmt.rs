//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::binding::*;
use super::expr::parse_expression;
use super::program::export_decl;
use super::typescript::*;
use super::util::check_label_use;
use crate::parser::{ParseNodeList, ParsedSyntax, ParserProgress};
use crate::parser::{RecoveryError, RecoveryResult};
use crate::syntax::assignment::{expression_to_assignment_pattern, AssignmentExprPrecedence};
use crate::syntax::class::{parse_class_declaration, parse_initializer_clause};
use crate::syntax::expr::{
	is_at_expression, is_at_identifier, is_nth_at_identifier_name, parse_expr_or_assignment,
	parse_expression_or_recover_to_next_statement, parse_identifier,
};
use crate::syntax::function::{is_at_async_function, parse_function_declaration, LineBreak};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{expected_binding, expected_statement};
use crate::syntax::module::parse_import;
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::SyntaxFeature;
use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;
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
	if p.eat(T![;]) {
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
pub fn parse_statement(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
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
		t if (t == T![enum] && is_nth_at_identifier_name(p, 1))
			|| (t == T![const] && p.nth_at(1, T![enum]) && is_nth_at_identifier_name(p, 2)) =>
		{
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
		T![function] => parse_function_declaration(p),
		T![class] => parse_class_declaration(p),
		T![ident] if is_at_async_function(p, LineBreak::DoCheck) => parse_function_declaration(p),

		T![ident] if p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)) => {
			variable_declaration_statement(p)
		}
		// TODO: handle `<T>() => {};` with less of a hack
		_ if is_at_expression(p) => {
			if is_at_identifier(p) && p.nth_at(1, T![:]) {
				parse_labeled_statement(p)
			} else {
				parse_expression_statement(p)
			}
		}
		_ => Absent,
	}
}

// test_err double_label
// label1: {
// 	label2: {
// 		label1: {}
// 	}
// }
fn parse_labeled_statement(p: &mut Parser) -> ParsedSyntax {
	parse_identifier(p, JS_LABELED_STATEMENT).map(|identifier| {
		if !identifier.kind().is_unknown() {
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

		let labelled_statement = identifier.undo_completion(p);
		p.bump_any();
		parse_statement(p).or_add_diagnostic(p, expected_statement);

		labelled_statement.complete(p, JS_LABELED_STATEMENT)
	})
}

// test ts_keyword_assignments
// declare = 1;
// abstract = 2;
// namespace = 3;
// type = 4;
// module = 5;
// global = 6;
//
// test ts_keywords_assignments_script
// // SCRIPT
// interface = 1;
// private = 2;
// protected = 3;
// public = 4;
// implements = 5;
fn parse_expression_statement(p: &mut Parser) -> ParsedSyntax {
	let start = p.cur_tok().range.start;

	let expr = parse_expression_or_recover_to_next_statement(p, false);

	if let Ok(expr) = expr {
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
pub fn parse_debugger_statement(p: &mut Parser) -> ParsedSyntax {
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
pub fn parse_throw_statement(p: &mut Parser) -> ParsedSyntax {
	// test_err throw_stmt_err
	// throw
	// new Error("oh no :(")
	// throw;
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

		if is_at_expression(p) {
			err = err.secondary(p.cur_tok().range, "Help: did you mean to throw this?");
		}

		p.error(err);
	} else {
		parse_expression_or_recover_to_next_statement(p, false).ok();
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
pub fn parse_break_statement(p: &mut Parser) -> ParsedSyntax {
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
pub fn parse_continue_statement(p: &mut Parser) -> ParsedSyntax {
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
pub fn parse_return_statement(p: &mut Parser) -> ParsedSyntax {
	// test_err return_stmt_err
	// return;
	// return foo;
	if !p.at(T![return]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any(); // return keyword
	if !p.has_linebreak_before_n(0) {
		parse_expression(p).ok();
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
pub fn parse_empty_statement(p: &mut Parser) -> ParsedSyntax {
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
pub(crate) fn parse_block_stmt(p: &mut Parser) -> ParsedSyntax {
	parse_block_impl(p, JS_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn parse_block_impl(p: &mut Parser, block_kind: JsSyntaxKind) -> ParsedSyntax {
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

	parse_statements(p, true);

	p.expect(T!['}']);

	if let Some(old_parser_state) = old_parser_state {
		p.state = old_parser_state;
	}

	Present(m.complete(p, block_kind))
}

#[derive(Default)]
struct DirectivesList {
	old_state: Option<ParserState>,
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
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
		let directive_token = p.cur_tok();

		let directive = p.start();
		// bump string token
		p.bump_any();

		// eat semicolon if present, correct termination guaranteed by is_directive
		p.eat(JsSyntaxKind::SEMICOLON);

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

	fn recover(&mut self, _p: &mut Parser, _parsed_element: ParsedSyntax) -> RecoveryResult {
		// directives don't need proper error recovery
		Err(RecoveryError::AlreadyRecovered)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_DIRECTIVE_LIST
	}
}

// test directives
// // SCRIPT
// "use new"
// let a = 10;
// "use strict"; // not a directive
// function test() {
// 	'use strict';
// 	let a = 10;
// 	'use strict'; // not a directive
// }
// (function () {
// 	"use strict";
// 	let a = 10;
// 	"use strict"; // not a directive
// });
// let b = () => {
// 	"use strict";
// 	let a = 10;
// 	"use strict";  // not a directive
// }
// {
// 	"use strict"; // not a directive
// }
//
// test_err directives_err
// // SCRIPT
// function test() {
// 	"use strict";
// 	function inner_a() {
// 		"use strict";
// 	}
// 	function inner_b() {
// 		function inner_inner() {
// 			"use strict";
// 		}
// 	}
// }
#[must_use]
pub(crate) fn directives(p: &mut Parser) -> Option<ParserState> {
	let mut list = DirectivesList::default();
	list.parse_list(p);
	list.old_state
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn parse_statements(p: &mut Parser, stop_on_r_curly: bool) {
	let list_start = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) {
		progress.assert_progressing(p);
		if stop_on_r_curly && p.at(T!['}']) {
			break;
		}

		if parse_statement(p)
			.or_recover(
				p,
				&ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
				expected_statement,
			)
			.is_err()
		{
			break;
		}
	}

	list_start.complete(p, JS_STATEMENT_LIST);
}

/// An expression wrapped in parentheses such as `()`
pub fn parenthesized_expression(p: &mut Parser) {
	p.state.allow_object_expr = p.expect(T!['(']);
	parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
	p.expect(T![')']);
	p.state.allow_object_expr = true;
}

/// An if statement such as `if (foo) { bar(); }`
// test if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
pub fn parse_if_statement(p: &mut Parser) -> ParsedSyntax {
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
	parenthesized_expression(p);

	// body
	parse_statement(p).or_add_diagnostic(p, expected_statement);

	// else clause
	if p.at(T![else]) {
		let else_clause = p.start();
		p.bump_any(); // bump else
		parse_statement(p).or_add_diagnostic(p, expected_statement);
		else_clause.complete(p, JS_ELSE_CLAUSE);
	}

	Present(m.complete(p, JS_IF_STATEMENT))
}

// test with_statement
// // SCRIPT
// function f(x, o) {
// 	with (o) {
// 		console.log(x);
// 	}
// }
/// A with statement such as `with (foo) something()`
pub fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![with]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // with
	parenthesized_expression(p);

	parse_statement(p).or_add_diagnostic(p, expected_statement);

	let with_stmt = m.complete(p, JS_WITH_STATEMENT);

	// or SloppyMode.exclusive_syntax(...) but this reads better with the error message, saying that
	// it's only forbidden in strict mode
	StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
		p.err_builder("`with` statements are not allowed in strict mode")
			.primary(marker.range(p), "")
	})
}

/// A while statement such as `while(true) { do_something() }`
// test while_stmt
// while (true) {}
// while (5) {}
pub fn parse_while_statement(p: &mut Parser) -> ParsedSyntax {
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

	{
		let guard = &mut *p.with_state(ParserState {
			break_allowed: true,
			continue_allowed: true,
			..p.state.clone()
		});
		parse_statement(guard).or_add_diagnostic(guard, expected_statement);
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
pub fn variable_declaration_statement(p: &mut Parser) -> ParsedSyntax {
	// test_err var_decl_err
	// var a =;
	// const a = 5 let b = 5;
	let start = p.cur_tok().range.start;

	let declaration =
		parse_variable_declaration_list(p, VariableDeclarationParent::VariableStatement)
			.or_add_diagnostic(p, js_parse_error::expected_variable);
	if let Some(declaration) = declaration {
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
) -> ParsedSyntax {
	let m = p.start();
	if parse_variable_declarations(p, declaration_context).is_some() {
		Present(m.complete(p, JS_VARIABLE_DECLARATIONS))
	} else {
		m.abandon(p);
		Absent
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
) -> Option<(CompletedMarker, Option<Range<usize>>)> {
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
			return None;
		}
	}

	let mut parse_declarations = ParseVariableDeclarations {
		declaration_context: context,
		remaining_declaration_range: None,
	};

	let list = parse_declarations.parse_list(p);

	p.state.name_map.clear();
	Some((list, parse_declarations.remaining_declaration_range))
}

struct ParseVariableDeclarations {
	declaration_context: VariableDeclarationContext,
	// Range of the declarations succeeding the first declaration
	// None until this hits the second declaration
	remaining_declaration_range: Option<Range<usize>>,
}

impl ParseSeparatedList for ParseVariableDeclarations {
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
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

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN, STMT_RECOVERY_SET.union(token_set!(T![,])))
				.enable_recovery_on_line_break(),
			expected_binding,
		)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_VARIABLE_DECLARATION_LIST
	}

	fn separating_element_kind(&mut self) -> JsSyntaxKind {
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
) -> ParsedSyntax {
	p.state.duplicate_binding_parent = context.duplicate_binding_parent_name();
	let id = parse_binding_pattern(p);
	p.state.duplicate_binding_parent = None;

	id.map(|id| {
		let m = id.precede(p);

		let cur = p.cur_tok().range;
		let opt = p.eat(T![!]);
		if opt && !p.typescript() {
			let err = p
				.err_builder("definite assignment assertions can only be used in TypeScript files")
				.primary(cur, "");

			p.error(err);
		}

		let type_annotation = maybe_ts_type_annotation(p);

		let initializer = parse_initializer_clause(p).ok();

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
		} else if initializer.is_none()
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
		} else if initializer.is_none() && context.is_const.is_some() && !p.state.in_declare {
			let err = p
				.err_builder("Const var declarations must have an initialized value")
				.primary(id.range(p), "this variable needs to be initialized");

			p.error(err);
		}

		m.complete(p, JS_VARIABLE_DECLARATION)
	})
}

// A do.. while statement, such as `do {} while (true)`

// test do_while_statement
// do console.log("test"); while(true)
// do {
// 	console.log("test")
// } while (true);
// let a = 1;
// do
// do {
// 	a = a + 1
// } while(a < 5)
// while (a < 100)
//
// test do_while_stmt
// do { } while (true)
// do { throw Error("foo") } while (true)
// do { break; } while (true)
pub fn parse_do_statement(p: &mut Parser) -> ParsedSyntax {
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

	{
		let guard = &mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		});
		parse_statement(guard).or_add_diagnostic(guard, expected_statement);
	}

	p.expect(T![while]);
	parenthesized_expression(p);
	let end_range = p.cur_tok().range.end;
	semi(p, start..end_range);
	Present(m.complete(p, JS_DO_WHILE_STATEMENT))
}

/// Parses the header of a for statement into the current node and returns whatever it is a for in/of or "regular" for statement
fn parse_for_head(p: &mut Parser) -> JsSyntaxKind {
	// for (;...
	if p.at(T![;]) {
		parse_normal_for_head(p);
		return JS_FOR_STATEMENT;
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
			// remove the intermediate list node created by parse variable declarations that is not needed
			// for a ForInOrOfInitializer where the variable declaration is a direct child.
			declarations.undo_completion(p).abandon(p);

			if let Some(additional_declarations_range) = additional_declarations {
				p.error(
					p.err_builder(&format!(
						"Only a single declaration is allowed in a `for...{}` statement.",
						if is_of { "of" } else { "in" },
					))
					.primary(additional_declarations_range, "additional declarations"),
				);
			}

			m.complete(p, JS_FOR_VARIABLE_DECLARATION);

			parse_for_of_or_in_head(p)
		} else {
			m.complete(p, JS_VARIABLE_DECLARATIONS);
			parse_normal_for_head(p);
			JS_FOR_STATEMENT
		}
	} else {
		// for (some_expression`
		let checkpoint = p.checkpoint();
		let init_expr = {
			let guard = &mut *p.with_state(ParserState {
				include_in: false,
				..p.state.clone()
			});
			// Replace the `p.token_pos() == checkpoint.token_pos` once `expr()` returns `ParsedSyntax`
			parse_expression(guard)
		};

		if p.at(T![in]) || p.cur_src() == "of" {
			// for (assignment_pattern in ...
			if let Present(assignment_expr) = init_expr {
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
			}

			return parse_for_of_or_in_head(p);
		}

		init_expr.or_add_diagnostic(p, js_parse_error::expected_expression);

		parse_normal_for_head(p);
		JS_FOR_STATEMENT
	}
}

/// Parses the parenthesized part of a non for in or for of statement
/// Expects to be positioned right after the initializer
fn parse_normal_for_head(p: &mut Parser) {
	p.expect(T![;]);

	if !p.at(T![;]) {
		parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
	}

	p.expect(T![;]);

	if !p.at(T![')']) {
		parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
	}
}

/// Expects to be positioned right before the of or in keyword
fn parse_for_of_or_in_head(p: &mut Parser) -> JsSyntaxKind {
	let is_in = p.at(T![in]);

	if is_in {
		p.bump_any();
		parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);

		JS_FOR_IN_STATEMENT
	} else {
		p.bump_remap(T![of]);

		parse_expr_or_assignment(p)
			.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

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
pub fn parse_for_statement(p: &mut Parser) -> ParsedSyntax {
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
	if p.at(T![await]) {
		await_range = Some(p.cur_tok().range);
		p.bump_any();
	}

	p.expect(T!['(']);
	let kind = parse_for_head(p);
	p.expect(T![')']);

	{
		let guard = &mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		});
		parse_statement(guard).or_add_diagnostic(guard, expected_statement);
	}

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
	}

	Present(completed)
}

struct SwitchClausesList;

impl ParseNodeList for SwitchClausesList {
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
		parse_statement(p)
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at_ts(token_set![T![default], T![case], T!['}']])
	}

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> parser::RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
			js_parse_error::expected_case,
		)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_STATEMENT_LIST
	}
}

struct ConsList;
impl ParseNodeList for ConsList {
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
		parse_statement(p)
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at_ts(token_set![T![default], T![case], T!['}']])
	}

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_STATEMENT, token_set!()),
			js_parse_error::expected_case,
		)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_STATEMENT_LIST
	}
}
// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn parse_switch_clause(
	p: &mut Parser,
	first_default: &mut Option<CompletedMarker>,
) -> ParsedSyntax {
	let m = p.start();
	match p.cur() {
		T![default] => {
			// in case we have two `default` expression, we mark the second one
			// as `JS_CASE_CLAUSE` where the the "default" keyword is an Unknown node
			let syntax_kind = if first_default.is_some() {
				let discriminant = p.start();
				p.bump_any(); // interpret `default` as the test of the case
				discriminant.complete(p, JS_UNKNOWN_EXPRESSION);
				JS_CASE_CLAUSE
			} else {
				p.bump_any();
				JS_DEFAULT_CLAUSE
			};

			p.expect(T![:]);
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
			parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
			p.expect(T![:]);

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
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
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

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
		if let Present(marker) = parsed_element {
			Ok(marker)
		} else {
			let m = p.start();
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
					statements.complete(p, JS_STATEMENT_LIST);
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

	fn list_kind() -> JsSyntaxKind {
		JS_SWITCH_CASE_LIST
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
pub fn parse_switch_statement(p: &mut Parser) -> ParsedSyntax {
	// test_err switch_stmt_err
	// switch foo {}
	// switch {}
	// switch { var i = 0 }
	// switch { var i = 0; case "bar": {} }
	// switch (foo) {
	// 	default: {}
	// 	default: {}
	// }
	// switch (foo) { case : }

	if !p.at(T![switch]) {
		return Absent;
	}
	let m = p.start();
	p.bump_any(); // switch keyword
	parenthesized_expression(p);
	p.expect(T!['{']);

	{
		let mut temp = p.with_state(ParserState {
			break_allowed: true,
			..p.state.clone()
		});
		SwitchCasesList::default().parse_list(&mut *temp);
	}
	p.expect(T!['}']);
	Present(m.complete(p, JS_SWITCH_STATEMENT))
}

fn parse_catch_clause(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![catch]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // bump catch

	parse_catch_declaration(p).ok();
	parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

	Present(m.complete(p, JS_CATCH_CLAUSE))
}

fn parse_catch_declaration(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['(']) {
		return Absent;
	}

	let declaration_marker = p.start();

	p.bump_any(); // bump (

	let pattern_marker = parse_binding_pattern(p).or_add_diagnostic(p, expected_binding);
	let pattern_kind = pattern_marker.map(|x| x.kind());

	if p.at(T![:]) {
		let error_marker = match pattern_marker {
			Some(pattern_node) => pattern_node.precede(p),
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
		error_marker.complete(
			p,
			pattern_kind
				.filter(|_| p.typescript())
				.unwrap_or(JS_UNKNOWN),
		);

		if !p.typescript() {
			let err = p
				.err_builder("type annotations can only be used in TypeScript files")
				.primary(start..end, "");

			p.error(err);
		}
	}
	p.expect(T![')']);

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
pub fn parse_try_statement(p: &mut Parser) -> ParsedSyntax {
	// TODO: recover from `try catch` and `try finally`. The issue is block_items
	// will cause infinite recursion because parsing a stmt would not consume the catch token
	// and block_items would not exit, and if we exited on any error that would greatly limit
	// block_items error recovery

	if !p.at(T![try]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // eat try

	parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

	let catch = parse_catch_clause(p);

	if p.at(T![finally]) {
		catch.ok();

		let finalizer = p.start();
		p.bump_any();
		parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);
		finalizer.complete(p, JS_FINALLY_CLAUSE);
		Present(m.complete(p, JS_TRY_FINALLY_STATEMENT))
	} else {
		catch.or_add_diagnostic(p, js_parse_error::expected_catch_clause);
		Present(m.complete(p, JS_TRY_STATEMENT))
	}
}
