//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-11).

use super::binding::parse_binding_pattern;
use super::decl::{parse_arrow_body, parse_parameter_list};
use super::typescript::*;
use super::util::*;
#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::{ParserProgress, RecoveryResult};
use crate::state::{InAsync, InConditionExpression, PotentialArrowStart};
use crate::syntax::assignment::{
	expression_to_assignment, expression_to_assignment_pattern, parse_assignment,
	AssignmentExprPrecedence,
};
use crate::syntax::binding::{parse_binding, parse_identifier_binding};
use crate::syntax::class::parse_class_expression;
use crate::syntax::function::parse_function_expression;
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
	expected_binding, expected_expression, expected_identifier, expected_parameter,
	expected_simple_assignment_target,
};
use crate::syntax::object::parse_object_expression;
use crate::syntax::stmt::{is_semi, STMT_RECOVERY_SET};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;

pub const EXPR_RECOVERY_SET: TokenSet = token_set![VAR_KW, R_PAREN, L_PAREN, L_BRACK, R_BRACK];

pub const ASSIGN_TOKENS: TokenSet = token_set![
	T![=],
	T![+=],
	T![-=],
	T![*=],
	T![/=],
	T![%=],
	T![<<=],
	T![>>=],
	T![>>>=],
	T![&=],
	T![|=],
	T![^=],
	T![&&=],
	T![||=],
	T![??=],
	T![**=],
];

const STARTS_EXPR: TokenSet = token_set![
	T![!],
	T!['('],
	T!['['],
	T!['{'],
	T![++],
	T![--],
	T![~],
	T![+],
	T![-],
	T![throw],
	T![new],
	T![typeof],
	T![void],
	T![delete],
	T![ident],
	T![...],
	T![this],
	T![yield],
	T![enum],
	T![await],
	T![function],
	T![class],
	T![import],
	T![super],
	BACKTICK,
	TRUE_KW,
	FALSE_KW,
	JS_NUMBER_LITERAL,
	JS_STRING_LITERAL,
	NULL_KW,
	JS_REGEX_LITERAL
];

/// Parses an expression or recovers to the point of where the next statement starts
pub fn parse_expression_or_recover_to_next_statement(
	p: &mut Parser,
	assign: bool,
) -> RecoveryResult {
	let func = if assign {
		syntax::expr::parse_expr_or_assignment
	} else {
		syntax::expr::parse_expression
	};

	func(p).or_recover(
		p,
		&ParseRecovery::new(
			JsSyntaxKind::JS_UNKNOWN_EXPRESSION,
			STMT_RECOVERY_SET.union(token_set![T!['}']]),
		)
		.enable_recovery_on_line_break(),
		expected_expression,
	)
}

/// A literal expression.
///
/// `TRUE | FALSE | NUMBER | STRING | NULL`
// test literals
// 5
// true
// false
// 5n
// "foo"
// 'bar'
// null
pub(super) fn parse_literal_expression(p: &mut Parser) -> ParsedSyntax {
	let literal_kind = match p.cur_tok().kind {
		JsSyntaxKind::JS_NUMBER_LITERAL => {
			if p.cur_src().ends_with('n') {
				let m = p.start();
				p.bump_remap(JsSyntaxKind::JS_BIG_INT_LITERAL);
				return Present(m.complete(p, JS_BIG_INT_LITERAL_EXPRESSION));
			};

			JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION
		}
		JsSyntaxKind::JS_STRING_LITERAL => JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
		JsSyntaxKind::NULL_KW => JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
		JsSyntaxKind::TRUE_KW | JsSyntaxKind::FALSE_KW => {
			JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION
		}
		JsSyntaxKind::JS_REGEX_LITERAL => JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
		_ => return Absent,
	};

	let m = p.start();
	p.bump_any();
	Present(m.complete(p, literal_kind))
}

/// Parses an expression that might turn out to be an assignment target if an assignment operator is found
pub(crate) fn parse_expr_or_assignment(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![<]) && is_nth_at_name(p, 1) {
		let res = try_parse_ts(p, |p| {
			let m = p.start();
			if ts_type_params(p).is_none() {
				m.abandon(p);
				return None;
			}

			let res = parse_assign_expr_base(p);
			if res.kind() == Some(JS_ARROW_FUNCTION_EXPRESSION) {
				m.abandon(p);
				return None;
			}
			res.abandon(p);
			Some(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
		});
		if let Some(mut res) = res {
			res.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
			return Present(res);
		}
	}
	parse_assign_expr_base(p)
}

fn parse_assign_expr_base(p: &mut Parser) -> ParsedSyntax {
	if p.state.in_generator() && p.at(T![yield]) {
		return Present(yield_expr(p));
	}
	let potential_arrow_start = p.at(T!['(']) | is_at_identifier(p);

	p.with_state(PotentialArrowStart(potential_arrow_start), |p| {
		let checkpoint = p.checkpoint();

		parse_conditional_expr(p)
			.and_then(|target| parse_assign_expr_recursive(p, target, checkpoint))
	})
}

// test assign_expr
// foo += bar = b ??= 3;
// foo -= bar;
// (foo = bar);
// [foo, bar] = baz;
// [foo, bar = "default", ...rest] = baz;
// [,,,foo,bar] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});

// test_err assign_expr_right
// (foo = );

