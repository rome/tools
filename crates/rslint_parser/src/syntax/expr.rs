//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-11).

use syntax::decl::is_semi;

use super::decl::{
	arrow_body, class_decl, formal_parameters, function_decl, maybe_private_name, method,
};
use super::pat::pattern;
use super::typescript::*;
use super::util::*;
use crate::{SyntaxKind::*, *};

pub const LITERAL: TokenSet = token_set![TRUE_KW, FALSE_KW, NUMBER, STRING, NULL_KW, REGEX];

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
]
.union(LITERAL);

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
pub fn literal(p: &mut Parser) -> Option<CompletedMarker> {
	if !p.at_ts(LITERAL) {
		return None;
	}
	let m = p.start();
	p.bump_any();
	Some(m.complete(p, SyntaxKind::LITERAL))
}

/// An assignment expression such as `foo += bar` or `foo = 5`.
pub fn assign_expr(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![<])
		&& (token_set![T![ident], T![await], T![yield]].contains(p.nth(1)) || p.nth(1).is_keyword())
	{
		let res = try_parse_ts(p, |p| {
			let m = p.start();
			ts_type_params(p)?;
			let res = assign_expr_base(p);
			if res.map(|x| x.kind()) != Some(ARROW_EXPR) {
				m.abandon(p);
				None
			} else {
				res.unwrap().undo_completion(p).abandon(p);
				Some(m.complete(p, ARROW_EXPR))
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

pub(crate) fn is_valid_target(p: &mut Parser, marker: &CompletedMarker) -> bool {
	match marker.kind() {
		DOT_EXPR | BRACKET_EXPR | NAME_REF | PRIVATE_PROP_ACCESS | TS_CONST_ASSERTION
		| TS_ASSERTION | TS_NON_NULL => true,
		GROUPING_EXPR => {
			// avoid parsing the marker because it is incredibly expensive and this is a hot path
			for (idx, event) in p.events[marker.start_pos as usize..].iter().enumerate() {
				match event {
					Event::Finish { .. } if marker.finish_pos as usize == idx => return true,
					Event::Start {
						kind: SyntaxKind::GROUPING_EXPR,
						..
					} => {}
					Event::Start {
						kind: SyntaxKind::TOMBSTONE,
						..
					} => {}
					Event::Start { kind, .. } => {
						return matches!(
							kind,
							DOT_EXPR
								| BRACKET_EXPR | NAME_REF | PRIVATE_PROP_ACCESS
								| TS_CONST_ASSERTION | TS_ASSERTION
								| TS_NON_NULL
						);
					}
					_ => {}
				}
			}
			true
		}
		_ => false,
	}
}

fn check_assign_target_from_marker(p: &mut Parser, marker: &CompletedMarker) {
	if !is_valid_target(p, marker) {
		let err = p
			.err_builder(&format!(
				"Invalid assignment to `{}`",
				p.source(marker.range(p))
			))
			.primary(marker.range(p), "This expression cannot be assigned to");
		p.error(err);
	}
}

// test assign_expr
// foo += bar = b ??= 3;
// foo -= bar;
// [foo, bar] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz], foo } = {});
fn assign_expr_recursive(
	p: &mut Parser,
	mut target: CompletedMarker,
	checkpoint: Checkpoint,
) -> Option<CompletedMarker> {
	// TODO: dont always reparse as pattern since it will yield wonky errors for `(foo = true) = bar`
	if p.at_ts(ASSIGN_TOKENS) {
		if p.at(T![=]) {
			if !is_valid_target(p, &target) && target.kind() != TEMPLATE {
				p.rewind(checkpoint);
				target = pattern(p, false, true)?;
			}
		} else {
			if !is_valid_target(p, &target) {
				let err = p
					.err_builder(&format!(
						"Invalid assignment to `{}`",
						p.source(target.range(p)).trim()
					))
					.primary(target.range(p), "This expression cannot be assigned to");

				p.error(err);
			}
			let text = p.source(target.range(p));
			if (text == "eval" || text == "arguments") && p.state.strict.is_some() && p.typescript()
			{
				let err = p
					.err_builder("`eval` and `arguments` cannot be assigned to")
					.primary(target.range(p), "");

				p.error(err);
			}
		}
		let m = target.precede(p);
		p.bump_any();
		assign_expr(p);
		Some(m.complete(p, ASSIGN_EXPR))
	} else {
		Some(target)
	}
}

// test yield_expr
// function *foo() {
//  yield foo;
//  yield* foo;
// }
pub fn yield_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![yield]);

	if !is_semi(p, 0) && (p.at(T![*]) || p.at_ts(STARTS_EXPR)) {
		p.eat(T![*]);
		assign_expr(p);
	}

	m.complete(p, YIELD_EXPR)
}

/// A conditional expression such as `foo ? bar : baz`
// test conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz
pub fn conditional_expr(p: &mut Parser) -> Option<CompletedMarker> {
	// test_err conditional_expr_err
	// foo ? bar baz
	// foo ? bar baz ? foo : bar
	let lhs = binary_expr(p);

	if p.at(T![?]) {
		let m = lhs?.precede(p);
		p.bump_any();
		assign_expr(&mut *p.with_state(ParserState {
			in_cond_expr: true,
			..p.state.clone()
		}));
		p.expect(T![:]);
		assign_expr(p);
		return Some(m.complete(p, COND_EXPR));
	}
	lhs
}

/// A binary expression such as `2 + 2` or `foo * bar + 2`
pub fn binary_expr(p: &mut Parser) -> Option<CompletedMarker> {
	let left = unary_expr(p);
	binary_expr_recursive(p, left, 0)
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
// 1 + 1 + 1 + 1
// 5 + 6 - 1 * 2 / 1 ** 6

// test_err binary_expressions_err
// foo(foo +);
// foo + * 2;
// !foo * bar;
fn binary_expr_recursive(
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
		return binary_expr_recursive(p, Some(res), min_prec);
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

	binary_expr_recursive(
		p,
		right,
		// ** is right recursive
		if op == T![**] {
			precedence - 1
		} else {
			precedence
		},
	);

	let complete = m.complete(p, BIN_EXPR);
	binary_expr_recursive(p, Some(complete), min_prec)

	// FIXME(RDambrosio016): We should check for nullish-coalescing and logical expr being used together,
	// however, i can't figure out a way to do this efficiently without using parse_marker which is way too
	// expensive to use since this is a hot path
}

/// A parenthesis expression, also called a grouping expression.
///
/// `"(" Expr ")"`
pub fn paren_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T!['(']);
	expr(p);
	p.expect(T![')']);
	m.complete(p, GROUPING_EXPR)
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

		let complete = member_or_new_expr(p, new_expr)?;
		if complete.kind() == ARROW_EXPR {
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
	if p.at(T![super]) && token_set!(T![.], T!['[']).contains(p.nth(1)) {
		let m = p.start();
		p.bump_any();
		let lhs = match p.cur() {
			T![.] => {
				p.bump_any();
				identifier_name(p);
				m.complete(p, DOT_EXPR)
			}
			T!['['] => {
				p.bump_any();
				expr(p);
				p.expect(T![']']);
				m.complete(p, BRACKET_EXPR)
			}
			_ => unreachable!(),
		};
		return Some(subscripts(p, lhs, true));
	}

	let lhs = primary_expr(p)?;
	Some(subscripts(p, lhs, true))
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
	while !p.at(EOF) {
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
			T![?.] if p.nth_at(1, T!['[']) => lhs = bracket_expr(p, lhs, true),
			T!['['] => lhs = bracket_expr(p, lhs, false),
			T![?.] => lhs = dot_expr(p, lhs, true),
			T![.] => lhs = dot_expr(p, lhs, false),
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

/// A dot expression for accessing a property
// test dot_expr
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
pub fn dot_expr(p: &mut Parser, lhs: CompletedMarker, optional_chain: bool) -> CompletedMarker {
	let m = lhs.precede(p);
	let range = if optional_chain {
		Some(p.cur_tok().range).filter(|_| p.expect(T![?.]))
	} else {
		p.expect(T![.]);
		None
	};
	if let Some(priv_range) = maybe_private_name(p).filter(|x| x.kind() == PRIVATE_NAME) {
		if !p.syntax.class_fields {
			let err = p
				.err_builder("private identifiers are unsupported")
				.primary(priv_range.range(p), "");

			p.error(err);
			return m.complete(p, ERROR);
		}
		if let Some(range) = range {
			let err = p
				.err_builder("optional chaining cannot contain private identifiers")
				.primary(range, "");

			p.error(err);
			m.complete(p, ERROR)
		} else {
			m.complete(p, PRIVATE_PROP_ACCESS)
		}
	} else {
		m.complete(p, DOT_EXPR)
	}
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test bracket_expr
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
pub fn bracket_expr(p: &mut Parser, lhs: CompletedMarker, optional_chain: bool) -> CompletedMarker {
	// test_err bracket_expr_err
	// foo[]
	// foo?.[]
	// foo[
	let m = lhs.precede(p);
	if optional_chain {
		p.expect(T![?.]);
	}
	p.expect(T!['[']);
	expr(p);
	p.expect(T![']']);
	m.complete(p, BRACKET_EXPR)
}

/// An identifier name, either an ident or a keyword
pub fn identifier_name(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	match p.cur() {
		t if t.is_keyword() || t == T![ident] => p.bump_remap(T![ident]),
		_ => {
			let err = p
				.err_builder("Expected an identifier or keyword")
				.primary(p.cur_tok().range, "Expected an identifier or keyword here");
			p.error(err);
			m.abandon(p);
			return None;
		}
	}
	Some(m.complete(p, NAME))
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`

// test_err invalid_arg_LIST
// foo(a,b;
// foo(a,b var
pub fn args(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T!['(']);
	let args_list = p.start();

	while !p.at(EOF) && !p.at(T![')']) {
		if p.at(T![...]) {
			spread_element(p);
		} else {
			assign_expr(p);
		}

		if p.at(T![,]) {
			p.bump_any();
		} else {
			break;
		}
	}

	args_list.complete(p, LIST);
	p.expect(T![')']);
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
pub fn paren_or_arrow_expr(p: &mut Parser, can_be_arrow: bool) -> CompletedMarker {
	let m = p.start();
	let checkpoint = p.checkpoint();
	let start = p.cur_tok().range.start;
	p.expect(T!['(']);
	let mut spread_range = None;
	let mut trailing_comma_marker = None;
	let mut had_comma = false;
	let mut params_marker = None;

	let mut temp = p.with_state(ParserState {
		potential_arrow_start: true,
		..p.state.clone()
	});
	let expr_m = temp.start();
	let mut is_empty = false;

	if temp.eat(T![')']) {
		is_empty = true;
	} else {
		loop {
			if temp.at(T![...]) {
				let m = temp.start();
				temp.bump_any();
				pattern(&mut *temp, false, false);
				if temp.eat(T![:]) {
					if let Some(mut ty) = ts_type(&mut *temp) {
						ty.err_if_not_ts(
							&mut *temp,
							"spread elements can only have type annotations in TypeScript files",
						);
					}
				}
				let complete = m.complete(&mut *temp, REST_PATTERN);
				spread_range = Some(complete.range(&*temp));
				if !temp.eat(T![')']) {
					if temp.eat(T![=]) {
						// formal params will handle this error
						assign_expr(&mut *temp);
						temp.expect(T![')']);
					} else {
						let err = temp.err_builder(&format!("expect a closing parenthesis after a spread element, but instead found `{}`", temp.cur_src()))
                    .primary(temp.cur_tok().range, "");

						temp.err_recover(err, EXPR_RECOVERY_SET, false);
					}
				}
				break;
			}
			let expr = assign_expr(&mut *temp);
			if expr.is_some() && temp.at(T![:]) {
				temp.rewind(checkpoint);
				params_marker = Some(formal_parameters(&mut *temp));
				break;
			}

			let sub_m = temp.start();
			if temp.eat(T![,]) {
				if temp.at(T![')']) {
					trailing_comma_marker = Some(sub_m.complete(&mut *temp, ERROR));
					temp.bump_any();
					break;
				} else {
					sub_m.abandon(&mut *temp);
				}
				had_comma = true;
			} else {
				sub_m.abandon(&mut *temp);
				if had_comma {
					expr_m.complete(&mut *temp, SEQUENCE_EXPR);
				}
				temp.expect(T![')']);
				break;
			}
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
			formal_parameters(p);
			if p.at(T![:]) {
				if let Some(mut ret) = ts_type_or_type_predicate_ann(p, T![:]) {
					ret.err_if_not_ts(
						p,
						"arrow functions can only have return types in TypeScript files",
					);
				}
			}
			p.expect_no_recover(T![=>])?;
			arrow_body(p)?;
			Some(())
		};
		// we can't just rewind the parser, since the function rewinds, and cloning and replacing the
		// events does not work apparently, therefore we need to clone the entire parser
		let cloned = p.clone();
		if func(p).is_some() {
			let c = m.complete(p, ARROW_EXPR);
			return c;
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
				formal_parameters(p);
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
			arrow_body(p);
			return m.complete(p, ARROW_EXPR);
		}
	}

	if let Some(params) = params_marker {
		let err = p
			.err_builder("grouping expressions cannot contain parameters")
			.primary(params.range(p), "");

		p.error(err);
		return m.complete(p, ERROR);
	}

	if is_empty {
		let err = p
			.err_builder("grouping expressions cannot be empty")
			.primary(start..p.cur_tok().range.start, "");

		p.error(err);
		return m.complete(p, GROUPING_EXPR);
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

	m.complete(p, GROUPING_EXPR)
}

pub fn expr_or_spread(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![...]) {
		let m = p.start();
		p.bump_any();
		assign_expr(p);
		Some(m.complete(p, SPREAD_ELEMENT))
	} else {
		assign_expr(p)
	}
}

/// A general expression.
// test sequence_expr
// 1, 2, 3, 4, 5
pub fn expr(p: &mut Parser) -> Option<CompletedMarker> {
	let first = assign_expr(p)?;

	if p.at(T![,]) {
		let list_marker = first.precede(p);

		// let m = LIST.precede(p);
		p.bump_any();
		assign_expr(p)?;

		while p.at(T![,]) {
			p.bump_any();
			assign_expr(p)?;
		}

		let list = list_marker.complete(p, LIST);
		let m = list.precede(p);

		return Some(m.complete(p, SEQUENCE_EXPR));
	}

	Some(first)
}

/// A primary expression such as a literal, an object, an array, or `this`.
pub fn primary_expr(p: &mut Parser) -> Option<CompletedMarker> {
	if let Some(m) = literal(p) {
		return Some(m);
	}

	let complete = match p.cur() {
		T![this] => {
			// test this_expr
			// this
			// this.foo
			let m = p.start();
			p.bump_any();
			m.complete(p, THIS_EXPR)
		}
		T![class] => {
			// test class_expr
			// let a = class {};
			// let a = class foo {
			//  constructor() {}
			// }
			// foo[class {}]
			let mut m = class_decl(p, true);
			m.change_kind(p, CLASS_EXPR);
			m
		}
		// test async_ident
		// let a = async;
		T![ident] if p.cur_src() == "async" => {
			// test async_function_expr
			// let a = async function() {};
			// let b = async function foo() {};
			if p.nth_at(1, T![function]) {
				let m = p.start();
				p.bump_remap(T![async]);
				let mut complete = function_decl(
					&mut *p.with_state(ParserState {
						in_async: true,
						..p.state.clone()
					}),
					m,
					true,
				);
				complete.change_kind(p, FN_EXPR);
				complete
			} else {
				// `async a => {}` and `async (a) => {}`
				if p.state.potential_arrow_start
					&& token_set![T![ident], T![yield], T!['(']].contains(p.nth(1))
				{
					// test async_arrow_expr
					// let a = async foo => {}
					// let b = async (bar) => {}
					// async (foo, bar, ...baz) => foo
					let m = p.start();
					p.bump_remap(T![async]);
					if p.at(T!['(']) {
						formal_parameters(p);
					} else {
						let m = p.start();
						// test_err async_arrow_expr_await_parameter
						// let a = async await => {}
						p.bump_remap(T![ident]);
						m.complete(p, NAME);
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
					arrow_body(&mut *p.with_state(ParserState {
						in_async: true,
						..p.state.clone()
					}));
					m.complete(p, ARROW_EXPR)
				} else {
					identifier_reference(p)?
				}
			}
		}
		T![function] => {
			// test function_expr
			// let a = function() {}
			// let b = function foo() {}
			let m = p.start();
			let mut complete = function_decl(p, m, true);
			complete.change_kind(p, FN_EXPR);
			complete
		}
		T![ident] | T![yield] | T![await] => {
			// test identifier_reference
			// foo;
			// yield;
			// await;
			let mut ident = identifier_reference(p)?;
			if p.state.potential_arrow_start && p.at(T![=>]) && !p.has_linebreak_before_n(0) {
				// test arrow_expr_single_param
				// foo => {}
				// yield => {}
				// await => {}
				// foo =>
				// {}

				// parameters are binding so we need to change the kind from NAME_REF to NAME
				ident.change_kind(p, NAME);
				let m = ident.precede(p);
				p.bump_any();
				arrow_body(p);
				m.complete(p, ARROW_EXPR)
			} else {
				ident
			}
		}
		// test grouping_expr
		// ((foo))
		// (foo)
		T!['('] => paren_or_arrow_expr(p, p.state.potential_arrow_start),
		T!['['] => array_expr(p),
		T!['{'] if p.state.allow_object_expr => object_expr(p),
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

					p.err_and_bump(err);
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
				p.expect(T!['(']);
				assign_expr(p);
				p.expect(T![')']);
				m.complete(p, IMPORT_CALL)
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
			p.err_recover(err, p.state.expr_recovery_set, true);
			return None;
		}
	};

	Some(complete)
}

pub fn identifier_reference(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur() {
		T![ident] | T![yield] | T![await] => {
			let m = p.start();
			p.bump_remap(T![ident]);
			Some(m.complete(p, NAME_REF))
		}
		_ => {
			let err = p
				.err_builder("Expected an identifier, but found none")
				.primary(p.cur_tok().range, "");

			p.err_recover(err, p.state.expr_recovery_set, true);
			None
		}
	}
}

/// A template literal such as "`abcd ${efg}`"
// test template_literal
// let a = `foo ${bar}`;
// let a = ``;
// let a = `${foo}`;
// let a = `foo`;
pub fn template(p: &mut Parser, tag: Option<CompletedMarker>) -> CompletedMarker {
	let m = tag.map(|m| m.precede(p)).unwrap_or_else(|| p.start());
	p.expect(BACKTICK);
	let elements_list = p.start();

	while !p.at(EOF) && !p.at(BACKTICK) {
		match p.cur() {
            TEMPLATE_CHUNK => p.bump_any(),
            DOLLARCURLY => {
                let e = p.start();
                p.bump_any();
                expr(p);
                p.expect(T!['}']);
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
	p.expect(T!['[']);
	let elements_list = p.start();

	while !p.at(EOF) {
		while p.eat(T![,]) {}

		if p.at(T![']']) {
			break;
		}

		if p.at(T![...]) {
			spread_element(p);
		} else {
			assign_expr(p);
		}

		if p.at(T![']']) {
			break;
		}

		p.expect(T![,]);
	}
	elements_list.complete(p, LIST);

	p.expect(T![']']);
	m.complete(p, ARRAY_EXPR)
}

/// A spread element consisting of three dots and an assignment expression such as `...foo`
pub fn spread_element(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![...]);
	assign_expr(p);
	m.complete(p, SPREAD_ELEMENT)
}

/// An object literal such as `{ a: b, "b": 5 + 5 }`.
// test object_expr
// let a = {};
// let b = {foo,}
pub fn object_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T!['{']);
	let props_list = p.start();
	let mut first = true;

	while !p.at(EOF) && !p.at(T!['}']) {
		if first {
			first = false;
		} else {
			p.expect(T![,]);
			if p.at(T!['}']) {
				break;
			}
		}
		object_property(p);
	}
	props_list.complete(p, LIST);

	p.expect(T!['}']);
	m.complete(p, OBJECT_EXPR)
}

const STARTS_OBJ_PROP: TokenSet =
	token_set![STRING, NUMBER, T![ident], T![await], T![yield], T!['[']];

/// An individual object property such as `"a": b` or `5: 6 + 6`.
pub fn object_property(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();

	match p.cur() {
		// test object_expr_getter_setter
		// let a = {
		//  get foo() {
		//    return foo;
		//  }
		// }
		//
		// let b = {
		//  set [foo](bar) {
		//     return 5;
		//  }
		// }
		T![ident]
			if (p.cur_src() == "get" || p.cur_src() == "set")
				&& (p.nth_at(1, T![ident]) || p.nth_at(1, T!['['])) =>
		{
			method(p, None, None)
		}
		// test object_expr_async_method
		// let a = {
		//   async foo() {},
		//   async *foo() {}
		// }
		T![ident]
			if p.cur_src() == "async"
				&& !p.has_linebreak_before_n(1)
				&& (STARTS_OBJ_PROP.contains(p.nth(1)) || p.nth_at(1, T![*])) =>
		{
			method(p, None, None)
		}
		// test object_expr_spread_prop
		// let a = {...foo}
		T![...] => {
			p.bump_any();
			assign_expr(p);
			Some(m.complete(p, SPREAD_PROP))
		}
		T![*] => {
			// test object_expr_generator_method
			// let b = { *foo() {} }
			let m = p.start();
			method(p, m, None)
		}
		_ => {
			let prop = object_prop_name(p, false);
			// test object_expr_assign_prop
			// let b = { foo = 4, foo = bar }
			if let Some(NAME) = prop.map(|m| m.kind()) {
				if p.eat(T![=]) {
					assign_expr(p);
					return Some(m.complete(p, INITIALIZED_PROP));
				}
			}

			// test object_expr_method
			// let b = {
			//  foo() {},
			// }
			if p.at(T!['(']) || p.at(T![<]) {
				method(p, m, None)
			} else {
				// test_err object_expr_error_prop_name
				// let a = { /: 6, /: /foo/ }
				// let a = {{}}
				if prop.is_none() {
					p.err_recover_no_err(token_set![T![:], T![,]], false);
				}
				// test_err object_expr_non_ident_literal_prop
				// let b = {5}

				// test object_expr_ident_literal_prop
				// let b = { a: true }
				if p.at(T![:]) || prop?.kind() != NAME {
					p.expect(T![:]);
					assign_expr(p);
					Some(m.complete(p, LITERAL_PROP))
				} else {
					// test object_expr_ident_prop
					// let b = {foo}
					Some(m.complete(p, IDENT_PROP))
				}
			}
		}
	}
}

// test object_prop_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
pub fn object_prop_name(p: &mut Parser, binding: bool) -> Option<CompletedMarker> {
	match p.cur() {
		STRING | NUMBER => literal(p),
		T!['['] => {
			let m = p.start();
			p.bump_any();
			assign_expr(p);
			p.expect(T![']']);
			Some(m.complete(p, COMPUTED_PROPERTY_NAME))
		}
		_ if binding => super::pat::binding_identifier(p),
		_ => identifier_name(p),
	}
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub fn lhs_expr(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![super]) && p.nth_at(1, T!['(']) {
		let m = p.start();
		p.bump_any();
		args(p);
		let lhs = m.complete(p, SUPER_CALL);
		return Some(subscripts(p, lhs, false));
	}

	let lhs = member_or_new_expr(p, true)?;
	if lhs.kind() == ARROW_EXPR {
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
		p.expect(T!['(']);
	}

	m.abandon(p);
	Some(lhs)
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test postfix_expr
// foo++
// foo--
pub fn postfix_expr(p: &mut Parser) -> Option<CompletedMarker> {
	let lhs = lhs_expr(p);
	if !p.has_linebreak_before_n(0) {
		match p.cur() {
			T![++] => {
				check_assign_target_from_marker(p, &lhs?);
				let m = lhs?.precede(p);
				p.bump(T![++]);
				let complete = m.complete(p, UNARY_EXPR);
				Some(complete)
			}
			T![--] => {
				check_assign_target_from_marker(p, &lhs?);
				let m = lhs?.precede(p);
				p.bump(T![--]);
				let complete = m.complete(p, UNARY_EXPR);
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

	// FIXME: this shouldnt allow await in sync functions
	if (p.state.in_async || p.syntax.top_level_await) && p.at(T![await]) {
		let m = p.start();
		p.bump_any();
		unary_expr(p);
		return Some(m.complete(p, AWAIT_EXPR));
	}

	if p.at(T![<]) {
		let m = p.start();
		p.bump_any();
		if p.eat(T![const]) {
			p.expect(T![>]);
			unary_expr(p);
			let mut res = m.complete(p, TS_CONST_ASSERTION);
			res.err_if_not_ts(p, "const assertions can only be used in TypeScript files");
			return Some(res);
		} else {
			ts_type(p);
			p.expect(T![>]);
			unary_expr(p);
			let mut res = m.complete(p, TS_ASSERTION);
			res.err_if_not_ts(p, "type assertions can only be used in TypeScript files");
			return Some(res);
		}
	}

	if p.at(T![++]) {
		let m = p.start();
		p.bump(T![++]);
		let right = unary_expr(p)?;
		let complete = m.complete(p, UNARY_EXPR);
		check_assign_target_from_marker(p, &right);
		return Some(complete);
	}
	if p.at(T![--]) {
		let m = p.start();
		p.bump(T![--]);
		let right = unary_expr(p)?;
		let complete = m.complete(p, UNARY_EXPR);
		check_assign_target_from_marker(p, &right);
		return Some(complete);
	}

	if p.at_ts(UNARY_SINGLE) {
		let m = p.start();
		let op = p.cur();
		p.bump_any();
		let res = unary_expr(p)?;
		if op == T![delete] && p.typescript() {
			match res.kind() {
				DOT_EXPR | BRACKET_EXPR => {}
				_ => {
					let err = p
						.err_builder("the target for a delete operator must be a property access")
						.primary(res.range(p), "");

					p.error(err);
				}
			}
		}
		return Some(m.complete(p, UNARY_EXPR));
	}

	postfix_expr(p)
}
