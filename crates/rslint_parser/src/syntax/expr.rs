//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-11).

use super::binding::parse_binding;
use super::decl::{parse_arrow_body, parse_parameter_list};
use super::typescript::*;
use super::util::*;
#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParserProgress;
use crate::syntax::assignment_target::{
	expression_to_assignment_target, expression_to_simple_assignment_target,
	parse_simple_assignment_target, SimpleAssignmentTargetExprKind,
};
use crate::syntax::binding::parse_identifier_binding;
use crate::syntax::class::class_expression;
use crate::syntax::function::parse_function_expression;
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
	expected_binding, expected_identifier, expected_parameter, expected_simple_assignment_target,
};
use crate::syntax::object::parse_object_expression;
use crate::syntax::stmt::is_semi;
use crate::ConditionalParsedSyntax::{Invalid, Valid};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{SyntaxKind::*, *};

pub const EXPR_RECOVERY_SET: TokenSet = token_set![VAR_KW, R_PAREN, L_PAREN, L_BRACK, R_BRACK];

pub const ASSIGN_TOKENS: TokenSet = token_set![
	T![=],
	T![+=],
	T![-=],
	T![*=],
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
	T![/=],
	T![>>=]
];

pub const STARTS_EXPR: TokenSet = token_set![
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
pub fn parse_literal_expression(p: &mut Parser) -> ParsedSyntax {
	let literal_kind = match p.cur_tok().kind {
		SyntaxKind::JS_NUMBER_LITERAL => {
			if p.cur_src().ends_with('n') {
				let m = p.start();
				p.bump_remap(SyntaxKind::JS_BIG_INT_LITERAL);
				return Present(m.complete(p, JS_BIG_INT_LITERAL_EXPRESSION));
			};

			SyntaxKind::JS_NUMBER_LITERAL_EXPRESSION
		}
		SyntaxKind::JS_STRING_LITERAL => SyntaxKind::JS_STRING_LITERAL_EXPRESSION,
		SyntaxKind::NULL_KW => SyntaxKind::JS_NULL_LITERAL_EXPRESSION,
		SyntaxKind::TRUE_KW | SyntaxKind::FALSE_KW => SyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION,
		SyntaxKind::JS_REGEX_LITERAL => SyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
		_ => return Absent,
	};

	let m = p.start();
	p.bump_any();
	Present(m.complete(p, literal_kind))
}

/// Parses an expression that might turn out to be an assignment target if an assignment operator is found
pub(crate) fn expr_or_assignment(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![<])
		&& (token_set![T![ident], T![await], T![yield]].contains(p.nth(1)) || p.nth(1).is_keyword())
	{
		let res = try_parse_ts(p, |p| {
			let m = p.start();
			ts_type_params(p)?;
			let res = assign_expr_base(p);
			if res.map(|x| x.kind()) != Some(JS_ARROW_FUNCTION_EXPRESSION) {
				m.abandon(p);
				None
			} else {
				res.unwrap().undo_completion(p).abandon(p);
				Some(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
			}
		});
		if let Some(mut res) = res {
			res.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
			return Some(res);
		}
	}
	assign_expr_base(p)
}

fn assign_expr_base(p: &mut Parser) -> Option<CompletedMarker> {
	if p.state.in_generator && p.at(T![yield]) {
		return Some(yield_expr(p));
	}
	let potential_arrow_start = matches!(p.cur(), T![ident] | T!['('] | T![yield] | T![await]);
	let mut guard = p.with_state(ParserState {
		potential_arrow_start,
		..p.state.clone()
	});

	let checkpoint = guard.checkpoint();
	let target = conditional_expr(&mut *guard)?;
	assign_expr_recursive(&mut *guard, target, checkpoint)
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
fn assign_expr_recursive(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> Option<CompletedMarker> {
	if p.at_ts(ASSIGN_TOKENS) {
		let target = expression_to_assignment_target(
			p,
			target,
			checkpoint,
			SimpleAssignmentTargetExprKind::Conditional,
		);
		let m = target.precede(p);
		p.bump_any(); // operator
		expr_or_assignment(p);
		Some(m.complete(p, JS_ASSIGNMENT_EXPRESSION))
	} else {
		Some(target)
	}
}

// test yield_expr
// function *foo() {
//  yield foo;
//  yield* foo;
//  yield;
// }
pub fn yield_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T![yield]);

	if !is_semi(p, 0) && (p.at(T![*]) || p.at_ts(STARTS_EXPR)) {
		p.eat(T![*]);
		expr_or_assignment(p);
	}

	m.complete(p, JS_YIELD_EXPRESSION)
}

/// A conditional expression such as `foo ? bar : baz`
// test conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz
pub fn conditional_expr(p: &mut Parser) -> Option<CompletedMarker> {
	// test_err conditional_expr_err
	// foo ? bar baz
	// foo ? bar baz ? foo : bar
	let lhs = binary_or_logical_expression(p);

	if p.at(T![?]) {
		let m = lhs?.precede(p);
		p.bump_any();
		expr_or_assignment(&mut *p.with_state(ParserState {
			in_cond_expr: true,
			..p.state.clone()
		}));
		p.expect_required(T![:]);
		expr_or_assignment(p);
		return Some(m.complete(p, JS_CONDITIONAL_EXPRESSION));
	}
	lhs
}

/// A binary expression such as `2 + 2` or `foo * bar + 2` or a logical expression 'a || b'
pub fn binary_or_logical_expression(p: &mut Parser) -> Option<CompletedMarker> {
	let left = unary_expr(p);
	binary_or_logical_expression_recursive(p, left, 0)
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
fn binary_or_logical_expression_recursive(
	p: &mut Parser,
	left: Option<CompletedMarker>,
	min_prec: u8,
) -> Option<CompletedMarker> {
	if 7 > min_prec && !p.has_linebreak_before_n(0) && p.cur_src() == "as" {
		let m = left.map(|x| x.precede(p)).unwrap_or_else(|| p.start());
		p.bump_any();
		let mut res = if p.eat(T![const]) {
			m.complete(p, TS_CONST_ASSERTION)
		} else {
			ts_type(p);
			m.complete(p, TS_ASSERTION)
		};
		res.err_if_not_ts(p, "type assertions can only be used in TypeScript files");
		return binary_or_logical_expression_recursive(p, Some(res), min_prec);
	}
	let kind = match p.cur() {
		T![>] if p.nth_at(1, T![>]) && p.nth_at(2, T![>]) => T![>>>],
		T![>] if p.nth_at(1, T![>]) => T![>>],
		k => k,
	};

	let precedence = match kind {
		T![in] if p.state.include_in => 7,
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

	let m = left.map(|m| m.precede(p)).unwrap_or_else(|| p.start());
	if op == T![>>] {
		p.bump_multiple(2, T![>>]);
	} else if op == T![>>>] {
		p.bump_multiple(3, T![>>>]);
	} else {
		p.bump_any();
	}

	// This is a hack to allow us to effectively recover from `foo + / bar`
	let right = if get_precedence(p.cur()).is_some() && !p.at_ts(token_set![T![-], T![+], T![<]]) {
		let err = p.err_builder(&format!("Expected an expression for the right hand side of a `{}`, but found an operator instead", p.token_src(&op_tok)))
            .secondary(op_tok.range, "This operator requires a right hand side value")
            .primary(p.cur_tok().range, "But this operator was encountered instead");

		p.error(err);
		None
	} else {
		unary_expr(p)
	};

	binary_or_logical_expression_recursive(
		p,
		right,
		// ** is right recursive
		if op == T![**] {
			precedence - 1
		} else {
			precedence
		},
	);

	let expression_kind = match op {
		T![??] | T![||] | T![&&] => JS_LOGICAL_EXPRESSION,
		_ => JS_BINARY_EXPRESSION,
	};

	let complete = m.complete(p, expression_kind);
	binary_or_logical_expression_recursive(p, Some(complete), min_prec)

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
pub fn member_or_new_expr(p: &mut Parser, new_expr: bool) -> Option<CompletedMarker> {
	if p.at(T![new]) {
		// We must start the marker here and not outside or else we make
		// a needless node if the node ends up just being a primary expr
		let m = p.start();
		p.bump_any();

		// new.target
		if p.at(T![.]) && p.token_src(&p.nth_tok(1)) == "target" {
			p.bump_any();
			p.bump_any();
			let complete = m.complete(p, NEW_TARGET);
			return Some(subscripts(p, complete, true));
		}

		let complete = if let Some(expr) = member_or_new_expr(p, new_expr) {
			expr
		} else {
			m.abandon(p);
			return None;
		};
		if complete.kind() == JS_ARROW_FUNCTION_EXPRESSION {
			m.abandon(p);
			return Some(complete);
		}

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
			args(p);
			let complete = m.complete(p, NEW_EXPR);
			return Some(subscripts(p, complete, true));
		}
		return Some(m.complete(p, NEW_EXPR));
	}

	// super.foo and super[bar]
	// test super_property_access
	// super.foo
	// super[bar]
	// super[foo][bar]
	if p.at(T![super]) && token_set!(T![.], T!['['], T![?.]).contains(p.nth(1)) {
		let mut super_completed = super_expression(p);

		let lhs = match p.cur() {
			T![.] => static_member_expression(p, super_completed, T![.]),
			T!['['] => computed_member_expression(p, super_completed, false),
			T![?.] => {
				super_completed.change_kind(p, JS_UNKNOWN_EXPRESSION);
				p.error(
					p.err_builder(
						"Super doesn't support optional chaining as super can never be null",
					)
					.primary(super_completed.range(p), ""),
				);
				static_member_expression(p, super_completed, T![?.])
			}
			_ => unreachable!(),
		};

		return Some(subscripts(p, lhs, true));
	}

	let lhs = primary_expr(p)?;
	Some(subscripts(p, lhs, true))
}

fn super_expression(p: &mut Parser) -> CompletedMarker {
	let super_marker = p.start();
	p.expect_required(T![super]);
	super_marker.complete(p, JS_SUPER_EXPRESSION)
}

/// Dot, Array, or Call expr subscripts. Including optional chaining.
// test subscripts
// foo`bar`
// foo(bar)(baz)(baz)[bar]
pub fn subscripts(p: &mut Parser, mut lhs: CompletedMarker, no_call: bool) -> CompletedMarker {
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
					args(p);
					m.complete(p, CALL_EXPR)
				}
			}
			T!['('] if !no_call => {
				lhs = {
					let m = lhs.precede(p);
					args(p);
					m.complete(p, CALL_EXPR)
				}
			}
			T![?.] if p.nth_at(1, T!['[']) => lhs = computed_member_expression(p, lhs, true),
			T!['['] => lhs = computed_member_expression(p, lhs, false),
			T![?.] => lhs = static_member_expression(p, lhs, T![?.]),
			T![.] => lhs = static_member_expression(p, lhs, T![.]),
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
					ts_type_args(p)?;
					if !no_call && p.at(T!['(']) {
						args(p);
						Some(m.complete(p, CALL_EXPR))
					} else if p.at(BACKTICK) {
						m.abandon(p);
						Some(template(p, Some(lhs)))
					} else {
						None
					}
				});
				if res.is_none() {
					should_try_parsing_ts = false;
				}
			}
			BACKTICK => lhs = template(p, Some(lhs)),
			_ => return lhs,
		}
	}
	lhs
}