// test_err assign_expr_left
// ( = foo);
fn parse_assign_expr_recursive(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> ParsedSyntax {
	if p.at_ts(ASSIGN_TOKENS) {
		let target = expression_to_assignment_pattern(
			p,
			target,
			checkpoint,
			AssignmentExprPrecedence::Conditional,
		);
		let m = target.precede(p);
		p.bump_any(); // operator
		parse_expr_or_assignment(p)
			.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
		Present(m.complete(p, JS_ASSIGNMENT_EXPRESSION))
	} else {
		Present(target)
	}
}

// test yield_expr
// function *foo() {
//  yield foo;
//  yield* foo;
//  yield;
//  yield
//  yield
// }
fn yield_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![yield]);

	if !is_semi(p, 0) && (p.at(T![*]) || p.at_ts(STARTS_EXPR)) {
		let argument = p.start();
		p.eat(T![*]);
		parse_expr_or_assignment(p).ok();

		argument.complete(p, JS_YIELD_ARGUMENT);
	}

	m.complete(p, JS_YIELD_EXPRESSION)
}

/// A conditional expression such as `foo ? bar : baz`
// test conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz
pub(super) fn parse_conditional_expr(p: &mut Parser) -> ParsedSyntax {
	// test_err conditional_expr_err
	// foo ? bar baz
	// foo ? bar baz ? foo : bar
	// foo ? bar :
	let lhs = parse_binary_or_logical_expression(p);

	if p.at(T![?]) {
		return lhs.map(|marker| {
			let m = marker.precede(p);
			p.bump_any();

			p.with_state(InConditionExpression(true), parse_expr_or_assignment)
				.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

			p.expect(T![:]);
			parse_expr_or_assignment(p)
				.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
			m.complete(p, JS_CONDITIONAL_EXPRESSION)
		});
	}
	lhs
}

/// A binary expression such as `2 + 2` or `foo * bar + 2` or a logical expression 'a || b'
fn parse_binary_or_logical_expression(p: &mut Parser) -> ParsedSyntax {
	let left = parse_unary_expr(p);

	parse_binary_or_logical_expression_recursive(p, left, 0)
}

// test binary_expressions
// 5 * 5
// 6 ** 6 ** 7
// 1 + 2 * 3
// (1 + 2) * 3
// 1 / 2
// 74 in foo
// foo instanceof Array
// foo ?? bar
// a >> b
// a >>> b
// 1 + 1 + 1 + 1
// 5 + 6 - 1 * 2 / 1 ** 6

// test_err binary_expressions_err
// foo(foo +);
// foo + * 2;
// !foo * bar;
fn parse_binary_or_logical_expression_recursive(
	p: &mut Parser,
	left: ParsedSyntax,
	min_prec: u8,
) -> ParsedSyntax {
	if 7 > min_prec && !p.has_linebreak_before_n(0) && p.cur_src() == "as" {
		let m = left.precede(p);
		p.bump_any();
		let mut res = if p.eat(T![const]) {
			m.complete(p, TS_CONST_ASSERTION)
		} else {
			ts_type(p);
			m.complete(p, TS_ASSERTION)
		};
		res.err_if_not_ts(p, "type assertions can only be used in TypeScript files");
		return parse_binary_or_logical_expression_recursive(p, Present(res), min_prec);
	}
	let kind = match p.cur() {
		T![>] if p.nth_at(1, T![>]) && p.nth_at(2, T![>]) => T![>>>],
		T![>] if p.nth_at(1, T![>]) => T![>>],
		k => k,
	};

	let precedence = match kind {
		T![in] if p.state.include_in() => 7,
		T![instanceof] => 7,
		_ => {
			if let Some(prec) = get_precedence(kind) {
				prec
			} else {
				return left;
			}
		}
	};

	if precedence <= min_prec {
		return left;
	}

	let op = kind;
	let op_tok = p.cur_tok();

	let m = left.precede(p);

	if op == T![>>] {
		p.bump_multiple(2, T![>>]);
	} else if op == T![>>>] {
		p.bump_multiple(3, T![>>>]);
	} else {
		p.bump_any();
	}

	// test logical_expressions
	// foo ?? bar
	// a || b
	// a && b
	//
	// test_err logical_expressions_err
	// foo ?? * 2;
	// !foo && bar;
	// foo(foo ||)
	let expression_kind = match op {
		T![??] | T![||] | T![&&] => JS_LOGICAL_EXPRESSION,
		_ => JS_BINARY_EXPRESSION,
	};

	// This is a hack to allow us to effectively recover from `foo + / bar`
	let right = if get_precedence(p.cur()).is_some() && !p.at_ts(token_set![T![-], T![+], T![<]]) {
		let err = p.err_builder(&format!("Expected an expression for the right hand side of a `{}`, but found an operator instead", p.token_src(op_tok)))
            .secondary(op_tok.range(), "This operator requires a right hand side value")
            .primary(p.cur_tok().range(), "But this operator was encountered instead");

		p.error(err);

		parse_binary_or_logical_expression_recursive(p, Absent, 0)
	} else {
		parse_unary_expr(p)
	};

	parse_binary_or_logical_expression_recursive(
		p,
		right,
		// ** is right recursive
		if op == T![**] {
			precedence - 1
		} else {
			precedence
		},
	)
	.or_add_diagnostic(p, expected_expression);

	let complete = m.complete(p, expression_kind);
	parse_binary_or_logical_expression_recursive(p, Present(complete), min_prec)

	// FIXME(RDambrosio016): We should check for nullish-coalescing and logical expr being used together,
	// however, i can't figure out a way to do this efficiently without using parse_marker which is way too
	// expensive to use since this is a hot path
}

