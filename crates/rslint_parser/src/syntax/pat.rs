use super::expr::{assign_expr, identifier_name, identifier_reference, lhs_expr, object_prop_name};
use crate::{SyntaxKind::*, *};

pub fn pattern(p: &mut Parser, parameters: bool, assignment: bool) -> Option<CompletedMarker> {
	Some(match p.cur() {
		T![this] if parameters => {
			let m = p.start();
			let _m = p.start();
			p.bump_remap(T![ident]);
			_m.complete(p, NAME);
			m.complete(p, SINGLE_PATTERN)
		}
		T!['['] => array_binding_pattern(p, parameters, assignment),
		T!['{'] if p.state.allow_object_expr => object_binding_pattern(p, parameters),
		_ if assignment => {
			let m = p.start();
			let mut complete = lhs_expr(p)?;
			if complete.kind() == NAME_REF {
				complete.change_kind(p, NAME);
			}
			m.complete(
				p,
				if complete.kind() == NAME {
					SINGLE_PATTERN
				} else {
					EXPR_PATTERN
				},
			)
		}
		T![ident] | T![yield] | T![await] => {
			let m = p.start();
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
			binding_identifier(p);
			m.complete(p, SINGLE_PATTERN)
		}
		_ => {
			let err = p
				.err_builder("Expected an identifier or pattern, but found none")
				.primary(p.cur_tok().range, "");
			let mut ts = token_set![T![ident], T![yield], T![await], T!['['],];
			if p.state.allow_object_expr {
				ts = ts.union(token_set![T!['{']]);
			}
			p.err_recover(err, ts, false);
			return None;
		}
	})
}

pub fn opt_binding_identifier(p: &mut Parser) -> Option<CompletedMarker> {
	const BINDING_IDENTS: TokenSet = token_set![T![ident], T![yield], T![await]];

	if p.at_ts(BINDING_IDENTS) {
		binding_identifier(p)
	} else {
		None
	}
}

// test_err binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
pub fn binding_identifier(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![yield]) && p.state.in_generator {
		let err = p
			.err_builder("Illegal use of `yield` as an identifier in generator function")
			.primary(p.cur_tok().range, "");

		p.error(err);
	}

	if p.at(T![await]) && p.state.in_async {
		let err = p
			.err_builder("Illegal use of `await` as an identifier in an async context")
			.primary(p.cur_tok().range, "");

		p.error(err);
	}

	if p.state.strict.is_some()
		&& (p.cur_src() == "eval" || p.cur_src() == "arguments" || p.cur_src() == "yield")
	{
		let err = p
			.err_builder(&format!(
				"Illegal use of `{}` as an identifier in strict mode",
				p.cur_src()
			))
			.primary(p.cur_tok().range, "");

		p.error(err);
	}

	let mut m = identifier_reference(p)?;
	m.change_kind(p, NAME);
	Some(m)
}

pub fn binding_element(
	p: &mut Parser,
	parameters: bool,
	assignment: bool,
) -> Option<CompletedMarker> {
	let left = pattern(p, parameters, assignment);

	if p.at(T![=]) {
		let m = left.map(|m| m.precede(p)).unwrap_or_else(|| p.start());
		p.bump_any();

		assign_expr(p);
		return Some(m.complete(p, ASSIGN_PATTERN));
	}

	left
}

pub fn array_binding_pattern(
	p: &mut Parser,
	parameters: bool,
	assignment: bool,
) -> CompletedMarker {
	let m = p.start();
	p.expect(T!['[']);

	let elements_list = p.start();

	while !p.at(EOF) && !p.at(T![']']) {
		if p.eat(T![,]) {
			continue;
		}
		if p.at(T![...]) {
			let m = p.start();
			p.bump_any();

			pattern(p, parameters, assignment);

			m.complete(p, REST_PATTERN);
			break;
		} else if binding_element(p, parameters, assignment).is_none() {
			p.err_recover_no_err(
				token_set![T![await], T![ident], T![yield], T![:], T![=], T![']']],
				false,
			);
		}
		if !p.at(T![']']) {
			p.expect(T![,]);
		}
	}

	elements_list.complete(p, LIST);

	p.expect(T![']']);
	m.complete(p, ARRAY_PATTERN)
}

pub fn object_binding_pattern(p: &mut Parser, parameters: bool) -> CompletedMarker {
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

		if p.at(T![...]) {
			let m = p.start();
			p.bump_any();

			pattern(p, parameters, false);
			m.complete(p, REST_PATTERN);
			break;
		}

		object_binding_prop(p, parameters);
	}
	props_list.complete(p, LIST);

	p.expect(T!['}']);
	m.complete(p, OBJECT_PATTERN)
}

// test object_binding_prop
// let { default: foo, bar } = {}
// let { foo = bar, baz } = {}
fn object_binding_prop(p: &mut Parser, parameters: bool) -> Option<CompletedMarker> {
	let m = p.start();
	let name = if (p.cur().is_keyword() || p.cur() == T![ident]) && p.nth(1) == T![:] {
		identifier_name(p)
	} else {
		object_prop_name(p, true)
	};

	if p.eat(T![:]) {
		binding_element(p, parameters, false);
		return Some(m.complete(p, KEY_VALUE_PATTERN));
	}

	let name = if let Some(n) = name {
		n
	} else {
		p.err_recover_no_err(
			token_set![T![await], T![ident], T![yield], T![:], T![=], T!['}']],
			false,
		);
		return None;
	};

	if name.kind() != NAME {
		let err = p
			.err_builder("Expected an identifier for a pattern, but found none")
			.primary(name.range(p), "");

		p.error(err);
		return None;
	}

	let sp_marker = name.precede(p).complete(p, SINGLE_PATTERN);
	if p.eat(T![=]) {
		assign_expr(p);
		Some(m.complete(p, ASSIGN_PATTERN))
	} else {
		Some(sp_marker)
	}
}
