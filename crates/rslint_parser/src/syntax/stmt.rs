//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::expr::{assign_expr, expr, EXPR_RECOVERY_SET, STARTS_EXPR};
use super::pat::*;
use super::program::{export_decl, import_decl};
use super::typescript::*;
use super::util::{check_for_stmt_declaration, check_label_use, check_lhs};
#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParsedSyntax;
use crate::syntax::class::class_declaration;
use crate::syntax::function::function_declaration;
use crate::syntax::js_parse_error;
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::SyntaxFeature;
use crate::{SyntaxKind::*, *};

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
	EXPORT_KW
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
pub fn semi(p: &mut Parser, err_range: Range<usize>) {
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
#[allow(deprecated)]
pub fn stmt(p: &mut Parser, recovery_set: impl Into<Option<TokenSet>>) -> Option<CompletedMarker> {
	let res = match p.cur() {
		T![;] => empty_stmt(p).ok().unwrap(), // It is only ever Err if there's no ;
		T!['{'] => block_stmt(p).ok().unwrap(), // It is only ever None if there is no `{`,
		T![if] => if_stmt(p).ok().unwrap(),   // It is only ever Err if there's no if
		T![with] => with_stmt(p).ok().unwrap(), // ever only Err if there's no with keyword
		T![while] => while_stmt(p).ok().unwrap(), // It is only ever Err if there's no while keyword
		t if (t == T![const] && p.nth_at(1, T![enum])) || t == T![enum] => {
			let mut res = ts_enum(p);
			res.err_if_not_ts(p, "enums can only be declared in TypeScript files");
			res
		}
		T![var] | T![const] => variable_declaration_statement(p),
		T![for] => for_stmt(p).ok().unwrap(),
		T![do] => do_stmt(p).ok().unwrap(),
		T![switch] => switch_stmt(p),
		T![try] => parse_try_statement(p).ok().unwrap(), // it is only ever Err if there's no try
		T![return] => return_stmt(p),
		T![break] => break_stmt(p).ok().unwrap(),
		T![continue] => continue_stmt(p).ok().unwrap(), // It is only ever Err if there's no continue keyword
		T![throw] => throw_stmt(p).ok().unwrap(),
		T![debugger] => debugger_stmt(p).ok().unwrap(),
		T![function] => function_declaration(p),
		T![class] => class_declaration(p),
		T![ident]
			if p.cur_src() == "async"
				&& p.nth_at(1, T![function])
				&& !p.has_linebreak_before_n(1) =>
		{
			function_declaration(p)
		}

		T![ident] if p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)) => {
			variable_declaration_statement(p)
		}
		// TODO: handle `<T>() => {};` with less of a hack
		_ if p.at_ts(STARTS_EXPR) || p.at(T![<]) => return expr_stmt(p),
		_ => {
			let err = p
				.err_builder("Expected a statement or declaration, but found none")
				.primary(
					p.cur_tok().range,
					"Expected a statement or declaration here",
				);

			// We must explicitly handle this case or else infinite recursion can happen
			if p.at_ts(token_set![T!['}'], T![import], T![export]]) {
				p.err_and_bump(err, JS_UNKNOWN_STATEMENT);
				return None;
			}
			SingleTokenParseRecovery::with_error(
				recovery_set.into().unwrap_or(STMT_RECOVERY_SET),
				JS_UNKNOWN_STATEMENT,
				err,
			)
			.recover(p);
			return None;
		}
	};

	Some(res)
}

// test_err double_label
// label1: {
// 	label2: {
// 		label1: {}
// 	}
// }

