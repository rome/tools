//! General utility functions for parsing and error checking.

use crate::{
	ast::{JsAnyExpression, JsParenthesizedExpression},
	SyntaxKind::*,
	*,
};

/// Check if assignment to an expression is invalid and report an error if so.
///
/// For example: `++true` is invalid.
pub fn check_simple_assign_target(p: &mut Parser, target: &JsAnyExpression, range: TextRange) {
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

fn is_simple_assign_target(p: &mut Parser, target: &JsAnyExpression) -> bool {
	match target.syntax().kind() {
		JS_REFERENCE_IDENTIFIER_EXPRESSION | BRACKET_EXPR | DOT_EXPR | PRIVATE_PROP_ACCESS => true,
		JS_PARENTHESIZED_EXPRESSION => {
			let inner = JsParenthesizedExpression::cast(target.syntax().to_owned())
				.unwrap()
				.expression();
			if let Ok(inner) = inner {
				is_simple_assign_target(p, &inner)
			} else {
				// avoid throwing extra errors for empty grouping exprs
				true
			}
		}
		_ => false,
	}
}

pub fn check_assign_target(
	p: &mut Parser,
	target: &JsAnyExpression,
	range: TextRange,
	deny_call: bool,
) {
	if p.typescript() {
		let is_eval_or_args = target.text() == "eval" || target.text() == "arguments";
		if is_eval_or_args && p.state.strict.is_some() {
			let err = p
				.err_builder("`eval` and `arguments` cannot be assigned to in strict mode")
				.primary(range, "");

			p.error(err);
		}

		fn should_deny(e: &JsAnyExpression, deny_call: bool) -> bool {
			match e {
				JsAnyExpression::JsAnyLiteral(_) => false,
				JsAnyExpression::CallExpr(_) => deny_call,
				JsAnyExpression::JsBinaryExpression(_) => false,
				JsAnyExpression::JsParenthesizedExpression(it) => it
					.expression()
					.map_or(false, |i| should_deny(&i, deny_call)),
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
pub fn check_label_use(p: &mut Parser, label: &Token) {
	let name = p.token_src(label);

	if p.state.labels.get(name).is_none() {
		let err = p
			.err_builder(&format!("Use of undefined statement label `{}`", name))
			.primary(&label.range, "This label is used, but it is never defined");

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

/// Check the LHS expression inside of a for...in or for...of statement according to
pub fn check_for_stmt_lhs(p: &mut Parser, expr: JsAnyExpression, marker: &CompletedMarker) {
	match expr {
		JsAnyExpression::JsReferenceIdentifierExpression(ident) => {
			check_simple_assign_target(p, &JsAnyExpression::from(ident), marker.range(p))
		}
		JsAnyExpression::DotExpr(_) | JsAnyExpression::BracketExpr(_) => {}
		JsAnyExpression::AssignExpr(expr) => {
			if let Some(rhs) = expr.rhs() {
				check_for_stmt_lhs(p, rhs, marker);
			}
		}
		JsAnyExpression::JsParenthesizedExpression(expr) => {
			if let Ok(inner) = expr.expression() {
				check_for_stmt_lhs(p, inner, marker);
			}
		}
		JsAnyExpression::JsArrayExpression(expr) => {
			let elem_count = expr.elements().len();

			for (idx, elem) in expr.elements().iter().enumerate() {
				if let ast::JsAnyArrayElement::SpreadElement(ref spread) = elem {
					if idx != elem_count - 1 {
						let err = p.err_builder("Spread element may only occur as the last element of an assignment target")
                            .primary(marker.offset_range(p, spread.syntax().text_trimmed_range()), "");

						p.error(err);
					} else if let Ok(element) = spread.element() {
						check_spread_element(p, element, marker);
					}
				}
				check_for_stmt_lhs(p, elem.syntax().to::<JsAnyExpression>(), marker);
			}
		}
		JsAnyExpression::JsObjectExpression(expr) => {
			if let Some(trailing_comma) = expr.members().trailing_separator() {
				// Untyped node machine go brr
				let comma_range = trailing_comma.text_range();
				let err = p
					.err_builder("Illegal trailing comma in assignment target")
					.primary(comma_range, "");

				p.error(err);
			}

			for (idx, prop) in expr.members().iter().enumerate() {
				match prop {
					ast::JsAnyObjectMember::JsPropertyObjectMember(prop) => {
						if let Ok(expr) = prop.value() {
							check_for_stmt_lhs(p, expr, marker);
						}
					}
					ast::JsAnyObjectMember::JsSpread(prop) if idx != expr.members().len() - 1 => {
						if let Ok(lhs) = prop.argument() {
							check_spread_element(p, lhs, marker);
						}
					}
					ast::JsAnyObjectMember::InitializedProp(_) => {}
					_ => {
						let err = p
							.err_builder("Illegal object property in assignment target")
							.primary(marker.offset_range(p, prop.syntax().text_trimmed_range()), "");

						p.error(err);
					}
				}
			}
		}
		_ => {
			let err = p
				.err_builder("Illegal expression in assignment target")
				.primary(marker.offset_range(p, expr.syntax().text_trimmed_range()), "");

			p.error(err);
		}
	}
}

fn check_spread_element(p: &mut Parser, lhs: JsAnyExpression, marker: &CompletedMarker) {
	if let JsAnyExpression::AssignExpr(expr) = lhs {
		let err = p
			.err_builder("Illegal spread element in assignment target")
			.primary(marker.offset_range(p, expr.syntax().text_trimmed_range()), "");

		p.error(err);
	} else {
		check_for_stmt_lhs(p, lhs, marker);
	}
}

pub fn check_lhs(p: &mut Parser, expr: JsAnyExpression, marker: &CompletedMarker) {
	if expr.syntax().kind() == ASSIGN_EXPR {
		let err = p
			.err_builder("Illegal assignment expression in for statement")
			.primary(marker.offset_range(p, expr.syntax().text_trimmed_range()), "");

		p.error(err);
	} else {
		check_for_stmt_lhs(p, expr, marker);
	}
}

/// Check if the var declaration in a for statement has multiple declarators, which is invalid
pub fn check_for_stmt_declaration(p: &mut Parser, marker: &CompletedMarker) {
	let parsed = p.parse_marker::<ast::JsVariableDeclaration>(marker);
	let excess = parsed.declarators().iter().skip(1).collect::<Vec<_>>();

	if !excess.is_empty() {
		let start = marker
			.offset_range(p, excess.first().unwrap().syntax().text_trimmed_range())
			.start();
		let end = marker
			.offset_range(p, excess.last().unwrap().syntax().text_trimmed_range())
			.end();

		let err = p
			.err_builder("For statement variable declarations may only have one declaration")
			.primary(TextRange::new(start, end), "");

		p.error(err);
	}
}