/// A static member expression for accessing a property
// test dot_expr
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
pub fn static_member_expression(
	p: &mut Parser,
	lhs: CompletedMarker,
	operator: SyntaxKind,
) -> CompletedMarker {
	let m = lhs.precede(p);
	p.expect_required(operator);

	let member_name = any_reference_member(p);

	if !p.syntax.class_fields {
		if let Some(priv_range) = member_name.filter(|x| x.kind() == JS_REFERENCE_PRIVATE_MEMBER) {
			let err = p
				.err_builder("private identifiers are unsupported")
				.primary(priv_range.range(p), "");

			p.error(err);
			return m.complete(p, ERROR);
		}
	}

	m.complete(p, JS_STATIC_MEMBER_EXPRESSION)
}

pub(super) fn any_reference_member(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![#]) {
		Some(reference_private_member(p))
	} else {
		parse_reference_identifier_member(p).ok()
	}
}

pub(super) fn is_at_reference_identifier_member(p: &Parser) -> bool {
	p.at(T![ident]) || p.cur().is_keyword()
}

pub(super) fn parse_reference_identifier_member(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T![ident] => {
			let m = p.start();
			p.bump_any();
			Present(m.complete(p, JS_REFERENCE_IDENTIFIER_MEMBER))
		}
		t if t.is_keyword() => {
			let m = p.start();
			p.bump_remap(T![ident]);
			Present(m.complete(p, JS_REFERENCE_IDENTIFIER_MEMBER))
		}
		_ => Absent,
	}
}