#[allow(deprecated)]
fn expr_stmt(p: &mut Parser) -> Option<CompletedMarker> {
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
			return Some(res);
		}
	}

	// module and global are special because its used normally in js a lot so we cant assume its a ts module decl
	if p.cur_src() == "module" || (p.cur_src() == "global" && p.nth_at(1, T!['{'])) {
		if let Some(mut res) = try_parse_ts(p, ts_expr_stmt) {
			res.err_if_not_ts(
				p,
				"TypeScript declarations can only be used in TypeScript files",
			);
			return Some(res);
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

	let mut expr = p.expr_with_semi_recovery(false)?;
	// Labelled stmt
	if expr.kind() == JS_REFERENCE_IDENTIFIER_EXPRESSION && p.at(T![:]) {
		expr.change_kind(p, NAME);
		// Its not possible to have a name without an inner ident token
		let range = p.events[expr.start_pos as usize..]
			.iter()
			.find_map(|x| match x {
				Event::Token {
					kind: T![ident],
					range,
				} => Some(range),
				_ => None,
			})
			.expect(
				"Tried to get the ident of a name node, but there was no ident. This is erroneous",
			);

		let text_range = TextRange::new((range.start as u32).into(), (range.end as u32).into());
		let text = p.source(text_range);
		if let Some(range) = p.state.labels.get(text) {
			let err = p
				.err_builder("Duplicate statement labels are not allowed")
				.secondary(
					range.to_owned(),
					&format!("`{}` is first used as a label here", text),
				)
				.primary(
					text_range,
					&format!("a second use of `{}` here is not allowed", text),
				);

			p.error(err);
		} else {
			let string = text.to_string();
			p.state.labels.insert(string, range.to_owned());
		}

		let m = expr.undo_completion(p);
		p.bump_any();
		stmt(p, None);
		return Some(m.complete(p, JS_LABELED_STATEMENT));
	}

	let m = expr.precede(p);
	semi(p, start..p.cur_tok().range.end);
	Some(m.complete(p, JS_EXPRESSION_STATEMENT))
}

/// A debugger statement such as `debugger;`
// test debugger_stmt
// debugger;

// test_err debugger_stmt
// function foo() {
// 	debugger {
// 		var something = "lorem";
// 	}
// }

pub fn debugger_stmt(p: &mut Parser) -> ParsedSyntax {
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
pub fn throw_stmt(p: &mut Parser) -> ParsedSyntax {
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
	} else {
		p.expr_with_semi_recovery(false);
	}
	semi(p, start..p.cur_tok().range.end);
	Present(m.complete(p, JS_THROW_STATEMENT))
}

/// A break statement with an optional label such as `break a;`
// test break_stmt
// foo: {}
// rust: {}
// break;
// break foo;
// break rust
pub fn break_stmt(p: &mut Parser) -> ParsedSyntax {
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
	}

	Present(m.complete(p, JS_BREAK_STATEMENT))
}

/// A continue statement with an optional label such as `continue a;`
// test continue_stmt
// foo: {}
// while (true) {
//   continue;
//   continue foo;
//   continue
// }
pub fn continue_stmt(p: &mut Parser) -> ParsedSyntax {
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
	}

	Present(m.complete(p, JS_CONTINUE_STATEMENT))
}

/// A return statement with an optional value such as `return a;`
// test return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
#[allow(deprecated)]
pub fn return_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err return_stmt_err
	// return;
	// return foo;
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.expect_required(T![return]);
	if !p.has_linebreak_before_n(0) && p.at_ts(STARTS_EXPR) {
		p.expr_with_semi_recovery(false);
	}
	semi(p, start..p.cur_tok().range.end);
	let complete = m.complete(p, JS_RETURN_STATEMENT);

	if !p.state.in_function && !p.syntax.global_return {
		let err = p
			.err_builder("Illegal return statement outside of a function")
			.primary(complete.range(p), "");

		p.error(err);
	}
	complete
}

/// An empty statement denoted by a single semicolon.
// test empty_stmt
// ;
pub fn empty_stmt(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![;]) {
		let m = p.start();
		p.bump_any(); // bump ;
		m.complete(p, JS_EMPTY_STATEMENT).into()
	} else {
		Absent
	}
}

