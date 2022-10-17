use crate::parser::expected_any;
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{is_nth_at_identifier, parse_identifier, ExpressionContext};
use crate::syntax::js_parse_error::{
    expected_binding, expected_identifier, expected_object_member_name,
};
use crate::syntax::object::{is_at_object_member_name, parse_object_member_name};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{ParseDiagnostic, ParsedSyntax, Parser, SyntaxFeature, ToDiagnostic};
use rome_diagnostics::Span;
use rome_js_syntax::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind as SyntaxKindTrait;

pub(crate) fn parse_binding_pattern(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    match p.cur() {
        T!['['] => ArrayBindingPattern.parse_array_pattern(p),
        T!['{'] if context.is_object_expression_allowed() => {
            ObjectBindingPattern.parse_object_pattern(p)
        }
        _ => parse_identifier_binding(p),
    }
}

#[inline]
pub(crate) fn is_at_identifier_binding(p: &mut Parser) -> bool {
    is_nth_at_identifier_binding(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier_binding(p: &mut Parser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
}

#[inline]
pub(crate) fn parse_binding(p: &mut Parser) -> ParsedSyntax {
    parse_identifier_binding(p)
}

// test_err binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
// let let = 5;
// const let = 5;
// let a, a;
//
// test_err binding_identifier_invalid_script
// // SCRIPT
// let let = 5;
// const let = 5;
/// Parses an identifier binding or returns an invalid syntax if the identifier isn't valid in this context.
/// An identifier may not be valid if:
/// * it is named "eval" or "arguments" inside of strict mode
/// * it is named "let" inside of a "let" or "const" declaration
/// * the same identifier is bound multiple times inside of a `let` or const` declaration
/// * it is named "yield" inside of a generator function or in strict mode
/// * it is named "await" inside of an async function
pub(crate) fn parse_identifier_binding(p: &mut Parser) -> ParsedSyntax {
    let parsed = parse_identifier(p, JS_IDENTIFIER_BINDING);

    parsed.map(|mut identifier| {
        if identifier.kind().is_unknown() {
            return identifier;
        }

        let identifier_name = identifier.text(p);

        if StrictMode.is_supported(p) && matches!(identifier_name, "eval" | "arguments") {
            let err = p.err_builder(
                format!(
                    "Illegal use of `{}` as an identifier in strict mode",
                    identifier_name
                ),
                identifier.range(p),
            );
            p.error(err);

            identifier.change_to_unknown(p);
            return identifier;
        }

        if let Some(parent) = p.state.duplicate_binding_parent {
            if identifier_name == "let" {
                let err = p
                    .err_builder(
                        format!(
                        "`let` cannot be declared as a variable name inside of a `{}` declaration",
                        parent,

                    ),
                        identifier.range(p),
                    )
                    .hint("Rename the let identifier here");

                p.error(err);
                identifier.change_to_unknown(p);
                return identifier;
            }

            if let Some(existing) = p.state.name_map.get(identifier_name) {
                let err = p
                    .err_builder(
                        format!(
                            "Declarations inside of a `{}` declaration may not have duplicates",
                            parent
                        ),
                        identifier.range(p),
                    )
                    .detail(
                        identifier.range(p),
                        format!(
                            "a second declaration of `{}` is not allowed",
                            identifier_name
                        ),
                    )
                    .detail(
                        existing.to_owned(),
                        format!("`{}` is first declared here", identifier_name),
                    );
                p.error(err);
                identifier.change_to_unknown(p);
                return identifier;
            }

            let identifier_name = String::from(identifier_name);
            p.state
                .name_map
                .insert(identifier_name, identifier.range(p).as_range());
        }

        identifier
    })
}

struct BindingPatternWithDefault;

impl ParseWithDefaultPattern for BindingPatternWithDefault {
    #[inline]
    fn pattern_with_default_kind() -> JsSyntaxKind {
        JS_BINDING_PATTERN_WITH_DEFAULT
    }

    #[inline]
    fn expected_pattern_error(p: &Parser, range: TextRange) -> ParseDiagnostic {
        expected_binding(p, range)
    }

    #[inline]
    fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        parse_binding_pattern(p, ExpressionContext::default())
    }
}

struct ArrayBindingPattern;

// test array_binding
// let a = "b";
// let [c, b] = [1, 2];
// let [d, ...abcd] = [1];
// let [e = "default", x] = []
// let [, f, ...rest] = []
// let [[...rest2], { g }] = []
//
// test_err array_binding_err
// let [a b] = [1, 2];
// let [="default"] = [1, 2];
// let ["default"] = [1, 2];
// let [[c ] = [];
//
// test array_binding_rest
// let [ ...abcd ] = a;
// let [ ...[x, y] ] = b;
// let [ ...[ ...a ] ] = c;
//
// test_err array_binding_rest_err
// let [ ... ] = a;
// let [ ...c = "default" ] = a;
// let [ ...rest, other_assignment ] = a;
impl ParseArrayPattern<BindingPatternWithDefault> for ArrayBindingPattern {
    #[inline]
    fn unknown_pattern_kind() -> JsSyntaxKind {
        JS_UNKNOWN_BINDING
    }

    #[inline]
    fn array_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN
    }

    #[inline]
    fn rest_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
    }

    fn list_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
    }

    #[inline]
    fn expected_element_error(p: &Parser, range: TextRange) -> ParseDiagnostic {
        expected_any(
            &[
                "identifier",
                "object pattern",
                "array pattern",
                "rest pattern",
            ],
            range,
        )
        .to_diagnostic(p)
    }

    #[inline]
    fn pattern_with_default(&self) -> BindingPatternWithDefault {
        BindingPatternWithDefault
    }
}