fn reference_private_member(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T![#]);
	p.expect_required(T![ident]);
	m.complete(p, JS_REFERENCE_PRIVATE_MEMBER)
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test bracket_expr
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
pub fn computed_member_expression(
	p: &mut Parser,
	lhs: CompletedMarker,
	optional_chain: bool,
) -> CompletedMarker {
	// test_err bracket_expr_err
	// foo[]
	// foo?.[]
	// foo[
	let m = lhs.precede(p);
	if optional_chain {
		p.expect_required(T![?.]);
	}

	p.expect_required(T!['[']);
	expr(p);
	p.expect_required(T![']']);

	m.complete(p, JS_COMPUTED_MEMBER_EXPRESSION)
}

/// An identifier name, either an ident or a keyword
pub fn identifier_name(p: &mut Parser) -> Option<CompletedMarker> {
	if is_at_identifier_name(p) {
		let m = p.start();
		p.bump_remap(T![ident]);
		Some(m.complete(p, NAME))
	} else {
		let err = p
			.err_builder("Expected an identifier or keyword")
			.primary(p.cur_tok().range, "Expected an identifier or keyword here");
		p.error(err);
		None
	}
}

fn is_at_identifier_name(p: &Parser) -> bool {
	let t = p.cur();

	t.is_keyword() || t == T![ident]
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`

// test_err invalid_arg_LIST
// foo(a,b;
// foo(a,b var
pub fn args(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T!['(']);
	let args_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) && !p.at(T![')']) {
		progress.assert_progressing(p);

		if p.at(T![...]) {
			spread_element(p);
		} else {
			expr_or_assignment(p);
		}

		if p.at(T![,]) {
			p.bump_any();
		} else {
			break;
		}
	}

	args_list.complete(p, LIST);
	p.expect_required(T![')']);
	m.complete(p, ARG_LIST)
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
pub fn paren_or_arrow_expr(p: &mut Parser, can_be_arrow: bool) -> CompletedMarker {
	let m = p.start();
	let checkpoint = p.checkpoint();
	let start = p.cur_tok().range.start;
	p.expect_required(T!['(']);
	let mut spread_range = None;
	let mut trailing_comma_marker = None;
	let mut params_marker = None;

	let mut temp = p.with_state(ParserState {
		potential_arrow_start: true,
		..p.state.clone()
	});

	let is_empty = temp.eat(T![')']);

	if !is_empty {
		// stores a potentially started sequence expression
		let mut sequence: Option<Marker> = None;

		loop {
			if temp.at(T![...]) {
				let m = temp.start();
				temp.bump_any();
				parse_binding(&mut *temp).or_missing_with_error(&mut *temp, expected_binding);
				if temp.eat(T![:]) {
					if let Some(mut ty) = ts_type(&mut *temp) {
						ty.err_if_not_ts(
							&mut *temp,
							"spread elements can only have type annotations in TypeScript files",
						);
					}
				}
				let complete = m.complete(&mut *temp, JS_REST_PARAMETER);
				spread_range = Some(complete.range(&*temp));
				if !temp.eat(T![')']) {
					if temp.eat(T![=]) {
						// formal params will handle this error
						expr_or_assignment(&mut *temp);
						temp.expect_required(T![')']);
					} else {
						let err = temp.err_builder(&format!("expect a closing parenthesis after a spread element, but instead found `{}`", temp.cur_src()))
                    .primary(temp.cur_tok().range, "");

						#[allow(deprecated)]
						SingleTokenParseRecovery::with_error(
							EXPR_RECOVERY_SET,
							JS_UNKNOWN_BINDING,
							err,
						)
						.recover(&mut temp);
					}
				}
				break;
			}
			let expr = expr_or_assignment(&mut *temp);
			if expr.is_some() && temp.at(T![:]) {
				temp.rewind(checkpoint);
				// TODO: review this when `paren_or_arrow_expr` is refactored to use the new API
				params_marker = Some(parse_parameter_list(&mut *temp).ok().unwrap());
				break;
			}

			if temp.at(T![,]) {
				if temp.at(T![')']) {
					// case where we are at a `,)` so the `,` is a trailing comma
					let trailing_marker = temp.start();
					temp.bump_any(); // bump ,
					trailing_comma_marker = Some(trailing_marker.complete(&mut *temp, ERROR));
					temp.bump_any(); // bump )
					break;
				} else {
					// start a sequence expression that precedes the before parsed expression statement
					// and bump the ',' into it.
					sequence = sequence
						.or_else(|| expr.map(|expr| expr.precede(&mut *temp)))
						.or_else(|| Some(temp.start()));
					temp.bump_any(); // bump ; into sequence expression which may or may not miss a lhs
				}
			} else {
				temp.expect_required(T![')']);
				break;
			}
		}

		if let Some(sequence) = sequence.take() {
			sequence.complete(&mut *temp, JS_SEQUENCE_EXPRESSION);
		}
	}

	drop(temp);
	// if we are in a ternary expression, then we need to try and see if parsing as an arrow worked
	// if it did then we just return it, otherwise it should be interpreted as a grouping expr
	if p.state.in_cond_expr && p.at(T![:]) && params_marker.is_none() {
		let func = |p: &mut Parser| {
			let p = &mut *p.with_state(ParserState {
				no_recovery: true,
				..p.state.clone()
			});
			p.rewind(checkpoint);
			parse_parameter_list(p).or_missing_with_error(p, js_parse_error::expected_parameters);
			if p.at(T![:]) {
				if let Some(mut ret) = ts_type_or_type_predicate_ann(p, T![:]) {
					ret.err_if_not_ts(
						p,
						"arrow functions can only have return types in TypeScript files",
					);
				}
			}
			p.expect_no_recover(T![=>])?;
			parse_arrow_body(p).or_missing_with_error(p, js_parse_error::expected_arrow_body)
		};
		// we can't just rewind the parser, since the function rewinds, and cloning and replacing the
		// events does not work apparently, therefore we need to clone the entire parser
		let cloned = p.clone();
		if func(p).is_some() {
			return m.complete(p, JS_ARROW_FUNCTION_EXPRESSION);
		} else {
			*p = cloned;
		}
	}
	let has_ret_type = !p.state.in_cond_expr && p.at(T![:]) && !p.state.in_case_cond;

	// This is an arrow expr, so we rewind the parser and reparse as parameters
	// This is kind of inefficient but in the grand scheme of things it does not matter
	// FIXME: verify that this logic is correct
	if (p.at(T![=>]) && !p.has_linebreak_before_n(0)) || has_ret_type || params_marker.is_some() {
		if !can_be_arrow && !p.at(T![:]) {
			let err = p
				.err_builder("Unexpected token `=>`")
				.primary(p.cur_tok().range, "an arrow expression is not allowed here");

			p.error(err);
		} else {
			if params_marker.is_none() {
				// Rewind the parser so we can reparse as formal parameters
				p.rewind(checkpoint);
				parse_parameter_list(p)
					.or_missing_with_error(p, js_parse_error::expected_parameters);
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
			parse_arrow_body(p).or_missing_with_error(p, js_parse_error::expected_arrow_body);
			return m.complete(p, JS_ARROW_FUNCTION_EXPRESSION);
		}
	}

	if let Some(params) = params_marker {
		let err = p
			.err_builder("grouping expressions cannot contain parameters")
			.primary(params.range(p), "");

		p.error(err);
		return m.complete(p, JS_UNKNOWN_EXPRESSION);
	}

	if is_empty {
		let err = p
			.err_builder("grouping expressions cannot be empty")
			.primary(start..p.cur_tok().range.start, "");

		p.error(err);
		return m.complete(p, JS_PARENTHESIZED_EXPRESSION);
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

	m.complete(p, JS_PARENTHESIZED_EXPRESSION)
}

/// A general expression.
// test sequence_expr
// 1, 2, 3, 4, 5
pub fn expr(p: &mut Parser) -> Option<CompletedMarker> {
	let first = expr_or_assignment(p)?;

	if p.at(T![,]) {
		let sequence_expr_marker = first.precede(p);

		p.bump_any();
		expr(p);

		Some(sequence_expr_marker.complete(p, JS_SEQUENCE_EXPRESSION))
	} else {
		Some(first)
	}
}

/// A primary expression such as a literal, an object, an array, or `this`.
pub fn primary_expr(p: &mut Parser) -> Option<CompletedMarker> {
	if let Present(m) = parse_literal_expression(p) {
		return Some(m);
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
			class_expression(p)
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
				if p.state.potential_arrow_start
					&& token_set![T![ident], T![yield], T![await], T!['(']].contains(p.nth(1))
				{
					// test async_arrow_expr
					// let a = async foo => {}
					// let b = async (bar) => {}
					// async (foo, bar, ...baz) => foo
					let m = p.start();
					p.bump_remap(T![async]);
					{
						let in_async_p = &mut *p.with_state(ParserState {
							in_async: true,
							..p.state.clone()
						});

						let parsed_parameters = parse_parameter_list(in_async_p);
						if parsed_parameters.is_absent() {
							// test_err async_arrow_expr_await_parameter
							// let a = async await => {}
							match parse_identifier_binding(in_async_p) {
								Valid(identifier) => {
									identifier
										.or_missing_with_error(in_async_p, expected_parameter);
								}
								Invalid(invalid) => {
									let identifier =
										invalid.or_to_unknown(in_async_p, JS_UNKNOWN_BINDING);
									let list =
										identifier.precede(in_async_p).complete(in_async_p, LIST);
									let parameter_list = list.precede(in_async_p);
									parameter_list.complete(in_async_p, JS_PARAMETER_LIST);
								}
							}
						}

						if in_async_p.at(T![:]) {
							let complete = ts_type_or_type_predicate_ann(in_async_p, T![:]);
							if let Some(mut complete) = complete {
								complete.err_if_not_ts(
									in_async_p,
								"arrow functions can only have return types in TypeScript files",
							);
							}
						}

						in_async_p.expect_required(T![=>]);

						parse_arrow_body(in_async_p)
							.or_missing_with_error(in_async_p, js_parse_error::expected_arrow_body);
					}

					m.complete(p, JS_ARROW_FUNCTION_EXPRESSION)
				} else {
					parse_reference_identifier_expression(p).unwrap()
				}
			}
		}
		T![function] => {
			// test function_expr
			// let a = function() {}
			// let b = function foo() {}

			parse_function_expression(p).unwrap()
		}
		T![ident] | T![yield] | T![await] => {
			// test identifier_reference
			// // SCRIPT
			// foo;
			// yield;
			// await;
			if p.state.potential_arrow_start && p.nth_at(1, T![=>]) && !p.has_linebreak_before_n(1)
			{
				// test arrow_expr_single_param
				// // SCRIPT
				// foo => {}
				// yield => {}
				// await => {}
				// foo =>
				// {}
				let m = p.start();
				parse_identifier_binding(p)
					.or_invalid_to_unknown(p, JS_UNKNOWN_BINDING)
					.or_missing_with_error(p, expected_identifier);
				p.bump(T![=>]);
				parse_arrow_body(p).or_missing_with_error(p, js_parse_error::expected_arrow_body);
				m.complete(p, JS_ARROW_FUNCTION_EXPRESSION)
			} else {
				parse_reference_identifier_expression(p).unwrap()
			}
		}
		// test grouping_expr
		// ((foo))
		// (foo)
		T!['('] => paren_or_arrow_expr(p, p.state.potential_arrow_start),
		T!['['] => array_expr(p),
		T!['{'] if p.state.allow_object_expr => parse_object_expression(p).unwrap(),
		T![import] => {
			let m = p.start();
			p.bump_any();

			// test import_meta
			// import.meta
			if p.eat(T![.]) {
				// test_err import_no_meta
				// import.foo
				// import.metaa
				if p.at(T![ident]) && p.token_src(&p.cur_tok()) == "meta" {
					p.bump_any();
					m.complete(p, IMPORT_META)
				} else if p.at(T![ident]) {
					let err = p
						.err_builder(&format!(
							"Expected `meta` following an import keyword, but found `{}`",
							p.token_src(&p.cur_tok())
						))
						.primary(p.cur_tok().range, "");

					p.err_and_bump(err, ERROR);
					m.complete(p, ERROR)
				} else {
					let err = p
						.err_builder("Expected `meta` following an import keyword, but found none")
						.primary(p.cur_tok().range, "");

					p.error(err);
					m.complete(p, ERROR)
				}
			} else {
				// test_err import_call_no_arg
				// let a = import();
				// foo();

				// test import_call
				// import("foo")
				p.expect_required(T!['(']);
				expr_or_assignment(p);
				p.expect_required(T![')']);
				m.complete(p, JS_IMPORT_CALL_EXPRESSION)
			}
		}
		BACKTICK => template(p, None),
		ERROR_TOKEN => {
			let m = p.start();
			p.bump_any();
			m.complete(p, ERROR)
		}
		// test_err primary_expr_invalid_recovery
		// let a = \; foo();
		_ => {
			let err = p
				.err_builder("Expected an expression, but found none")
				.primary(p.cur_tok().range, "Expected an expression here");
			#[allow(deprecated)]
			SingleTokenParseRecovery::with_error(
				p.state.expr_recovery_set,
				JS_UNKNOWN_EXPRESSION,
				err,
			)
			.enabled_braces_check()
			.recover(p);
			return None;
		}
	};

	Some(complete)
}

fn parse_reference_identifier_expression(p: &mut Parser) -> ParsedSyntax {
	parse_identifier(p, JS_REFERENCE_IDENTIFIER_EXPRESSION)
		.or_invalid_to_unknown(p, JS_UNKNOWN_EXPRESSION)
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
/// Parses an identifier if it is valid in this context or returns `Invalid` if the context isn't valid in this context.
/// An identifier is invalid if:
/// * It is named `await` inside of an async function
/// * It is named `yield` inside of a generator function or in strict mode
pub(crate) fn parse_identifier(p: &mut Parser, kind: SyntaxKind) -> ConditionalParsedSyntax {
	match p.cur() {
		T![yield] | T![await] | T![ident] => {
			let m = p.start();
			let name = p.cur_src();

			let error = match name {
				"await" if p.state.in_async => Some(
					p.err_builder("Illegal use of `await` as an identifier in an async context")
						.primary(p.cur_tok().range, ""),
				),
				"await" if p.syntax.file_kind == FileKind::Module => Some(
					p.err_builder("Illegal use of `await` as an identifier inside of a module")
						.primary(p.cur_tok().range, ""),
				),
				"yield" if p.state.in_generator => Some(
					p.err_builder("Illegal use of `yield` as an identifier in generator function")
						.primary(p.cur_tok().range, ""),
				),
				"yield" | "let" if StrictMode.is_supported(p) => Some(
					p.err_builder(&format!(
						"Illegal use of `{}` as an identifier in strict mode",
						name
					))
					.primary(p.cur_tok().range, ""),
				),

				_ => None,
			};

			p.bump_remap(T![ident]);
			let completed = m.complete(p, kind);

			if let Some(error) = error {
				p.error(error);
				Invalid(completed.into())
			} else {
				Valid(completed.into())
			}
		}
		_ => Valid(Absent),
	}
}

pub(crate) fn is_at_identifier(p: &Parser) -> bool {
	matches!(p.cur(), T![ident] | T![await] | T![yield])
}

/// A template literal such as "`abcd ${efg}`"
// test template_literal
// let a = `foo ${bar}`;
// let a = ``;
// let a = `${foo}`;
// let a = `foo`;
pub fn template(p: &mut Parser, tag: Option<CompletedMarker>) -> CompletedMarker {
	let m = tag.map(|m| m.precede(p)).unwrap_or_else(|| p.start());
	p.expect_required(BACKTICK);
	let elements_list = p.start();

	while !p.at(EOF) && !p.at(BACKTICK) {
		match p.cur() {
            TEMPLATE_CHUNK => p.bump_any(),
            DOLLARCURLY => {
                let e = p.start();
                p.bump_any();
                expr(p);
                p.expect_required(T!['}']);
                e.complete(p, TEMPLATE_ELEMENT);
            },
            t => unreachable!("Anything not template chunk or dollarcurly should have been eaten by the lexer, but {:?} was found", t),
        }
	}

	elements_list.complete(p, LIST);

	// test_err template_literal_unterminated
	// let a = `${foo} bar

	// The lexer already should throw an error for unterminated template literal
	p.eat(BACKTICK);
	m.complete(p, TEMPLATE)
}

/// An array literal such as `[foo, bar, ...baz]`.
// test array_expr
// [foo, bar];
// [foo];
// [,foo];
// [foo,];
// [,,,,,foo,,,,];
// [...a, ...b];
pub fn array_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T!['[']);
	let elements_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) {
		progress.assert_progressing(p);

		while p.at(T![,]) {
			p.start().complete(p, SyntaxKind::JS_ARRAY_HOLE);
			p.eat(T![,]);
		}

		if p.at(T![']']) {
			break;
		}

		if p.at(T![...]) {
			spread_element(p);
		} else {
			expr_or_assignment(p);
		}

		if p.at(T![']']) {
			break;
		}

		p.expect_required(T![,]);
	}
	elements_list.complete(p, LIST);

	p.expect_required(T![']']);
	m.complete(p, JS_ARRAY_EXPRESSION)
}