/// A member or new expression with subscripts. e.g. `new foo`, `new Foo()`, `foo`, or `foo().bar[5]`
// test new_exprs
// new Foo()
// new foo;
// new.target
// new new new new Foo();
// new Foo(bar, baz, 6 + 6, foo[bar] + (foo) => {} * foo?.bar)

// test_err new_exprs
// new;
fn parse_member_or_new_expr(p: &mut Parser, new_expr: bool) -> ParsedSyntax {
	if p.at(T![new]) {
		// We must start the marker here and not outside or else we make
		// a needless node if the node ends up just being a primary expr
		let m = p.start();
		p.bump_any();

		// new.target
		if p.at(T![.]) && p.token_src(p.nth_tok(1)) == "target" {
			p.bump_any();
			p.bump_remap(T![target]);
			let complete = m.complete(p, NEW_TARGET);
			return Present(subscripts(p, complete, true));
		}

		let complete = parse_member_or_new_expr(p, new_expr);
		if complete.kind() == Some(JS_ARROW_FUNCTION_EXPRESSION) {
			m.abandon(p);
			return complete;
		}

		complete.or_add_diagnostic(p, expected_expression);

		if p.at(T![<]) {
			if let Some(mut complete) = try_parse_ts(p, |p| {
				let compl = ts_type_args(p);
				if !p.at(T!['(']) {
					return None;
				}
				compl
			}) {
				complete.err_if_not_ts(
					p,
					"`new` expressions can only have type arguments in TypeScript files",
				);
			}
		}

		if !new_expr || p.at(T!['(']) {
			// it's safe to unwrap to because we check beforehand the existence of '('
			// which is mandatory for `parse_arguments`
			parse_arguments(p).unwrap();
			let complete = m.complete(p, JS_NEW_EXPRESSION);
			return Present(subscripts(p, complete, true));
		}
		return Present(m.complete(p, JS_NEW_EXPRESSION));
	}

	// super.foo and super[bar]
	// test super_property_access
	// super.foo
	// super[bar]
	// super[foo][bar]
	if p.at(T![super]) && token_set!(T![.], T!['['], T![?.]).contains(p.nth(1)) {
		let super_completed = parse_super_expression(p);

		if let Present(mut super_marker) = super_completed {
			let lhs = match p.cur() {
				T![.] => parse_static_member_expression(p, super_marker, T![.]),
				T!['['] => parse_computed_member_expression(p, super_marker, false),
				T![?.] => {
					super_marker.change_kind(p, JS_UNKNOWN_EXPRESSION);
					p.error(
						p.err_builder(
							"Super doesn't support optional chaining as super can never be null",
						)
						.primary(super_marker.range(p), ""),
					);
					parse_static_member_expression(p, super_marker, T![?.])
				}
				_ => unreachable!(),
			};

			return lhs.map(|lhs| subscripts(p, lhs, true));
		}
	}

	parse_primary_expression(p).map(|lhs| subscripts(p, lhs, true))
}

// test super_expression
// class Test extends B {
// 	constructor() {
// 		super();
// 	}
// 	test() {
// 		super.test(a, b);
// 		super[1];
// 	}
// }
//
// test_err super_expression_err
// class Test extends B {
// 	test() {
// 		super();
// 		super?.test();
// 	}
// }
// super();
fn parse_super_expression(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![super]) {
		return Absent;
	}
	let super_marker = p.start();
	p.expect(T![super]);
	Present(super_marker.complete(p, JS_SUPER_EXPRESSION))
}

/// Dot, Array, or Call expr subscripts. Including optional chaining.
// test subscripts
// foo`bar`
// foo(bar)(baz)(baz)[bar]
fn subscripts(p: &mut Parser, mut lhs: CompletedMarker, no_call: bool) -> CompletedMarker {
	// test_err subscripts_err
	// foo()?.baz[].
	// BAR`b
	let mut should_try_parsing_ts = true;
	let mut progress = ParserProgress::default();
	while !p.at(EOF) {
		progress.assert_progressing(p);

		match p.cur() {
			T![?.] if p.nth_at(1, T!['(']) => {
				lhs = {
					let m = lhs.precede(p);
					p.bump_any();
					// it's safe to unwrap to because we check beforehand the existence of '('
					// which is mandatory for `parse_arguments`
					parse_arguments(p).unwrap();
					m.complete(p, JS_CALL_EXPRESSION)
				}
			}
			T!['('] if !no_call => {
				lhs = {
					let m = lhs.precede(p);
					// it's safe to unwrap to because we check beforehand the existence of '('
					// which is mandatory for `parse_arguments`
					parse_arguments(p).unwrap();
					m.complete(p, JS_CALL_EXPRESSION)
				}
			}
			T![?.] if p.nth_at(1, T!['[']) => {
				lhs = parse_computed_member_expression(p, lhs, true).unwrap()
			}
			T!['['] => lhs = parse_computed_member_expression(p, lhs, false).unwrap(),
			T![?.] => lhs = parse_static_member_expression(p, lhs, T![?.]).unwrap(),
			T![.] => lhs = parse_static_member_expression(p, lhs, T![.]).unwrap(),
			T![!] if !p.has_linebreak_before_n(0) => {
				lhs = {
					// FIXME(RDambrosio016): we need to tell the lexer that an expression is not
					// allowed here, but we have no way of doing that currently because we get all of the
					// tokens ahead of time, therefore we need to switch to using the lexer as an iterator
					// which isn't as simple as it sounds :)
					let m = lhs.precede(p);
					p.bump_any();
					let mut comp = m.complete(p, TS_NON_NULL);
					comp.err_if_not_ts(
						p,
						"non-null assertions can only be used in TypeScript files",
					);
					comp
				}
			}
			T![<] if p.typescript() && should_try_parsing_ts => {
				let res = try_parse_ts(p, |p| {
					let m = lhs.precede(p);
					// TODO: handle generic async arrow function expressions
					if ts_type_args(p).is_none() {
						m.abandon(p);
						return None;
					}

					if !no_call && p.at(T!['(']) {
						// we already to the check on '(', so it's safe to unwrap
						parse_arguments(p).unwrap();
						Some(m.complete(p, JS_CALL_EXPRESSION))
					} else if p.at(BACKTICK) {
						m.abandon(p);
						Some(parse_template_literal(p, Present(lhs)))
					} else {
						None
					}
				});
				if res.is_none() {
					should_try_parsing_ts = false;
				}
			}
			BACKTICK => lhs = parse_template_literal(p, Present(lhs)),
			_ => return lhs,
		}
	}
	lhs
}