// test_err object_binding_pattern
// let { 5 } } = { eval: "foo" };
// let { eval } = { eval: "foo" };
// let { 5, 6 } = { eval: "foo" };
// let { default , eval: } = {};
struct ObjectBindingPattern;

impl ParseObjectPattern for ObjectBindingPattern {
    #[inline]
    fn unknown_pattern_kind() -> JsSyntaxKind {
        JS_UNKNOWN_BINDING
    }

    #[inline]
    fn object_pattern_kind() -> JsSyntaxKind {
        JS_OBJECT_BINDING_PATTERN
    }

    fn list_kind() -> JsSyntaxKind {
        JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
    }

    #[inline]
    fn expected_property_pattern_error(p: &Parser, range: TextRange) -> ParseDiagnostic {
        expected_any(&["identifier", "member name", "rest pattern"], range).to_diagnostic(p)
    }

    // test object_property_binding
    // let { foo: bar  } = {}
    // let { foo: bar_bar = baz } = {}
    //
    // test_err object_property_binding_err
    // let { foo: , bar } = {}
    // let { : lorem = "test" } = {}
    // let { , ipsum: bazz } = {}
    //
    // test object_shorthand_property
    // let { a, b } = c
    // let { d = "default", e = call() } = c
    //
    // test_err object_shorthand_property_err
    // let { a b } = c
    // let { = "test" } = c
    // let { , d } = c
    fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        if !is_at_object_member_name(p) && !p.at_ts(token_set![T![:], T![=]]) {
            return Absent;
        }

        let m = p.start();

        let kind = if p.at(T![=]) || (is_at_identifier_binding(p) && !p.nth_at(1, T![:])) {
            parse_binding(p).or_add_diagnostic(p, expected_identifier);
            JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
        } else {
            parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
            if p.expect(T![:]) {
                parse_binding_pattern(p, ExpressionContext::default())
                    .or_add_diagnostic(p, expected_binding);
            }
            JS_OBJECT_BINDING_PATTERN_PROPERTY
        };

        parse_initializer_clause(p, ExpressionContext::default()).ok();

        Present(m.complete(p, kind))
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
    // let { ...rest2, } = a;
    // async function test() {
    //   let { ...await } = a;
    // }
    fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        if p.at(T![...]) {
            let m = p.start();
            p.bump(T![...]);

            let inner = parse_binding_pattern(p, ExpressionContext::default())
                .or_add_diagnostic(p, expected_identifier);

            if let Some(mut inner) = inner {
                if inner.kind() != JS_IDENTIFIER_BINDING {
                    let inner_range = inner.range(p);
                    // Don't add multiple errors
                    if inner.kind() != JS_UNKNOWN_BINDING {
                        p.error(p.err_builder("Expected identifier binding", inner_range,).hint( "Object rest patterns must bind to an identifier, other patterns are not allowed."));
                    }

                    inner.change_kind(p, JS_UNKNOWN_BINDING);
                }
            }

            Present(m.complete(p, JS_OBJECT_BINDING_PATTERN_REST))
        } else {
            Absent
        }
    }
}
