use super::expr::expr_or_assignment;
#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::syntax::expr::parse_identifier;
use crate::syntax::js_parse_error::{
	expected_object_binding_member, expected_object_binding_member_name, expected_object_member,
};
use crate::syntax::object::object_member_name;
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{SyntaxKind::*, *};

pub fn pattern(p: &mut Parser, parameters: bool) -> Option<CompletedMarker> {
	Some(match p.cur() {
		T![this] if parameters => {
			let m = p.start();
			p.bump_remap(T![ident]);
			m.complete(p, JS_IDENTIFIER_BINDING)
		}
		T!['['] => array_binding_pattern(p, parameters),
		T!['{'] if p.state.allow_object_expr => object_binding_pattern(p, parameters),
		T![ident] | T![yield] | T![await] => {
			if p.state.should_record_names {
				let string = p.cur_src().to_string();
				if string == "let" {
					let err = p
						.err_builder(
							"`let` cannot be declared as a variable name inside of a declaration",
						)
						.primary(p.cur_tok().range, "");

					p.error(err);
				} else if let Some(existing) = p.state.name_map.get(&string) {
					let err = p
                    .err_builder(
                        "Declarations inside of a `let` or `const` declaration may not have duplicates",
                    )
                    .secondary(
                        existing.to_owned(),
                        &format!("{} is first declared here", string),
                    )
                    .primary(
                        p.cur_tok().range,
                        &format!("a second declaration of {} is not allowed", string),
                    );
					p.error(err);
				} else {
					p.state
						.name_map
						.insert(p.cur_src().to_string(), p.cur_tok().range);
				}
			}
			parse_identifier_binding(p).ok().unwrap()
		}
		_ => {
			let err = p
				.err_builder("Expected an identifier or pattern, but found none")
				.primary(p.cur_tok().range, "");
			let mut ts = token_set![T![ident], T![yield], T![await], T!['['],];
			if p.state.allow_object_expr {
				ts = ts.union(token_set![T!['{']]);
			}
			#[allow(deprecated)]
			SingleTokenParseRecovery::with_error(ts, JS_UNKNOWN_BINDING, err).recover(p);
			return None;
		}
	})
}

// test_err binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
pub fn parse_identifier_binding(p: &mut Parser) -> ParsedSyntax {
	let parsed =
		parse_identifier(p, JS_IDENTIFIER_BINDING).or_invalid_to_unknown(p, JS_UNKNOWN_BINDING);

	if let Present(mut identifier) = parsed {
		let identifier_name = identifier.text(p);

		if StrictMode.is_supported(p)
			&& (identifier_name == "eval" || identifier_name == "arguments")
		{
			let err = p
				.err_builder(&format!(
					"Illegal use of `{}` as an identifier in strict mode",
					identifier_name
				))
				.primary(identifier.range(p), "");
			p.error(err);

			identifier.change_kind(p, JS_UNKNOWN_BINDING);
		}
		Present(identifier)
	} else {
		Absent
	}
}

pub fn binding_element(p: &mut Parser, parameters: bool) -> Option<CompletedMarker> {
	let left = pattern(p, parameters);

	if p.at(T![=]) {
		let m = left.map(|m| m.precede(p)).unwrap_or_else(|| p.start());
		p.bump_any();

		expr_or_assignment(p);
		return Some(m.complete(p, ASSIGN_PATTERN));
	}

	left
}

// test_err
// let [ default: , hey , ] = []
fn array_binding_pattern(p: &mut Parser, parameters: bool) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T!['[']);

	let elements_list = p.start();

	while !p.at(EOF) && !p.at(T![']']) {
		if p.eat(T![,]) {
			continue;
		}
		if p.at(T![...]) {
			let m = p.start();
			p.bump_any();

			pattern(p, parameters);

			m.complete(p, REST_PATTERN);
			break;
		} else if binding_element(p, parameters).is_none() {
			SingleTokenParseRecovery::new(
				token_set![T![await], T![ident], T![yield], T![:], T![=], T![']']],
				JS_UNKNOWN_BINDING,
			)
			.recover(p);
		}
		if !p.at(T![']']) {
			p.expect_required(T![,]);
		}
	}

	elements_list.complete(p, LIST);

	p.expect_required(T![']']);
	m.complete(p, ARRAY_PATTERN)
}

// test_err object_binding_pattern
// let { 5 } } = { eval: "foo" };
// let { eval } = { eval: "foo" };
// let { 5, 6 } = { eval: "foo" };
// let { default: , bar } = {};
pub fn object_binding_pattern(p: &mut Parser, parameters: bool) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T!['{']);
	let props_list = p.start();
	let mut first = true;

	while !p.at(EOF) && !p.at(T!['}']) {
		if first {
			first = false;
		} else {
			p.expect_required(T![,]);
			if p.at(T!['}']) {
				break;
			}
		}

		if p.at(T![,]) {
			p.missing(); // missing element
			continue;
		}

		if p.at(T![...]) {
			let m = p.start();
			p.bump_any();

			pattern(p, parameters);
			m.complete(p, REST_PATTERN);
			break;
		}

		let recovery_result = parse_property_binding(p, parameters).or_recover(
			p,
			ParseRecovery::new(
				JS_UNKNOWN_BINDING,
				token_set![T![,], T!['}'], T![=], T![...], T![=],],
			)
			.enable_recovery_on_line_break(),
			expected_object_binding_member,
		);

		if recovery_result.is_err() {
			break;
		}
	}
	props_list.complete(p, LIST);

	p.expect_required(T!['}']);
	m.complete(p, JS_OBJECT_BINDING)
}

// test object_binding_prop
// let { default: foo, bar } = {}
// let { foo = bar, baz } = {}
fn parse_property_binding(p: &mut Parser, parameters: bool) -> ParsedSyntax {
	let inner = if p.nth_at(1, T![:]) {
		let m = p.start();
		object_member_name(p).or_missing_with_error(p, expected_object_member);
		p.bump(T![:]);
		binding_element(p, parameters);
		Present(m.complete(p, JS_PROPERTY_BINDING))
	} else {
		parse_identifier_binding(p).map(|identifier| {
			let m = identifier.precede(p);
			m.complete(p, JS_SHORTHAND_PROPERTY_BINDING)
		})
	};

	if p.at(T![=]) {
		let assign_pattern =
			inner.precede_or_missing_with_error(p, expected_object_binding_member_name);
		p.bump(T![=]);
		expr_or_assignment(p);
		Present(assign_pattern.complete(p, ASSIGN_PATTERN))
	} else {
		inner
	}
}
