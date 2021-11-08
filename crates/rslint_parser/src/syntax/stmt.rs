//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::decl::{class_decl, decorators, function_decl};
use super::expr::{assign_expr, expr, primary_expr, EXPR_RECOVERY_SET, STARTS_EXPR};
use super::pat::*;
use super::program::{export_decl, import_decl};
use super::typescript::*;
use super::util::{check_for_stmt_declarators, check_label_use, check_lhs};
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
	if p.eat(T![;]) || p.at(EOF) || p.at(T!['}']) {
		return;
	}
	if !p.has_linebreak_before_n(0) {
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

/// A generic statement such as a block, if, while, with, etc
pub fn stmt(
	p: &mut Parser,
	recovery_set: impl Into<Option<TokenSet>>,
	decorator: Option<CompletedMarker>,
) -> Option<CompletedMarker> {
	let decorator = if p.at(T![@]) {
		decorators(p).into_iter().next() // should be always Some
	} else {
		decorator
	};
	let res = match p.cur() {
		T![;] => empty_stmt(p),
		T!['{'] => block_stmt(p, false, recovery_set).unwrap(), // It is only ever None if there is no `{`,
		T![if] => if_stmt(p),
		T![with] => with_stmt(p),
		T![while] => while_stmt(p),
		t if (t == T![const] && p.nth_at(1, T![enum])) || t == T![enum] => {
			let mut res = ts_enum(p);
			res.err_if_not_ts(p, "enums can only be declared in TypeScript files");
			res
		}
		T![var] | T![const] => var_decl(p, false),
		T![for] => for_stmt(p),
		T![do] => do_stmt(p),
		T![switch] => switch_stmt(p),
		T![try] => try_stmt(p),
		T![return] => return_stmt(p),
		T![break] => break_stmt(p),
		T![continue] => continue_stmt(p),
		T![throw] => throw_stmt(p),
		T![debugger] => debugger_stmt(p),
		T![function] => {
			p.state.decorators_were_valid = true;
			let m = decorator.map(|x| x.precede(p)).unwrap_or_else(|| p.start());
			// TODO: Should we change this to fn_expr if there is no name?
			function_decl(p, m, true)
		}
		T![class] => {
			p.state.decorators_were_valid = true;
			let complete = class_decl(p, false);
			if let Some(decorator) = decorator {
				let new = decorator.precede(p);
				complete.undo_completion(p);
				new.complete(p, CLASS_DECL)
			} else {
				complete
			}
		}
		T![ident]
			if p.cur_src() == "async"
				&& p.nth_at(1, T![function])
				&& !p.has_linebreak_before_n(1) =>
		{
			p.state.decorators_were_valid = true;
			let m = decorator.map(|x| x.precede(p)).unwrap_or_else(|| p.start());
			p.bump_any();
			function_decl(
				&mut *p.with_state(ParserState {
					in_async: true,
					..p.state.clone()
				}),
				m,
				true,
			)
		}

		T![ident] if p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)) => var_decl(p, false),
		// TODO: handle `<T>() => {};` with less of a hack
		_ if p.at_ts(STARTS_EXPR) || p.at(T![<]) => {
			let complete = expr_stmt(p, decorator);
			if let Some(decorator) = decorator.filter(|_| !p.state.decorators_were_valid) {
				let err = p
					.err_builder("decorators are not valid in this position")
					.primary(decorator.range(p), "");

				p.error(err);
			}
			p.state.decorators_were_valid = false;
			return complete;
		}
		_ => {
			let err = p
				.err_builder("Expected a statement or declaration, but found none")
				.primary(
					p.cur_tok().range,
					"Expected a statement or declaration here",
				);

			// We must explicitly handle this case or else infinite recursion can happen
			if p.at_ts(token_set![T!['}'], T![import], T![export]]) {
				p.err_and_bump(err);
				return None;
			}

			p.err_recover(err, recovery_set.into().unwrap_or(STMT_RECOVERY_SET), false);
			return None;
		}
	};

	if let Some(decorator) = decorator.filter(|_| !p.state.decorators_were_valid) {
		let err = p
			.err_builder("decorators are not valid in this position")
			.primary(decorator.range(p), "");

		p.error(err);
	}
	p.state.decorators_were_valid = false;

	Some(res)
}

