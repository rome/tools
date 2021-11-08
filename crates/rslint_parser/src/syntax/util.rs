//! General utility functions for parsing and error checking.

use crate::{
	ast::{Expr, GroupingExpr, NameRef, UnaryExpr},
	SyntaxKind::*,
	*,
};

use std::collections::HashMap;

/// Check if assignment to an expression is invalid and report an error if so.
///
/// For example: `++true` is invalid.
pub fn check_simple_assign_target(p: &mut Parser, target: &Expr, range: TextRange) {
	let err = p
		.err_builder(&format!(
			"Invalid assignment to `{}`",
			target.syntax().text().to_string().trim()
		))
		.primary(range, "This expression cannot be assigned to");

	if !is_simple_assign_target(p, target) {
		p.error(err);
	}
}

fn is_simple_assign_target(p: &mut Parser, target: &Expr) -> bool {
	match target.syntax().kind() {
		NAME_REF | BRACKET_EXPR | DOT_EXPR | PRIVATE_PROP_ACCESS => true,
		GROUPING_EXPR => {
			let inner = GroupingExpr::cast(target.syntax().to_owned())
				.unwrap()
				.inner();
			if let Some(inner) = inner {
				is_simple_assign_target(p, &inner)
			} else {
				// avoid throwing extra errors for empty grouping exprs
				true
			}
		}
		_ => false,
	}
}

pub fn check_assign_target(p: &mut Parser, target: &Expr, range: TextRange, deny_call: bool) {
	if p.typescript() {
		let is_eval_or_args = target.text() == "eval" || target.text() == "arguments";
		if is_eval_or_args && p.state.strict.is_some() {
			let err = p
				.err_builder("`eval` and `arguments` cannot be assigned to in strict mode")
				.primary(range, "");

			p.error(err);
		}

		fn should_deny(e: &Expr, deny_call: bool) -> bool {
			match e {
				Expr::Literal(_) => false,
				Expr::CallExpr(_) => deny_call,
				Expr::BinExpr(_) => false,
				Expr::GroupingExpr(it) => it.inner().map_or(false, |i| should_deny(&i, deny_call)),
				_ => true,
			}
		}

		if !is_eval_or_args && !is_simple_assign_target(p, target) && should_deny(target, deny_call)
		{
			let err = p
				.err_builder("invalid assignment target")
				.primary(range, "");

			p.error(err);
		}
	} else {
		check_simple_assign_target(p, target, range);
	}
}

/// Check if the use of a statement label is valid and the label is defined.
///
/// # Panics
/// Panics if the marker is not a name with an ident
// FIXME: Labels should not cross function boundaries
pub fn check_label_use(p: &mut Parser, label: &CompletedMarker) {
	let name = p.parse_marker::<NameRef>(label).ident_token().unwrap();
	if p.state.labels.get(name.text()).is_none() {
		let err = p
			.err_builder(&format!(
				"Use of undefined statement label `{}`",
				name.text()
			))
			.primary(
				label.range(p),
				"This label is used, but it is never defined",
			);

		p.error(err);
	}
}

/// Get the precedence of a token
pub fn get_precedence(tok: SyntaxKind) -> Option<u8> {
	Some(match tok {
		T![||] | T![??] => 1,
		T![&&] => 2,
		T![|] => 3,
		T![^] => 4,
		T![&] => 5,
		T![==] | T![!=] | T![===] | T![!==] => 6,
		T![>] | T![>=] | T![<] | T![<=] => 7,
		T![<<] | T![>>] | T![>>>] => 8,
		T![+] | T![-] => 9,
		T![*] | T![/] => 10,
		T![%] | T![**] => 11,
		_ => return None,
	})
}

pub fn is_update_expr(p: &Parser, marker: &CompletedMarker) -> bool {
	match marker.kind() {
		UNARY_EXPR => p.parse_marker::<UnaryExpr>(marker).is_update(),
		_ => false,
	}
}

/// Check the bound names of a variable declaration and issue errors according to `13.3.1.1`
///
/// # Panics
/// Panics if the marker does not represent a [`VarDecl`](ast::VarDecl).
pub fn check_var_decl_bound_names(p: &mut Parser, marker: &CompletedMarker) {
	let mut map = HashMap::with_capacity(3);

	let decl = p.parse_marker::<ast::VarDecl>(marker);
	if decl.is_let() || decl.is_const() {
		for declarator in decl.declared() {
			if let Some(pattern) = declarator.pattern() {
				check_pat(p, pattern, &mut map, marker)
			}
		}
	}
}

fn check_pat(
	p: &mut Parser,
	pattern: ast::Pattern,
	map: &mut HashMap<String, Range<usize>>,
	marker: &CompletedMarker,
) {
	match pattern {
		ast::Pattern::SinglePattern(name) => {
			if let Some(ident) = name.name().map(|x| x.ident_token()).flatten() {
				check_name_pat(p, &ident, map, marker);
			}
		}
		ast::Pattern::AssignPattern(pat) => {
			if let Some(subpat) = pat.value() {
				// This should always be a pattern
				if ast::Pattern::can_cast(subpat.syntax().kind()) {
					check_pat(
						p,
						ast::Pattern::cast(subpat.syntax().to_owned()).unwrap(),
						map,
						marker,
					)
				}
			}
		}
		ast::Pattern::ObjectPattern(obj) => {
			for subpat in obj.elements() {
				let pat = match subpat {
					ast::ObjectPatternProp::AssignPattern(pat) => pat.into(),
					ast::ObjectPatternProp::KeyValuePattern(pat) => {
						if let Some(val) = pat.value() {
							val
						} else {
							return;
						}
					}
					ast::ObjectPatternProp::RestPattern(pat) => pat.into(),
					ast::ObjectPatternProp::SinglePattern(pat) => pat.into(),
				};
				check_pat(p, pat, map, marker);
			}
		}
		ast::Pattern::ArrayPattern(pat) => {
			for subpat in pat.elements() {
				check_pat(p, subpat, map, marker);
			}
		}
		ast::Pattern::RestPattern(pat) => {
			if let Some(subpat) = pat.pat() {
				check_pat(p, subpat, map, marker);
			}
		}
		ast::Pattern::ExprPattern(_) => unreachable!(),
	}
}