/// A spread element consisting of three dots and an assignment expression such as `...foo`
pub fn spread_element(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T![...]);
	expr_or_assignment(p);
	m.complete(p, JS_SPREAD)
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub fn lhs_expr(p: &mut Parser) -> Option<CompletedMarker> {
	let lhs = if p.at(T![super]) && p.nth_at(1, T!['(']) {
		let mut super_marker = super_expression(p);

		if !p.state.in_constructor {
			p.error(
				p.err_builder("`super` is only valid inside of a class constructor of a subclass.")
					.primary(super_marker.range(p), ""),
			);
			super_marker.change_kind(p, JS_UNKNOWN_EXPRESSION);
		}

		super_marker
	} else {
		member_or_new_expr(p, true)?
	};

	if lhs.kind() == JS_ARROW_FUNCTION_EXPRESSION {
		return Some(lhs);
	}

	let m = lhs.precede(p);
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

	if p.at(T!['(']) {
		args(p);
		let lhs = m.complete(p, CALL_EXPR);
		return Some(subscripts(p, lhs, false));
	}

	if type_args.is_some() {
		p.expect_required(T!['(']);
	}

	m.abandon(p);
	Some(lhs)
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test postfix_expr
// foo++
// foo--
pub fn postfix_expr(p: &mut Parser) -> Option<CompletedMarker> {
	let checkpoint = p.checkpoint();
	let lhs = lhs_expr(p);
	if !p.has_linebreak_before_n(0) {
		match p.cur() {
			T![++] => {
				let assignment_target = expression_to_simple_assignment_target(p, lhs?, checkpoint);
				let m = assignment_target.precede(p);
				p.bump(T![++]);
				let complete = m.complete(p, JS_POST_UPDATE_EXPRESSION);
				Some(complete)
			}
			T![--] => {
				let assignment_target = expression_to_simple_assignment_target(p, lhs?, checkpoint);
				let m = assignment_target.precede(p);
				p.bump(T![--]);
				let complete = m.complete(p, JS_POST_UPDATE_EXPRESSION);
				Some(complete)
			}
			_ => lhs,
		}
	} else {
		lhs
	}
}

/// A unary expression such as `!foo` or `++bar`
pub fn unary_expr(p: &mut Parser) -> Option<CompletedMarker> {
	const UNARY_SINGLE: TokenSet =
		token_set![T![delete], T![void], T![typeof], T![+], T![-], T![~], T![!]];

	// FIXME: this shouldn't allow await in sync functions
	if (p.state.in_async || p.syntax.top_level_await) && p.at(T![await]) {
		let m = p.start();
		p.bump_any();
		unary_expr(p);
		return Some(m.complete(p, JS_AWAIT_EXPRESSION));
	}

	if p.at(T![<]) {
		let m = p.start();
		p.bump_any();
		if p.eat(T![const]) {
			p.expect_required(T![>]);
			unary_expr(p);
			let mut res = m.complete(p, TS_CONST_ASSERTION);
			res.err_if_not_ts(p, "const assertions can only be used in TypeScript files");
			return Some(res);
		} else {
			ts_type(p);
			p.expect_required(T![>]);
			unary_expr(p);
			let mut res = m.complete(p, TS_ASSERTION);
			res.err_if_not_ts(p, "type assertions can only be used in TypeScript files");
			return Some(res);
		}
	}

	if p.at(T![++]) {
		let m = p.start();
		p.bump(T![++]);
		parse_simple_assignment_target(p, SimpleAssignmentTargetExprKind::Unary)
			.or_missing_with_error(p, expected_simple_assignment_target);
		let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
		return Some(complete);
	}
	if p.at(T![--]) {
		let m = p.start();
		p.bump(T![--]);
		parse_simple_assignment_target(p, SimpleAssignmentTargetExprKind::Unary)
			.or_missing_with_error(p, expected_simple_assignment_target);
		let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
		return Some(complete);
	}

	if p.at_ts(UNARY_SINGLE) {
		let m = p.start();
		let op = p.cur();
		p.bump_any();

		let res = if let Some(unary) = unary_expr(p) {
			unary
		} else {
			m.abandon(p);
			return None;
		};

		if op == T![delete] && p.typescript() {
			match res.kind() {
				JS_STATIC_MEMBER_EXPRESSION | JS_COMPUTED_MEMBER_EXPRESSION => {}
				_ => {
					let err = p
						.err_builder("the target for a delete operator must be a property access")
						.primary(res.range(p), "");

					p.error(err);
				}
			}
		}
		return Some(m.complete(p, JS_UNARY_EXPRESSION));
	}

	postfix_expr(p)
}