/// A static member expression for accessing a property
// test static_member_expression
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
// class Test {
// 	#bar
// 	test(other) {
// 		this.#bar;
// 		this?.#bar;
// 		other.#bar;
// 		other?.#bar;
// 	}
// }
fn parse_static_member_expression(
	p: &mut Parser,
	lhs: CompletedMarker,
	operator: JsSyntaxKind,
) -> ParsedSyntax {
	let m = lhs.precede(p);
	p.expect(operator);

	parse_any_name(p).or_add_diagnostic(p, expected_identifier);

	Present(m.complete(p, JS_STATIC_MEMBER_EXPRESSION))
}

fn parse_private_name(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![#]) {
		return Absent;
	}

	let m = p.start();
	p.expect(T![#]);
	p.expect(T![ident]);
	Present(m.complete(p, JS_PRIVATE_NAME))
}

pub(super) fn parse_any_name(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T![#] => parse_private_name(p),
		_ => parse_name(p),
	}
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test computed_member_expression
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
pub fn parse_computed_member_expression(
	p: &mut Parser,
	lhs: CompletedMarker,
	optional_chain: bool,
) -> ParsedSyntax {
	// test_err bracket_expr_err
	// foo[]
	// foo?.[]
	// foo[
	let m = lhs.precede(p);
	if optional_chain {
		p.expect(T![?.]);
	}

	p.expect(T!['[']);
	parse_expression(p).or_add_diagnostic(p, expected_expression);
	p.expect(T![']']);

	Present(m.complete(p, JS_COMPUTED_MEMBER_EXPRESSION))
}

/// An identifier name, either an ident or a keyword
pub(super) fn parse_name(p: &mut Parser) -> ParsedSyntax {
	if is_at_name(p) {
		let m = p.start();
		p.bump_remap(T![ident]);
		Present(m.complete(p, JS_NAME))
	} else {
		Absent
	}
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`

// test_err invalid_arg_list
// foo(a,b;
// foo(a,b var
// foo (,,b)
fn parse_arguments(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['(']) {
		return Absent;
	}
	let m = p.start();
	p.bump(T!['(']);
	let args_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) && !p.at(T![')']) {
		progress.assert_progressing(p);

		if p.at(T![...]) {
			// already do a check on "..." so it's safe to unwrap
			parse_spread_element(p).unwrap();
		} else {
			parse_expr_or_assignment(p)
				.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
		}

		if p.at(T![,]) {
			p.bump_any();
		} else {
			break;
		}
	}

	args_list.complete(p, JS_CALL_ARGUMENT_LIST);
	p.expect(T![')']);
	Present(m.complete(p, JS_CALL_ARGUMENTS))
}

// test paren_or_arrow_expr
// (foo);
// (foo) => {};
// (5 + 5);
// ({foo, bar, b: [f, ...baz]}) => {};
// (foo, ...bar) => {}

// test_err paren_or_arrow_expr_invalid_params
// (5 + 5) => {}
// (a, ,b) => {}
// (a, b) =>
fn parse_paren_or_arrow_expr(p: &mut Parser, can_be_arrow: bool) -> ParsedSyntax {
	let m = p.start();
	let checkpoint = p.checkpoint();
	let start = p.cur_tok().start();

	p.expect(T!['(']);
	let mut spread_range = None;
	let mut trailing_comma_marker = None;
	let mut params_marker = None;

	let is_empty = p.eat(T![')']);

	if !is_empty {
		p.with_state(PotentialArrowStart(true), |p| {
			// stores a potentially started sequence expression
			let mut sequence: Option<Marker> = None;

			loop {
				if p.at(T![...]) {
					let m = p.start();
					p.bump_any();
					parse_binding_pattern(p).or_add_diagnostic(p, expected_binding);
					if p.eat(T![:]) {
						if let Some(mut ty) = ts_type(p) {
							ty.err_if_not_ts(
							p,
							"spread elements can only have type annotations in TypeScript files",
						);
						}
					}
					let complete = m.complete(p, JS_REST_PARAMETER);
					spread_range = Some(complete.range(p));
					if !p.eat(T![')']) {
						if p.eat(T![=]) {
							parse_expr_or_assignment(p).or_add_diagnostic(p, expected_expression);
							p.expect(T![')']);
						} else {
							let err = p.err_builder(&format!("expect a closing parenthesis after a spread element, but instead found `{}`", p.cur_src()))
							.primary(p.cur_tok().range(), "");

							#[allow(deprecated)]
							SingleTokenParseRecovery::with_error(
								EXPR_RECOVERY_SET,
								JS_UNKNOWN,
								err,
							)
							.recover(p);
						}
					}
					break;
				}
				let expr = parse_expr_or_assignment(p);
				if expr.is_absent() && p.at(T![:]) {
					p.rewind(checkpoint);
					params_marker = Some(parse_parameter_list(p).unwrap());
					break;
				}

				if p.at(T![,]) {
					if p.at(T![')']) {
						// case where we are at a `,)` so the `,` is a trailing comma
						let trailing_marker = p.start();
						p.bump_any(); // bump ,
						trailing_comma_marker = Some(trailing_marker.complete(p, JS_UNKNOWN));
						p.bump_any(); // bump )
						break;
					} else {
						// start a sequence expression that precedes the before parsed expression statement
						// and bump the ',' into it.
						sequence = sequence
							.or_else(|| {
								Some(expr.precede_or_add_diagnostic(
									p,
									js_parse_error::expected_expression,
								))
							})
							.or_else(|| Some(p.start()));
						p.bump_any(); // bump ; into sequence expression which may or may not miss a lhs
					}
				} else {
					if let Some(sequence) = sequence.take() {
						sequence.complete(p, JS_SEQUENCE_EXPRESSION);
					}
					p.expect(T![')']);
					break;
				}
			}

			if let Some(sequence) = sequence.take() {
				sequence.complete(p, JS_SEQUENCE_EXPRESSION);
			}
		});
	}

	let has_ret_type = !p.state.in_condition_expression() && p.at(T![:]);

	// This is an arrow expr, so we rewind the parser and reparse as parameters
	// This is kind of inefficient but in the grand scheme of things it does not matter
	// FIXME: verify that this logic is correct
	if (p.at(T![=>]) && !p.has_linebreak_before_n(0)) || has_ret_type || params_marker.is_some() {
		if !can_be_arrow && !p.at(T![:]) {
			let err = p.err_builder("Unexpected token `=>`").primary(
				p.cur_tok().range(),
				"an arrow expression is not allowed here",
			);

			p.error(err);
		} else {
			if params_marker.is_none() {
				// Rewind the parser so we can reparse as formal parameters
				p.rewind(checkpoint);
				parse_parameter_list(p).or_add_diagnostic(p, js_parse_error::expected_parameters);
			}

			if p.at(T![:]) {
				let complete = ts_type_or_type_predicate_ann(p, T![:]);
				if let Some(mut complete) = complete {
					complete.err_if_not_ts(
						p,
						"arrow functions can only have return types in TypeScript files",
					);
				}
			}

			p.bump_any();
			parse_arrow_body(p).or_add_diagnostic(p, js_parse_error::expected_arrow_body);
			return Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION));
		}
	}

	if let Some(params) = params_marker {
		let err = p
			.err_builder("grouping expressions cannot contain parameters")
			.primary(params.range(p), "");

		p.error(err);
		return Present(m.complete(p, JS_UNKNOWN_EXPRESSION));
	}

	if is_empty {
		let err = p
			.err_builder("grouping expressions cannot be empty")
			.primary(start..p.cur_tok().start(), "");

		p.error(err);
		return Present(m.complete(p, JS_PARENTHESIZED_EXPRESSION));
	}

	if let Some(range) = spread_range {
		let err = p
			.err_builder("Illegal spread element inside grouping expression")
			.primary(range, "");

		p.error(err);
	}

	if let Some(complete) = trailing_comma_marker {
		let err = p
			.err_builder("Illegal trailing comma in grouping expression")
			.primary(complete.range(p), "");

		p.error(err);
	}

	// test js_parenthesized_expression
	// ((foo))
	// (foo)
	Present(m.complete(p, JS_PARENTHESIZED_EXPRESSION))
}

pub fn parse_expression_snipped(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	parse_expression(p).or_add_diagnostic(p, expected_expression);
	m.complete(p, JS_EXPRESSION_SNIPPED).into()
}

/// A general expression.
// test sequence_expr
// 1, 2, 3, 4, 5

// test_err sequence_expr
// 1, 2, , 4
pub(crate) fn parse_expression(p: &mut Parser) -> ParsedSyntax {
	let first = parse_expr_or_assignment(p);

	first.map(|first_marker| {
		if p.at(T![,]) {
			let sequence_expr_marker = first_marker.precede(p);

			p.bump_any();
			parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);

			sequence_expr_marker.complete(p, JS_SEQUENCE_EXPRESSION)
		} else {
			first_marker
		}
	})
}

#[inline]
pub(crate) fn is_at_expression(p: &Parser) -> bool {
	is_nth_at_expression(p, 0)
}

pub(crate) fn is_nth_at_expression(p: &Parser, n: usize) -> bool {
	STARTS_EXPR.contains(p.nth(n))
		|| p.nth_at(n, T![<])
		|| (p.nth_at(n, T![enum]) && !p.has_linebreak_before_n(n))
}

/// A primary expression such as a literal, an object, an array, or `this`.
fn parse_primary_expression(p: &mut Parser) -> ParsedSyntax {
	let parsed_literal_expression = parse_literal_expression(p);
	if parsed_literal_expression.is_present() {
		return parsed_literal_expression;
	}

	let complete = match p.cur() {
		T![this] => {
			// test this_expr
			// this
			// this.foo
			let m = p.start();
			p.bump_any();
			m.complete(p, JS_THIS_EXPRESSION)
		}
		T![class] => {
			// test class_expr
			// let a = class {};
			// let a = class foo {
			//  constructor() {}
			// }
			// foo[class {}]
			parse_class_expression(p).unwrap()
		}
		// test async_ident
		// let a = async;
		T![ident] if p.cur_src() == "async" => {
			// test async_function_expr
			// let a = async function() {};
			// let b = async function foo() {};
			if p.nth_at(1, T![function]) {
				parse_function_expression(p).unwrap()
			} else {
				// `async a => {}` and `async (a) => {}`
				if p.state.potential_arrow_start() && (is_nth_at_name(p, 1) || p.nth(1) == T!['('])
				{
					// test async_arrow_expr
					// let a = async foo => {}
					// let b = async (bar) => {}
					// async (foo, bar, ...baz) => foo
					let m = p.start();
					p.bump_remap(T![async]);

					p.with_state(InAsync(true), |p| {
						let parsed_parameters = parse_parameter_list(p);
						if parsed_parameters.is_absent() {
							// test_err async_arrow_expr_await_parameter
							// let a = async await => {}
							parse_binding(p).or_add_diagnostic(p, expected_parameter);
						}

						if p.at(T![:]) {
							let complete = ts_type_or_type_predicate_ann(p, T![:]);
							if let Some(mut complete) = complete {
								complete.err_if_not_ts(
								p,
								"arrow functions can only have return types in TypeScript files",
							);
							}
						}

						p.expect(T![=>]);

						parse_arrow_body(p)
							.or_add_diagnostic(p, js_parse_error::expected_arrow_body);
					});

					m.complete(p, JS_ARROW_FUNCTION_EXPRESSION)
				} else {
					parse_identifier_expression(p).unwrap()
				}
			}
		}
		T![function] => {
			// test function_expr
			// let a = function() {}
			// let b = function foo() {}

			parse_function_expression(p).unwrap()
		}
		T![ident] | T![yield] | T![await] | T![enum] => {
			// test identifier_reference
			// // SCRIPT
			// foo;
			// yield;
			// await;
			if p.state.potential_arrow_start()
				&& p.nth_at(1, T![=>])
				&& !p.has_linebreak_before_n(1)
			{
				// test arrow_expr_single_param
				// // SCRIPT
				// foo => {}
				// yield => {}
				// await => {}
				// foo =>
				// {}
				let m = p.start();
				parse_identifier_binding(p).or_add_diagnostic(p, expected_identifier);
				p.bump(T![=>]);
				parse_arrow_body(p).or_add_diagnostic(p, js_parse_error::expected_arrow_body);
				m.complete(p, JS_ARROW_FUNCTION_EXPRESSION)
			} else {
				parse_identifier_expression(p).unwrap()
			}
		}
		// test grouping_expr
		// ((foo))
		// (foo)
		T!['('] => parse_paren_or_arrow_expr(p, p.state.potential_arrow_start()).unwrap(),
		T!['['] => parse_array_expr(p).unwrap(),
		T!['{'] if p.state.allow_object_expression() => parse_object_expression(p).unwrap(),
		T![import] => {
			let m = p.start();
			p.bump_any();

			// test import_meta
			// import.meta
			if p.eat(T![.]) {
				// test_err import_no_meta
				// import.foo
				// import.metaa
				if p.at(T![ident]) && p.token_src(p.cur_tok()) == "meta" {
					p.bump_remap(T![meta]);
					m.complete(p, IMPORT_META)
				} else if p.at(T![ident]) {
					let err = p
						.err_builder(&format!(
							"Expected `meta` following an import keyword, but found `{}`",
							p.token_src(p.cur_tok())
						))
						.primary(p.cur_tok().range(), "");

					p.err_and_bump(err, JS_UNKNOWN);
					m.complete(p, IMPORT_META)
				} else {
					let err = p
						.err_builder("Expected `meta` following an import keyword, but found none")
						.primary(p.cur_tok().range(), "");

					p.error(err);
					m.complete(p, JS_UNKNOWN)
				}
			} else {
				// test_err import_call_no_arg
				// let a = import();
				// foo();

				// test import_call
				// import("foo")

				// test_err import_call
				// import()
				p.expect(T!['(']);
				parse_expr_or_assignment(p)
					.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
				p.expect(T![')']);
				m.complete(p, JS_IMPORT_CALL_EXPRESSION)
			}
		}
		BACKTICK => parse_template_literal(p, Absent),
		ERROR_TOKEN => {
			let m = p.start();
			p.bump_any();
			m.complete(p, JS_UNKNOWN)
		}
		// test_err primary_expr_invalid_recovery
		// let a = \; foo();
		_ => {
			return Absent;
		}
	};

	Present(complete)
}

fn parse_identifier_expression(p: &mut Parser) -> ParsedSyntax {
	parse_reference_identifier(p)
		.map(|identifier| identifier.precede(p).complete(p, JS_IDENTIFIER_EXPRESSION))
}

// test_err identifier
// yield;
// await;
pub(crate) fn parse_reference_identifier(p: &mut Parser) -> ParsedSyntax {
	parse_identifier(p, JS_REFERENCE_IDENTIFIER)
}

pub(crate) fn is_nth_at_reference_identifier(p: &Parser, n: usize) -> bool {
	is_nth_at_identifier(p, n)
}

// test identifier_loose_mode
// // SCRIPT
// foo;
// yield;
// await;
//
// test identifier
// foo;
//
// test_err identifier_err
// yield;
// await;
// async function test(await) {}
// function* test(yield) {}
// enum;
// implements;
// interface;

/// Parses an identifier if it is valid in this context or returns `Invalid` if the context isn't valid in this context.
/// An identifier is invalid if:
/// * It is named `await` inside of an async function
/// * It is named `yield` inside of a generator function or in strict mode
pub(super) fn parse_identifier(p: &mut Parser, kind: JsSyntaxKind) -> ParsedSyntax {
	match p.cur() {
		T![yield] | T![await] | T![ident] | T![enum] => {
			let m = p.start();
			let name = p.cur_src();

			let error = match name {
				"await" if p.state.in_async() => Some(
					p.err_builder("Illegal use of `await` as an identifier in an async context")
						.primary(p.cur_tok().range(), ""),
				),
				"await" if p.syntax.file_kind == FileKind::Module => Some(
					p.err_builder("Illegal use of `await` as an identifier inside of a module")
						.primary(p.cur_tok().range(), ""),
				),
				"yield" if p.state.in_generator() => Some(
					p.err_builder("Illegal use of `yield` as an identifier in generator function")
						.primary(p.cur_tok().range(), ""),
				),

				"yield" | "let" | "public" | "protected" | "private" | "package" | "implements"
				| "interface" | "static"
					if StrictMode.is_supported(p) =>
				{
					Some(
						p.err_builder(&format!(
							"Illegal use of reserved keyword `{}` as an identifier in strict mode",
							name
						))
						.primary(p.cur_tok().range(), ""),
					)
				}
				_ if p.cur() == T![enum] => Some(
					p.err_builder("Illegal use of reserved keyword `enum` as an identifier")
						.primary(p.cur_tok().range(), ""),
				),
				_ => None,
			};

			p.bump_remap(T![ident]);
			let mut identifier = m.complete(p, kind);

			if let Some(error) = error {
				p.error(error);
				identifier.change_kind(p, kind.to_unknown());
			}

			Present(identifier)
		}
		_ => Absent,
	}
}

#[inline]
pub(crate) fn is_at_identifier(p: &Parser) -> bool {
	is_nth_at_identifier(p, 0)
}

pub(crate) fn is_nth_at_identifier(p: &Parser, n: usize) -> bool {
	matches!(p.nth(n), T![ident] | T![await] | T![yield] | T![enum])
}

/// A template literal such as "`abcd ${efg}`"
// test template_literal
// let a = `foo ${bar}`;
// let a = ``;
// let a = `${foo}`;
// let a = `foo`;

// test_err template_literal
// let a = `foo ${}`
pub fn parse_template_literal(p: &mut Parser, tag: ParsedSyntax) -> CompletedMarker {
	let m = tag.precede(p);

	p.expect(BACKTICK);
	let elements_list = p.start();

	while !p.at(EOF) && !p.at(BACKTICK) {
		match p.cur() {
            TEMPLATE_CHUNK => {
							let m = p.start();
							p.bump_any();
							m.complete(p, TEMPLATE_CHUNK_ELEMENT);
						},
            DOLLAR_CURLY => {
                let e = p.start();
                p.bump_any();
								parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
								p.expect(T!['}']);
                e.complete(p, TEMPLATE_ELEMENT);
            }
            t => unreachable!("Anything not template chunk or dollarcurly should have been eaten by the lexer, but {:?} was found", t),
        }
	}

	elements_list.complete(p, TEMPLATE_ELEMENT_LIST);

	// test_err template_literal_unterminated
	// let a = `${foo} bar

	// The lexer already should throw an error for unterminated template literal
	p.eat(BACKTICK);
	m.complete(p, TEMPLATE)
}

struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
		match p.cur() {
			T![...] => parse_spread_element(p),
			T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
			_ => parse_expr_or_assignment(p),
		}
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at(T![']'])
	}

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_EXPRESSION, EXPR_RECOVERY_SET),
			js_parse_error::expected_array_element,
		)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_ARRAY_ELEMENT_LIST
	}

	fn separating_element_kind(&mut self) -> JsSyntaxKind {
		T![,]
	}

	fn allow_trailing_separating_element(&self) -> bool {
		true
	}
}