/// A block statement consisting of statements wrapped in curly brackets.
// test block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
pub(crate) fn block_stmt(p: &mut Parser) -> ParsedSyntax {
	block_impl(p, JS_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn block_impl(p: &mut Parser, block_kind: SyntaxKind) -> ParsedSyntax {
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

	statements(p, false, true, None);

	p.expect_required(T!['}']);

	if let Some(old_parser_state) = old_parser_state {
		p.state = old_parser_state;
	}

	Present(m.complete(p, block_kind))
}

#[must_use]
pub(crate) fn directives(p: &mut Parser) -> Option<ParserState> {
	let list = p.start();

	let mut old_state: Option<ParserState> = None;

	fn is_directive(p: &Parser) -> bool {
		if !p.at(JS_STRING_LITERAL) {
			false
		} else {
			let next = p.nth(1);

			matches!(next, T![;] | EOF | T!['}']) || p.has_linebreak_before_n(1)
		}
	}

	while is_directive(p) {
		let directive_token = p.cur_tok();

		let directive = p.start();
		// bump string token
		p.bump_any();

		// eat semicolon if present, correct termination guaranteed by is_directive
		p.eat(SyntaxKind::SEMICOLON);

		directive.complete(p, JS_DIRECTIVE);

		let directive_text = p.token_src(&directive_token);

		if directive_text == "\"use strict\"" || directive_text == "'use strict'" {
			if old_state == None {
				old_state = Some(p.state.clone());
			}

			let mut new_state = p.state.clone();
			new_state.strict(p, directive_token.range);
			p.state = new_state;
		}
	}

	list.complete(p, LIST);

	old_state
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn statements(
	p: &mut Parser,
	top_level: bool,
	stop_on_r_curly: bool,
	recovery_set: impl Into<Option<TokenSet>>,
) {
	let recovery_set = recovery_set.into();

	let list_start = p.start();

	while !p.at(EOF) {
		if stop_on_r_curly && p.at(T!['}']) {
			break;
		}

		match p.cur() {
			// test_err import_decl_not_top_level
			// {
			//  import foo from "bar";
			// }

			// make sure we dont try parsing import.meta or import() as declarations
			T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
				let mut m = import_decl(p);
				if !p.state.is_module && !p.typescript() {
					let err = p
						.err_builder("Illegal use of an import declaration outside of a module")
						.primary(m.range(p), "not allowed inside scripts");

					p.error(err);
					m.change_kind(p, ERROR);
				}
				if !top_level {
					let err = p
						.err_builder("Illegal use of an import declaration not at the top level")
						.primary(m.range(p), "move this declaration to the top level");

					p.error(err);
					m.change_kind(p, ERROR);
				}
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
					m.change_kind(p, ERROR);
				}
				if !top_level {
					let err = p
						.err_builder("Illegal use of an import declaration not at the top level")
						.primary(m.range(p), "move this declaration to the top level");

					p.error(err);
					m.change_kind(p, ERROR);
				}
			}
			_ => {
				stmt(p, recovery_set);
			}
		};
	}

	list_start.complete(p, LIST);
}

/// An expression wrapped in parentheses such as `()`
pub fn parenthesized_expression(p: &mut Parser) {
	p.state.allow_object_expr = p.expect_required(T!['(']);
	expr(p);
	p.expect_required(T![')']);
	p.state.allow_object_expr = true;
}

/// An if statement such as `if (foo) { bar(); }`
// test if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
pub fn if_stmt(p: &mut Parser) -> ParsedSyntax {
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
	// TODO replace with statement(p).or_recover(STMT_RECOVERY_SET.union(...))
	stmt(p, STMT_RECOVERY_SET.union(token_set![T![else]]));

	// else clause
	if p.at(T![else]) {
		let else_clause = p.start();
		p.bump_any(); // bump else
		stmt(p, None); // stmt(p).into_required();
		else_clause.complete(p, JS_ELSE_CLAUSE);
	}

	Present(m.complete(p, JS_IF_STATEMENT))
}

/// A with statement such as `with (foo) something()`
pub fn with_stmt(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![with]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // with
	parenthesized_expression(p);

	stmt(p, None);

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
pub fn while_stmt(p: &mut Parser) -> ParsedSyntax {
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
	stmt(
		&mut *p.with_state(ParserState {
			break_allowed: true,
			continue_allowed: true,
			..p.state.clone()
		}),
		None,
	);
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
pub fn variable_declaration_statement(p: &mut Parser) -> CompletedMarker {
	// test_err var_decl_err
	// var a =;
	// const a = 5 let b = 5;
	let m = p.start();
	let start = p.cur_tok().range.start;

	variable_declaration(p, false);

	semi(p, start..p.cur_tok().range.start);

	m.complete(p, JS_VARIABLE_DECLARATION_STATEMENT)
}

fn variable_declaration(p: &mut Parser, no_semi: bool) -> CompletedMarker {
	let m = p.start();
	let mut is_const = None;
	let mut is_let = false;

	match p.cur() {
		T![var] => p.bump_any(),
		T![const] => {
			is_const = Some(p.cur_tok().range);
			p.bump_any()
		}
		T![ident] if p.cur_src() == "let" => {
			// let is a valid identifier name that's why the returns an ident for let.
			// remap it here because we know from the context that this is the let keyword.
			p.bump_remap(T![let]);
			is_let = true;
		}
		_ => {
			let err = p
				.err_builder(
					"Expected `var`, `let`, or `const` for a variable declaration, but found none",
				)
				.primary(p.cur_tok().range, "");

			p.error(err);
		}
	}

	let declared_list = p.start();

	variable_declarator(p, &is_const, no_semi, is_let);

	while p.eat(T![,]) {
		variable_declarator(p, &is_const, no_semi, is_let);
	}

	declared_list.complete(p, LIST);
	p.state.name_map.clear();
	m.complete(p, JS_VARIABLE_DECLARATION)
}

// A single declarator, either `ident` or `ident = assign_expr`
pub(crate) fn variable_declarator(
	p: &mut Parser,
	is_const: &Option<Range<usize>>,
	for_stmt: bool,
	is_let: bool,
) -> Option<CompletedMarker> {
	let m = p.start();
	p.state.should_record_names = is_const.is_some() || is_let;
	let pat = if let Some(pattern) = pattern(p, false, false) {
		pattern
	} else {
		m.abandon(p);
		return None;
	};

	let pat_m = pat.undo_completion(p);
	p.state.should_record_names = false;
	let kind = pat.kind();

	let cur = p.cur_tok().range;
	let opt = p.eat(T![!]);
	if opt && !p.typescript() {
		let err = p
			.err_builder("definite assignment assertions can only be used in TypeScript files")
			.primary(cur, "");

		p.error(err);
	}

	if p.eat(T![:]) {
		let start = p.cur_tok().range.start;
		let ty = ts_type(p);
		let end = ty
			.map(|x| usize::from(x.range(p).end()))
			.unwrap_or(p.cur_tok().range.start);
		if !p.typescript() {
			let err = p
				.err_builder("type annotations can only be used in TypeScript files")
				.primary(start..end, "");

			p.error(err);
		}
		if p.typescript() && for_stmt {
			let err = p
				.err_builder("`for` statement declarators cannot have a type annotation")
				.primary(start..end, "");

			p.state.for_head_error = Some(err);
		}
	}

	let marker = pat_m.complete(p, kind);

	if p.at(T![=]) {
		variable_initializer(p);
	} else if marker.kind() != SINGLE_PATTERN && !for_stmt && !p.state.in_declare {
		let err = p
			.err_builder("Object and Array patterns require initializers")
			.primary(
				marker.range(p),
				"this pattern is declared, but it is not given an initialized value",
			);

		p.error(err);
	// FIXME: does ts allow const var declarations without initializers in .d.ts files?
	} else if is_const.is_some() && !for_stmt && !p.state.in_declare {
		let err = p
			.err_builder("Const var declarations must have an initialized value")
			.primary(marker.range(p), "this variable needs to be initialized");

		p.error(err);
	}

	Some(m.complete(p, JS_VARIABLE_DECLARATOR))
}

#[allow(deprecated)]
fn variable_initializer(p: &mut Parser) {
	let m = p.start();

	p.expect_required(T![=]);
	p.expr_with_semi_recovery(true);

	m.complete(p, SyntaxKind::JS_EQUAL_VALUE_CLAUSE);
}

// A do.. while statement, such as `do {} while (true)`
// test do_while_stmtR
// do { } while (true)
// do throw Error("foo") while (true)
pub fn do_stmt(p: &mut Parser) -> ParsedSyntax {
	// test_err do_while_stmt_err
	// do while (true)
	// do while ()
	// do while true
	if !p.at(T![do]) {
		return Absent;
	}
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any(); // do keyword
	stmt(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
	);
	p.expect_required(T![while]);
	parenthesized_expression(p);
	semi(p, start..p.cur_tok().range.end);
	Present(m.complete(p, JS_DO_WHILE_STATEMENT))
}

#[allow(deprecated)]
fn for_head(p: &mut Parser) -> SyntaxKind {
	let m = p.start();
	if p.at(T![const]) || p.at(T![var]) || (p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)))
	{
		let mut guard = p.with_state(ParserState {
			include_in: false,
			..p.state.clone()
		});
		let decl = variable_declaration(&mut *guard, true);
		drop(guard);
		m.complete(p, FOR_STMT_INIT);

		if p.at(T![in]) || p.cur_src() == "of" {
			if let Some(err) = p.state.for_head_error.take() {
				p.error(err);
			}
			let is_in = p.at(T![in]);
			p.bump_any();

			check_for_stmt_declaration(p, &decl);

			for_each_head(p, is_in)
		} else {
			p.state.for_head_error = None;
			p.expect_required(T![;]);
			normal_for_head(p);
			FOR_STMT
		}
	} else {
		if p.eat(T![;]) {
			m.abandon(p);
			normal_for_head(p);
			return FOR_STMT;
		}
		let mut guard = p.with_state(ParserState {
			include_in: false,
			..p.state.clone()
		});
		let complete = expr(&mut *guard);
		drop(guard);
		m.complete(p, FOR_STMT_INIT);

		if p.at(T![in]) || p.cur_src() == "of" {
			let is_in = p.at(T![in]);
			p.bump_any();

			if let Some(ref expr) = complete {
				check_lhs(p, p.parse_marker(expr), &complete.unwrap());
				if p.typescript()
					&& matches!(expr.kind(), JS_ARRAY_EXPRESSION | JS_OBJECT_EXPRESSION)
				{
					let err = p.err_builder("the left hand side of a `for..in` or `for..of` statement cannot be a destructuring pattern")
                        .primary(expr.range(p), "");

					p.error(err);
				}
			}

			return for_each_head(p, is_in);
		}

		p.expect_required(T![;]);
		normal_for_head(p);
		FOR_STMT
	}
}

fn for_each_head(p: &mut Parser, is_in: bool) -> SyntaxKind {
	if is_in {
		expr(p);
		FOR_IN_STMT
	} else {
		assign_expr(p);
		FOR_OF_STMT
	}
}

fn normal_for_head(p: &mut Parser) {
	if !p.eat(T![;]) {
		let m = p.start();
		expr(p);
		m.complete(p, FOR_STMT_TEST);
		p.expect_required(T![;]);
	}

	if !p.at(T![')']) {
		let m = p.start();
		expr(p);
		m.complete(p, FOR_STMT_UPDATE);
	}
}

/// Either a traditional for statement or a for.. in statement
// test for_stmt
// for (let i = 5; i < 10; i++) {}
// for (let { foo, bar } of {}) {}
// for (foo in {}) {}
// for (;;) {}
pub fn for_stmt(p: &mut Parser) -> ParsedSyntax {
	// test_err for_stmt_err
	// for ;; {}
	// for let i = 5; i < 10; i++ {}
	// for let i = 5; i < 10; ++i {}
	if !p.at(T![for]) {
		return Absent;
	}
	let m = p.start();
	p.bump_any(); // for keyword

	// FIXME: This should emit an error for non-for-of
	p.eat(T![await]);

	p.expect_required(T!['(']);
	let kind = for_head(p);
	p.expect_required(T![')']);
	stmt(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
	);
	Present(m.complete(p, kind))
}

// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn switch_clause(p: &mut Parser) -> Option<Range<usize>> {
	let start = p.cur_tok().range.start;
	let m = p.start();
	match p.cur() {
		T![default] => {
			p.bump_any();
			p.expect_required(T![:]);
			// We stop the range here because we dont want to include the entire clause
			// including the statement list following it
			let end = p.cur_tok().range.end;
			let cons_list = p.start();
			while !p.at_ts(token_set![T![default], T![case], T!['}'], EOF]) {
				stmt(p, None);
			}
			cons_list.complete(p, LIST);
			m.complete(p, JS_DEFAULT_CLAUSE);
			return Some(start..end);
		}
		T![case] => {
			p.bump_any();
			expr(p);
			p.expect_required(T![:]);
			let cons_list = p.start();
			while !p.at_ts(token_set![T![default], T![case], T!['}'], EOF]) {
				stmt(p, None);
			}
			cons_list.complete(p, LIST);
			m.complete(p, JS_CASE_CLAUSE);
		}
		_ => {
			m.abandon(p);
			let err = p
				.err_builder(
					"Expected a `case` or `default` clause in a switch statement, but found none",
				)
				.primary(
					p.cur_tok().range,
					"Expected the start to a case or default clause here",
				);

			#[allow(deprecated)]
			SingleTokenParseRecovery::with_error(STMT_RECOVERY_SET, JS_UNKNOWN_STATEMENT, err)
				.enabled_braces_check()
				.recover(p);
		}
	}
	None
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
pub fn switch_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err switch_stmt_err
	// switch foo {}
	// switch {}
	let m = p.start();
	p.expect_required(T![switch]);
	parenthesized_expression(p);
	p.expect_required(T!['{']);
	let cases_list = p.start();
	let mut first_default: Option<Range<usize>> = None;

	while !p.at(EOF) && !p.at(T!['}']) {
		let mut temp = p.with_state(ParserState {
			break_allowed: true,
			..p.state.clone()
		});
		if let Some(default_range) = switch_clause(&mut *temp) {
			if let Some(ref err_range) = first_default {
				let err = temp
					.err_builder(
						"Multiple default clauses inside of a switch statement are not allowed",
					)
					.secondary(
						err_range.to_owned(),
						"the first default clause is defined here",
					)
					.primary(default_range, "a second clause here is not allowed");

				temp.error(err);
			} else {
				first_default = Some(default_range);
			}
		}
	}
	cases_list.complete(p, LIST);
	p.expect_required(T!['}']);
	m.complete(p, JS_SWITCH_STATEMENT)
}

fn parse_catch_clause(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![catch]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any(); // bump catch

	catch_declaration(p).or_missing(p);
	block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);

	Present(m.complete(p, JS_CATCH_CLAUSE))
}

fn catch_declaration(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['(']) {
		return Absent;
	}

	let declaration_marker = p.start();

	p.bump_any(); // bump (

	let pattern_marker = pattern(p, false, false);
	let pattern_kind = pattern_marker.map(|x| x.kind());

	if p.at(T![:]) {
		let error_marker = pattern_marker
			.map(|m| m.precede(p))
			.unwrap_or_else(|| p.start());
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

	block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);

	let catch = parse_catch_clause(p);

	if p.at(T![finally]) {
		catch.or_missing(p);

		let finalizer = p.start();
		p.bump_any();
		block_stmt(p).or_missing_with_error(p, js_parse_error::expected_block_statement);
		finalizer.complete(p, JS_FINALLY_CLAUSE);
		Present(m.complete(p, JS_TRY_FINALLY_STATEMENT))
	} else {
		catch.or_missing_with_error(p, js_parse_error::expected_catch_clause);
		Present(m.complete(p, JS_TRY_STATEMENT))
	}
}