fn expr_stmt(p: &mut Parser, decorator: Option<CompletedMarker>) -> Option<CompletedMarker> {
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
			if let Some(decorator) = decorator {
				let kind = res.kind();
				let m = decorator.precede(p);
				res.undo_completion(p);
				res = m.complete(p, kind);
			}
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
	if expr.kind() == NAME_REF && p.at(T![:]) {
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
					p.cur_tok().range,
					&format!("a second use of `{}` here is not allowed", text),
				);

			p.error(err);
		} else {
			let string = text.to_string();
			p.state.labels.insert(string, range.to_owned());
		}

		let m = expr.precede(p);
		p.bump_any();
		stmt(p, None, None);
		return Some(m.complete(p, LABELLED_STMT));
	}

	let m = expr.precede(p);
	semi(p, start..p.cur_tok().range.end);
	Some(m.complete(p, EXPR_STMT))
}

/// A debugger statement such as `debugger;`
// test debugger_stmt
// debugger;
pub fn debugger_stmt(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	let range = p.cur_tok().range;
	p.expect(T![debugger]);
	semi(p, range);
	m.complete(p, DEBUGGER_STMT)
}

/// A throw statement such as `throw new Error("uh oh");`
// test throw_stmt
// throw new Error("foo");
// throw "foo"
pub fn throw_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err throw_stmt_err
	// throw
	// new Error("oh no :(")
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.expect(T![throw]);
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
	m.complete(p, THROW_STMT)
}

/// A break statement with an optional label such as `break a;`
// test break_stmt
// foo: {}
// rust: {}
// break;
// break foo;
// break rust
pub fn break_stmt(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	let start = p.cur_tok().range;
	p.expect(T![break]);
	let end = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
		let end = p.cur_tok().range.end;
		let label = primary_expr(p).unwrap();
		check_label_use(p, &label);
		end
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

	m.complete(p, BREAK_STMT)
}

/// A continue statement with an optional label such as `continue a;`
// test continue_stmt
// foo: {}
// while (true) {
//   continue;
//   continue foo;
//   continue
// }
pub fn continue_stmt(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	let start = p.cur_tok().range;
	p.expect(T![continue]);
	let end = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
		let end = p.cur_tok().range.end;
		let mut guard = p.with_state(ParserState {
			expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![;]]),
			..p.state.clone()
		});
		let label = primary_expr(&mut *guard).unwrap();
		drop(guard);
		check_label_use(p, &label);
		end
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

	m.complete(p, CONTINUE_STMT)
}

/// A return statement with an optional value such as `return a;`
// test return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
pub fn return_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err return_stmt_err
	// return;
	// return foo;
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.expect(T![return]);
	if !p.has_linebreak_before_n(0) && p.at_ts(STARTS_EXPR) {
		p.expr_with_semi_recovery(false);
	}
	semi(p, start..p.cur_tok().range.end);
	let complete = m.complete(p, RETURN_STMT);

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
pub fn empty_stmt(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![;]);
	m.complete(p, EMPTY_STMT)
}

/// A block statement consisting of statements wrapped in curly brackets.
// test block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
pub fn block_stmt(
	p: &mut Parser,
	function_body: bool,
	recovery_set: impl Into<Option<TokenSet>>,
) -> Option<CompletedMarker> {
	if !p.at(T!['{']) {
		let err = p
			.err_builder(&format!(
				"expected a block statement but instead found `{}`",
				p.cur_src()
			))
			.primary(p.cur_tok().range, "");

		p.error(err);
		return None;
	}
	let m = p.start();
	let mut guard = p.with_state(ParserState {
		in_function: p.state.in_function || function_body,
		..p.state.clone()
	});
	guard.bump(T!['{']);
	block_items(&mut *guard, function_body, false, true, recovery_set);
	guard.expect(T!['}']);
	Some(m.complete(&mut *guard, BLOCK_STMT))
}