/// An array literal such as `[foo, bar, ...baz]`.
// test array_expr
// [foo, bar];
// [foo];
// [,foo];
// [foo,];
// [,,,,,foo,,,,];
// [...a, ...b];
fn parse_array_expr(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['[']) {
		return Absent;
	}
	let m = p.start();
	p.bump(T!['[']);
	ArrayElementsList.parse_list(p);
	p.expect(T![']']);
	Present(m.complete(p, JS_ARRAY_EXPRESSION))
}

// test_err spread
// [...]
/// A spread element consisting of three dots and an assignment expression such as `...foo`
fn parse_spread_element(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![...]) {
		return Absent;
	}
	let m = p.start();
	p.bump(T![...]);
	parse_expr_or_assignment(p)
		.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
	Present(m.complete(p, JS_SPREAD))
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub(super) fn parse_lhs_expr(p: &mut Parser) -> ParsedSyntax {
	let lhs = if p.at(T![super]) && p.nth_at(1, T!['(']) {
		let super_syntax = parse_super_expression(p);
		if let Present(mut super_marker) = super_syntax {
			if !p.state.in_constructor() {
				p.error(
					p.err_builder(
						"`super` is only valid inside of a class constructor of a subclass.",
					)
					.primary(super_marker.range(p), ""),
				);
				super_marker.change_kind(p, JS_UNKNOWN_EXPRESSION);
			}
		}

		super_syntax
	} else {
		parse_member_or_new_expr(p, true)
	};

	if lhs.kind() == Some(JS_ARROW_FUNCTION_EXPRESSION) {
		return lhs;
	}

	lhs.map(|lhs_marker| {
		let m = lhs_marker.precede(p);
		let type_args = if p.at(T![<]) {
			let checkpoint = p.checkpoint();
			let mut complete = try_parse_ts(p, ts_type_args);
			if !p.at(T!['(']) {
				p.rewind(checkpoint);
				None
			} else {
				if let Some(ref mut comp) = complete {
					comp.err_if_not_ts(p, "type arguments can only be used in TypeScript files");
				}
				complete
			}
		} else {
			None
		};

		if p.at(T!['(']) || type_args.is_some() {
			// it's safe to unwrap
			parse_arguments(p).unwrap();
			let lhs = m.complete(p, JS_CALL_EXPRESSION);
			return subscripts(p, lhs, false);
		}

		m.abandon(p);
		lhs_marker
	})
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test postfix_expr
// foo++
// foo--
fn parse_postfix_expr(p: &mut Parser) -> ParsedSyntax {
	let checkpoint = p.checkpoint();
	let lhs = parse_lhs_expr(p);
	lhs.map(|marker| {
		if !p.has_linebreak_before_n(0) {
			// test post_update_expr
			// foo++
			// foo--
			match p.cur() {
				T![++] => {
					let assignment_target = expression_to_assignment(p, marker, checkpoint);
					let m = assignment_target.precede(p);
					p.bump(T![++]);
					m.complete(p, JS_POST_UPDATE_EXPRESSION)
				}
				T![--] => {
					let assignment_target = expression_to_assignment(p, marker, checkpoint);
					let m = assignment_target.precede(p);
					p.bump(T![--]);
					m.complete(p, JS_POST_UPDATE_EXPRESSION)
				}
				_ => marker,
			}
		} else {
			marker
		}
	})
}

/// A unary expression such as `!foo` or `++bar`
pub(super) fn parse_unary_expr(p: &mut Parser) -> ParsedSyntax {
	const UNARY_SINGLE: TokenSet =
		token_set![T![delete], T![void], T![typeof], T![+], T![-], T![~], T![!]];

	// FIXME: this shouldn't allow await in sync functions
	if (p.state.in_async() || p.syntax.top_level_await) && p.at(T![await]) {
		// test await_expression
		// async function test() {
		// 	await inner();
		// 	await (inner()) + await inner();
		// }
		// async function inner() {
		// 	return 4;
		// }
		let m = p.start();
		p.bump_any();
		parse_unary_expr(p).or_add_diagnostic(p, js_parse_error::expected_unary_expression);
		return Present(m.complete(p, JS_AWAIT_EXPRESSION));
	}

	if p.at(T![<]) {
		let m = p.start();
		p.bump_any();
		return if p.eat(T![const]) {
			p.expect(T![>]);
			parse_unary_expr(p).or_add_diagnostic(p, js_parse_error::expected_unary_expression);
			let mut res = m.complete(p, TS_CONST_ASSERTION);
			res.err_if_not_ts(p, "const assertions can only be used in TypeScript files");
			Present(res)
		} else {
			ts_type(p);
			p.expect(T![>]);
			parse_unary_expr(p).or_add_diagnostic(p, js_parse_error::expected_unary_expression);
			let mut res = m.complete(p, TS_ASSERTION);
			res.err_if_not_ts(p, "type assertions can only be used in TypeScript files");
			Present(res)
		};
	}

	// test pre_update_expr
	// ++foo
	// --foo
	if p.at(T![++]) {
		let m = p.start();
		p.bump(T![++]);
		parse_assignment(p, AssignmentExprPrecedence::Unary)
			.or_add_diagnostic(p, expected_simple_assignment_target);
		let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
		return Present(complete);
	}
	if p.at(T![--]) {
		let m = p.start();
		p.bump(T![--]);
		parse_assignment(p, AssignmentExprPrecedence::Unary)
			.or_add_diagnostic(p, expected_simple_assignment_target);
		let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
		return Present(complete);
	}

	// test js_unary_expressions
	// delete a['test'];
	// void a;
	// typeof a;
	// +1;
	// -1;
	// ~1;
	// !true;
	// -a + -b + +a;

	// test_err unary_expr
	// ++ ;
	// -- ;
	// -;

	if p.at_ts(UNARY_SINGLE) {
		let m = p.start();
		let op = p.cur();
		p.bump_any();

		let res = parse_unary_expr(p).ok();

		if op == T![delete] && p.typescript() {
			if let Some(res) = res {
				match res.kind() {
					JS_STATIC_MEMBER_EXPRESSION | JS_COMPUTED_MEMBER_EXPRESSION => {}
					_ => {
						let err = p
							.err_builder(
								"the target for a delete operator must be a property access",
							)
							.primary(res.range(p), "");

						p.error(err);
					}
				}
			}
		}
		return Present(m.complete(p, JS_UNARY_EXPRESSION));
	}

	parse_postfix_expr(p)
}

pub(super) fn is_at_name(p: &Parser) -> bool {
	is_nth_at_name(p, 0)
}

pub(super) fn is_nth_at_name(p: &Parser, offset: usize) -> bool {
	p.nth_at(offset, T![ident]) || p.nth(offset).is_keyword()
}