fn check_name_pat(
	p: &mut Parser,
	token: &SyntaxToken,
	map: &mut HashMap<String, Range<usize>>,
	marker: &CompletedMarker,
) {
	let range = marker.offset_range(p, token.text_range());
	let token_src = p.source(range).to_owned();

	if token_src == "let" {
		let err = p
			.err_builder("`let` cannot be declared as a variable name inside of a declaration")
			.primary(range, "");

		p.error(err);
	}

	if let Some(entry) = map.get(&token_src) {
		let err = p
			.err_builder(
				"Declarations inside of a `let` or `const` declaration may not have duplicates",
			)
			.secondary(
				entry.to_owned(),
				&format!("{} is first declared here", token_src),
			)
			.primary(
				range,
				&format!("a second declaration of {} is not allowed", token_src),
			);

		p.error(err);
	} else {
		map.insert(token_src.to_owned(), range.into());
	}
}

/// Check the LHS expression inside of a for...in or for...of statement according to
pub fn check_for_stmt_lhs(p: &mut Parser, expr: Expr, marker: &CompletedMarker) {
	match expr {
		Expr::NameRef(ident) => check_simple_assign_target(p, &Expr::from(ident), marker.range(p)),
		Expr::DotExpr(_) | Expr::BracketExpr(_) => {}
		Expr::AssignExpr(expr) => {
			if let Some(rhs) = expr.rhs() {
				check_for_stmt_lhs(p, rhs, marker);
			}
		}
		Expr::GroupingExpr(expr) => {
			if let Some(inner) = expr.inner() {
				check_for_stmt_lhs(p, inner, marker);
			}
		}
		Expr::ArrayExpr(expr) => {
			let elem_count = expr.elements().len();

			for (idx, elem) in expr.elements().iter().enumerate() {
				if let ast::ExprOrSpread::SpreadElement(ref spread) = elem {
					if idx != elem_count - 1 {
						let err = p.err_builder("Spread element may only occur as the last element of an assignment target")
                            .primary(marker.offset_range(p, spread.syntax().trimmed_range()), "");

						p.error(err);
					} else if let Some(element) = spread.element() {
						check_spread_element(p, element, marker);
					}
				}
				check_for_stmt_lhs(p, elem.syntax().to::<Expr>(), marker);
			}
		}
		Expr::ObjectExpr(expr) => {
			// TODO replace with expr.props().trailing_comma()
			if expr.has_trailing_comma() {
				// Untyped node machine go brr
				let comma_range = expr
					.props()
					.last()
					.unwrap()
					.syntax()
					.next_sibling_or_token()
					.unwrap()
					.into_token()
					.unwrap()
					.text_range();
				let err = p
					.err_builder("Illegal trailing comma in assignment target")
					.primary(comma_range, "");

				p.error(err);
			}

			for (idx, prop) in expr.props().iter().enumerate() {
				match prop {
					ast::ObjectProp::LiteralProp(prop) => {
						if let Some(expr) = prop.value() {
							check_for_stmt_lhs(p, expr, marker);
						}
					}
					ast::ObjectProp::SpreadProp(prop) if idx != expr.props().len() - 1 => {
						if let Some(lhs) = prop.value() {
							check_spread_element(p, lhs, marker);
						}
					}
					ast::ObjectProp::InitializedProp(_) => {}
					_ => {
						let err = p
							.err_builder("Illegal object property in assignment target")
							.primary(marker.offset_range(p, prop.syntax().trimmed_range()), "");

						p.error(err);
					}
				}
			}
		}
		_ => {
			let err = p
				.err_builder("Illegal expression in assignment target")
				.primary(marker.offset_range(p, expr.syntax().trimmed_range()), "");

			p.error(err);
		}
	}
}

fn check_spread_element(p: &mut Parser, lhs: Expr, marker: &CompletedMarker) {
	if let Expr::AssignExpr(expr) = lhs {
		let err = p
			.err_builder("Illegal spread element in assignment target")
			.primary(marker.offset_range(p, expr.syntax().trimmed_range()), "");

		p.error(err);
	} else {
		check_for_stmt_lhs(p, lhs, marker);
	}
}

pub fn check_lhs(p: &mut Parser, expr: Expr, marker: &CompletedMarker) {
	if expr.syntax().kind() == ASSIGN_EXPR {
		let err = p
			.err_builder("Illegal assignment expression in for statement")
			.primary(marker.offset_range(p, expr.syntax().trimmed_range()), "");

		p.error(err);
	} else {
		check_for_stmt_lhs(p, expr, marker);
	}
}

/// Check if the var declaration in a for statement has multiple declarators, which is invalid
pub fn check_for_stmt_declarators(p: &mut Parser, marker: &CompletedMarker) {
	let parsed = p.parse_marker::<ast::VarDecl>(marker);
	let excess = parsed.declared().iter().skip(1).collect::<Vec<_>>();

	if !excess.is_empty() {
		let start = marker
			.offset_range(p, excess.first().unwrap().syntax().trimmed_range())
			.start();
		let end = marker
			.offset_range(p, excess.last().unwrap().syntax().trimmed_range())
			.end();

		let err = p
			.err_builder("For statement variable declarations may only have one declaration")
			.primary(TextRange::new(start, end), "");

		p.error(err);
	}
}