pub fn block_stmt_unchecked(p: &mut Parser, function_body: bool) -> CompletedMarker {
	let m = p.start();
	p.bump(T!['{']);
	block_items(p, function_body, false, true, None);
	p.expect(T!['}']);
	m.complete(p, BLOCK_STMT)
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn block_items(
	p: &mut Parser,
	directives: bool,
	top_level: bool,
	stop_on_r_curly: bool,
	recovery_set: impl Into<Option<TokenSet>>,
) {
	let old = p.state.clone();
	let recovery_set = recovery_set.into();

	let mut could_be_directive = directives;

	let list_start = p.start();

	while !p.at(EOF) {
		if stop_on_r_curly && p.at(T!['}']) {
			break;
		}

		let decorator = if p.at(T![@]) {
			decorators(p).into_iter().next()
		} else {
			None
		};
		let mut is_import_export = false;

		let complete = match p.cur() {
			// test_err import_decl_not_top_level
			// {
			//  import foo from "bar";
			// }

			// make sure we dont try parsing import.meta or import() as declarations
			T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
				is_import_export = true;
				let mut m = import_decl(p);
				if let Some(decorator) = decorator {
					let kind = m.kind();
					let new = decorator.precede(p);
					m.undo_completion(p);
					m = new.complete(p, kind)
				}
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
				Some(m)
			}
			// test_err export_decl_not_top_level
			// {
			//  export { pain } from "life";
			// }
			T![export] => {
				is_import_export = true;
				let mut m = export_decl(p);
				if let Some(decorator) = decorator {
					let kind = m.kind();
					let new = decorator.precede(p);
					m.undo_completion(p);
					m = new.complete(p, kind)
				}
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
				Some(m)
			}
			_ => stmt(p, recovery_set, decorator),
		};

		if let Some(decorator) =
			decorator.filter(|_| !p.state.decorators_were_valid && is_import_export)
		{
			let err = p
				.err_builder("decorators are not valid in this position")
				.primary(decorator.range(p), "");

			p.error(err);
		}
		p.state.decorators_were_valid = false;

		// Directives are the longest sequence of string literals, so
		// ```
		// function a() {
		//  "aaa";
		//  "use strict"
		// }
		// ```
		// Still makes the function body strict
		if let Some(kind) = complete.map(|x| x.kind()).filter(|_| could_be_directive) {
			match kind {
				EXPR_STMT => {
					let parsed = p
						.parse_marker::<ast::ExprStmt>(complete.as_ref().unwrap())
						.expr();
					if let Some(LITERAL) = parsed.as_ref().map(|it| it.syntax().kind()) {
						let unwrapped = parsed.unwrap().syntax().to::<ast::Literal>();
						if unwrapped.is_string() {
							if unwrapped.inner_string_text().unwrap() == "use strict" {
								let range = complete.as_ref().unwrap().range(p).into();
								// We must do this because we cannot have multiple mutable borrows of p
								let mut new = p.state.clone();
								new.strict(p, range);
								p.state = new;
								could_be_directive = false;
							}
						} else {
							could_be_directive = false;
						}
					}
				}
				_ => could_be_directive = false,
			}
		}
	}

	list_start.complete(p, LIST);

	p.state = old;
}

/// An expression wrapped in parentheses such as `()`
pub fn condition(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.state.allow_object_expr = p.expect(T!['(']);
	expr(p);
	p.expect(T![')']);
	p.state.allow_object_expr = true;
	m.complete(p, CONDITION)
}

/// An if statement such as `if (foo) { bar(); }`
// test if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
pub fn if_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err if_stmt_err
	// if (true) else {}
	// if (true) else
	// if else {}
	// if () {} else {}
	let m = p.start();
	p.expect(T![if]);
	condition(&mut *p.with_state(ParserState {
		expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![else]]),
		..p.state.clone()
	}));
	// allows us to recover from `if (true) else {}`
	stmt(p, STMT_RECOVERY_SET.union(token_set![T![else]]), None);
	if p.eat(T![else]) {
		stmt(p, None, None);
	}
	m.complete(p, IF_STMT)
}

/// A with statement such as `with (foo) something()`
pub fn with_stmt(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![with]);
	condition(p);
	stmt(p, None, None);

	let mut complete = m.complete(p, WITH_STMT);
	if p.state.strict.is_some() {
		let err = p
			.err_builder("`with` statements are not allowed in strict mode")
			.primary(complete.range(p), "");

		p.error(err);
		complete.change_kind(p, ERROR);
	}

	complete
}

