#[allow(deprecated)]
use crate::syntax::assignment_target::ObjectPattern;
use crate::syntax::assignment_target::{ArrayPattern, PatternWithDefault};
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{parse_identifier, EXPR_RECOVERY_SET};
use crate::syntax::js_parse_error::{
	expected_assignment_target, expected_identifier, expected_object_member_name, expected_pattern,
	expected_property_binding,
};
use crate::syntax::object::{is_at_object_member_name, object_member_name};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{SyntaxKind::*, *};
use rslint_errors::Span;

pub(crate) fn parse_binding(p: &mut Parser, parameters: bool) -> ParsedSyntax {
	match p.cur() {
		T![this] if parameters => {
			let m = p.start();
			p.bump_remap(T![ident]);
			Present(m.complete(p, JS_IDENTIFIER_BINDING))
		}
		T!['['] => array_binding_pattern(p, parameters),
		T!['{'] if p.state.allow_object_expr => object_binding_pattern(p, parameters),
		T![ident] | T![yield] | T![await] => parse_identifier_binding(p),
		_ => Absent,
	}
}

// test_err binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
pub(crate) fn parse_identifier_binding(p: &mut Parser) -> ParsedSyntax {
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
		} else if p.state.should_record_names {
			if identifier_name == "let" {
				let err = p
					.err_builder(
						"`let` cannot be declared as a variable name inside of a declaration",
					)
					.primary(identifier.range(p), "");

				p.error(err);
			} else if let Some(existing) = p.state.name_map.get(identifier_name) {
				let err = p
					.err_builder(
						"Declarations inside of a `let` or `const` declaration may not have duplicates",
					)
					.secondary(
						existing.to_owned(),
						&format!("{} is first declared here", identifier_name),
					)
					.primary(
						identifier.range(p),
						&format!("a second declaration of {} is not allowed", identifier_name),
					);
				p.error(err);
			} else {
				let identifier_name = String::from(identifier_name);
				p.state
					.name_map
					.insert(identifier_name, identifier.range(p).as_range());
			}
		}

		Present(identifier)
	} else {
		Absent
	}
}

pub(crate) fn parse_binding_with_optional_default(
	p: &mut Parser,
	parameters: bool,
) -> ParsedSyntax {
	BindingWithDefault { parameters }.parse_pattern_with_optional_default(p)
}

struct BindingWithDefault {
	parameters: bool,
}

impl PatternWithDefault for BindingWithDefault {
	fn pattern_with_default_kind(&self) -> SyntaxKind {
		JS_BINDING_WITH_DEFAULT
	}

	fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		// TODO
		expected_property_binding(p, range)
	}

	fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		parse_binding(p, self.parameters)
	}
}

// test_err
// let [ default: , hey , ] = []
fn array_binding_pattern(p: &mut Parser, parameters: bool) -> ParsedSyntax {
	ArrayBinding { parameters }.parse_array_pattern(p)
}

struct ArrayBinding {
	parameters: bool,
}

impl ArrayPattern<BindingWithDefault> for ArrayBinding {
	fn unknown_pattern_kind(&self) -> SyntaxKind {
		JS_UNKNOWN_BINDING
	}

	fn array_pattern_kind(&self) -> SyntaxKind {
		JS_ARRAY_BINDING
	}

	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		// TODO
		expected_assignment_target(p, range)
	}

	fn pattern_with_default(&self) -> BindingWithDefault {
		BindingWithDefault {
			parameters: self.parameters,
		}
	}

	// TODO unify?
	fn parse_rest_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T![...]) {
			return Absent;
		}

		let m = p.start();
		p.bump_any();

		// TODO error
		parse_binding(p, self.parameters).or_missing_with_error(p, expected_pattern);

		Present(m.complete(p, JS_ARRAY_REST_BINDING))
	}
}

// test_err object_binding_pattern
// let { 5 } } = { eval: "foo" };
// let { eval } = { eval: "foo" };
// let { 5, 6 } = { eval: "foo" };
// let { default: , bar } = {};
pub fn object_binding_pattern(p: &mut Parser, parameters: bool) -> ParsedSyntax {
	ObjectBindingPattern { parameters }.parse_object_pattern(p)
}

struct ObjectBindingPattern {
	parameters: bool,
}

impl ObjectPattern for ObjectBindingPattern {
	fn unknown_pattern_kind(&self) -> SyntaxKind {
		JS_UNKNOWN_BINDING
	}

	fn object_pattern_kind(&self) -> SyntaxKind {
		JS_OBJECT_BINDING
	}

	fn expected_property_pattern_error(&self, p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_property_binding(p, range)
	}

	// test object_property_binding
	// let { foo: bar  } = {}
	// let { foo: bar = baz } = {}
	//
	// test_err object_property_binding_err
	// let { foo: , bar } = {}
	// let { : bar = "test" } = {}
	// let { , foo: bar } = {}
	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !is_at_object_member_name(p) && !p.at_ts(token_set![T![:], T![=]]) {
			return Absent;
		}

		let m = p.start();
		object_member_name(p).or_missing_with_error(p, expected_object_member_name);
		p.expect_required(T![:]);
		let parameters_argument = self.parameters;
		parse_binding(p, parameters_argument).or_missing_with_error(p, expected_pattern);
		{
			// TODO remove after migrating expression to `ParsedSyntax`
			let mut guard = p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
				..p.state.clone()
			});
			parse_equal_value_clause(&mut *guard).or_missing(&mut *guard);
		}
		Present(m.complete(p, JS_PROPERTY_BINDING))
	}

	// test object_shorthand_property
	// let { a, b } = c
	// let { a = "default", b = call() } = c
	//
	// test_err object_shorthand_property_err
	// let { a b } = c
	// let { = "test" } = c
	// let { , a } = c
	fn parse_shorthand_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		let identifier = parse_identifier_binding(p);
		if p.at(T![=]) || identifier.is_present() {
			let shorthand_prop = identifier.precede_or_missing_with_error(p, expected_identifier);
			{
				// TODO remove after migrating expression to `ParsedSyntax`
				let mut guard = p.with_state(ParserState {
					expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
					..p.state.clone()
				});
				parse_equal_value_clause(&mut *guard).or_missing(&mut *guard);
			}
			Present(shorthand_prop.complete(p, JS_SHORTHAND_PROPERTY_BINDING))
		} else {
			Absent
		}
	}

	// test rest_property_binding
	// let { ...abcd } = a;
	// let { b: { ...a } } = c;
	//
	// test_err rest_property_binding_err
	// let { ... } = a;
	// let { ...c = "default" } = a;
	// let { ...{a} } = b;
	// let { ...rest, other_assignment } = a;
	// let { ...rest, } = a;
	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if p.at(T![...]) {
			let m = p.start();
			p.bump(T![...]);

			let inner = parse_binding(p, self.parameters);

			if let Present(mut inner) = inner {
				if inner.kind() != JS_IDENTIFIER_BINDING {
					inner.change_kind(p, JS_UNKNOWN_BINDING);
					p.error(p.err_builder("Expected identifier binding").primary(inner.range(p), "Object rest patterns must bind to an identifier, other patterns are not allowed."))
				}
			}

			Present(m.complete(p, JS_OBJECT_REST_BINDING))
		} else {
			Absent
		}
	}
}