/// A while statement such as `while(true) { do_something() }`
// test while_stmt
// while (true) {}
// while (5) {}
pub fn while_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err while_stmt_err
	// while true {}
	// while {}
	// while (true {}
	// while true) }
	let m = p.start();
	p.expect(T![while]);
	condition(p);
	stmt(
		&mut *p.with_state(ParserState {
			break_allowed: true,
			continue_allowed: true,
			..p.state.clone()
		}),
		None,
		None,
	);
	m.complete(p, WHILE_STMT)
}

/// A var, const, or let declaration such as `var a = 5, b;` or `let {a, b} = foo;`
// test var_decl
// var a = 5;
// let { foo, bar } = 5;
// let bar, foo;
// const a = 5;
// const { foo: [bar], baz } = {};
pub fn var_decl(p: &mut Parser, no_semi: bool) -> CompletedMarker {
	// test_err var_decl_err
	// var a =;
	// const a = 5 let b = 5;
	let m = p.start();
	let start = p.cur_tok().range.start;
	let mut is_const = None;
	let mut is_let = false;

	match p.cur() {
		T![var] => p.bump_any(),
		T![const] => {
			is_const = Some(p.cur_tok().range);
			p.bump_any()
		}
		T![ident] if p.cur_src() == "let" => {
			p.bump_any();
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

	declarator(p, &is_const, no_semi, is_let);

	if p.eat(T![,]) {
		declarator(p, &is_const, no_semi, is_let);
		while p.eat(T![,]) {
			declarator(p, &is_const, no_semi, is_let);
		}
	}

	declared_list.complete(p, LIST);

	if !no_semi {
		semi(p, start..p.cur_tok().range.start);
	}
	let complete = m.complete(p, VAR_DECL);
	p.state.name_map.clear();

	complete
}

// A single declarator, either `ident` or `ident = assign_expr`
fn declarator(
	p: &mut Parser,
	is_const: &Option<Range<usize>>,
	for_stmt: bool,
	is_let: bool,
) -> Option<CompletedMarker> {
	let m = p.start();
	p.state.should_record_names = is_const.is_some() || is_let;
	let pat_m = p.start();
	let pat = pattern(p, false, false)?;
	pat.undo_completion(p).abandon(p);
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

	if p.eat(T![=]) {
		p.expr_with_semi_recovery(true);
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

	Some(m.complete(p, DECLARATOR))
}

// A do.. while statement, such as `do {} while (true)`
// test do_while_stmt
// do { } while (true)
// do throw Error("foo") while (true)
pub fn do_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err do_while_stmt_err
	// do while (true)
	// do while ()
	// do while true
	let m = p.start();
	let start = p.cur_tok().range.start;
	p.expect(T![do]);
	stmt(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
		None,
	);
	p.expect(T![while]);
	condition(p);
	semi(p, start..p.cur_tok().range.end);
	m.complete(p, DO_WHILE_STMT)
}

fn for_head(p: &mut Parser) -> SyntaxKind {
	let m = p.start();
	if p.at(T![const]) || p.at(T![var]) || (p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1)))
	{
		let mut guard = p.with_state(ParserState {
			include_in: false,
			..p.state.clone()
		});
		let decl = var_decl(&mut *guard, true);
		drop(guard);
		m.complete(p, FOR_STMT_INIT);

		if p.at(T![in]) || p.cur_src() == "of" {
			if let Some(err) = p.state.for_head_error.take() {
				p.error(err);
			}
			let is_in = p.at(T![in]);
			p.bump_any();

			check_for_stmt_declarators(p, &decl);

			for_each_head(p, is_in)
		} else {
			p.state.for_head_error = None;
			p.expect(T![;]);
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
				if p.typescript() && matches!(expr.kind(), ARRAY_EXPR | OBJECT_EXPR) {
					let err = p.err_builder("the left hand side of a `for..in` or `for..of` statement cannot be a destructuring pattern")
                        .primary(expr.range(p), "");

					p.error(err);
				}
			}

			return for_each_head(p, is_in);
		}

		p.expect(T![;]);
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
		p.expect(T![;]);
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
pub fn for_stmt(p: &mut Parser) -> CompletedMarker {
	// test_err for_stmt_err
	// for ;; {}
	// for let i = 5; i < 10; i++ {}
	let m = p.start();
	p.expect(T![for]);
	// FIXME: This should emit an error for non-for-of
	p.eat(T![await]);

	p.expect(T!['(']);
	let kind = for_head(p);
	p.expect(T![')']);
	stmt(
		&mut *p.with_state(ParserState {
			continue_allowed: true,
			break_allowed: true,
			..p.state.clone()
		}),
		None,
		None,
	);
	m.complete(p, kind)
}

// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn switch_clause(p: &mut Parser) -> Option<Range<usize>> {
	let start = p.cur_tok().range.start;
	let m = p.start();
	match p.cur() {
		T![default] => {
			p.bump_any();
			p.expect(T![:]);
			// We stop the range here because we dont want to include the entire clause
			// including the statement list following it
			let end = p.cur_tok().range.end;
			let cons_list = p.start();
			while !p.at_ts(token_set![T![default], T![case], T!['}'], EOF]) {
				stmt(p, None, None);
			}
			cons_list.complete(p, LIST);
			m.complete(p, DEFAULT_CLAUSE);
			return Some(start..end);
		}
		T![case] => {
			p.bump_any();
			expr(p);
			p.expect(T![:]);
			let cons_list = p.start();
			while !p.at_ts(token_set![T![default], T![case], T!['}'], EOF]) {
				stmt(p, None, None);
			}
			cons_list.complete(p, LIST);
			m.complete(p, CASE_CLAUSE);
		}
		_ => {
			let err = p
				.err_builder(
					"Expected a `case` or `default` clause in a switch statement, but found none",
				)
				.primary(
					p.cur_tok().range,
					"Expected the start to a case or default clause here",
				);

			p.err_recover(err, STMT_RECOVERY_SET, true);
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
	p.expect(T![switch]);
	condition(p);
	p.expect(T!['{']);
	let cases_list = p.start();
	let mut first_default: Option<Range<usize>> = None;

	while !p.at(EOF) && !p.at(T!['}']) {
		let mut temp = p.with_state(ParserState {
			break_allowed: true,
			..p.state.clone()
		});
		if let Some(range) = switch_clause(&mut *temp) {
			if let Some(ref err_range) = first_default {
				let err = temp
					.err_builder(
						"Multiple default clauses inside of a switch statement are not allowed",
					)
					.secondary(
						err_range.to_owned(),
						"the first default clause is defined here",
					)
					.primary(range, "a second clause here is not allowed");

				temp.error(err);
			} else {
				first_default = Some(range);
			}
		}
	}
	cases_list.complete(p, LIST);
	p.expect(T!['}']);
	m.complete(p, SWITCH_STMT)
}

fn catch_clause(p: &mut Parser) {
	let m = p.start();
	p.expect(T![catch]);

	if p.eat(T!['(']) {
		let m = p.start();
		let kind = pattern(p, false, false).map(|x| x.kind());
		if p.at(T![:]) {
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
			m.complete(p, kind.filter(|_| p.typescript()).unwrap_or(ERROR));
			if !p.typescript() {
				let err = p
					.err_builder("type annotations can only be used in TypeScript files")
					.primary(start..end, "");

				p.error(err);
			}
		} else {
			m.abandon(p);
		}
		p.expect(T![')']);
	}

	block_stmt(p, false, STMT_RECOVERY_SET);
	m.complete(p, CATCH_CLAUSE);
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
// try {} catch (e) {}
// try {} catch {} finally {}
pub fn try_stmt(p: &mut Parser) -> CompletedMarker {
	// TODO: recover from `try catch` and `try finally`. The issue is block_items
	// will cause infinite recursion because parsing a stmt would not consume the catch token
	// and block_items would not exit, and if we exited on any error that would greatly limit
	// block_items error recovery
	let m = p.start();
	p.expect(T![try]);
	block_stmt(p, false, None);
	if p.at(T![catch]) {
		catch_clause(p);
	}
	if p.at(T![finally]) {
		let finalizer = p.start();
		p.bump_any();
		block_stmt(p, false, None);
		finalizer.complete(p, FINALIZER);
	}
	m.complete(p, TRY_STMT)
}
