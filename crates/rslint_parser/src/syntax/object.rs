#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::parser::{ParsedSyntax, RecoveryResult};
use crate::state::{EnterParameters, SignatureFlags};
use crate::syntax::expr::{
    is_nth_at_reference_identifier, parse_assignment_expression_or_higher, parse_expression,
    parse_reference_identifier, ExpressionContext,
};
use crate::syntax::function::{
    parse_formal_parameter, parse_function_body, parse_parameter_list, ParameterContext,
};
use crate::syntax::js_parse_error;
use crate::syntax::typescript::ts_parse_error::{
    ts_accessor_type_parameters_error, ts_only_syntax_error, ts_set_accessor_return_type_error,
};
use crate::syntax::typescript::{
    parse_ts_return_type_annotation, parse_ts_type_annotation, parse_ts_type_parameters,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::{ParseRecovery, ParseSeparatedList, Parser, SyntaxFeature};
use rome_js_syntax::JsSyntaxKind::*;
use rome_js_syntax::{JsSyntaxKind, T};

// test object_expr
// let a = {};
// let b = {foo,}
//
// test_err object_expr_err
// let a = {, foo}
// let b = { foo bar }
// let b = { foo

struct ObjectMembersList;

impl ParseSeparatedList for ObjectMembersList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_object_member(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN_MEMBER, token_set![T![,], T!['}'], T![;], T![:]])
                .enable_recovery_on_line_break(),
            js_parse_error::expected_object_member,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_OBJECT_MEMBER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An object literal such as `{ a: b, "b": 5 + 5 }`.
pub(super) fn parse_object_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);

    ObjectMembersList.parse_list(p);

    p.expect(T!['}']);
    Present(m.complete(p, JS_OBJECT_EXPRESSION))
}

/// An individual object property such as `"a": b` or `5: 6 + 6`.
fn parse_object_member(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        // test getter_object_member
        // let a = {
        //   get foo() {
        //     return foo;
        //   },
        //   get "bar"() {
        //     return "bar";
        //   },
        //   get ["a" + "b"]() {
        //     return "a" + "b"
        //   },
        //   get 5() {
        //     return 5;
        //   },
        //   get() {
        //    return "This is a method and not a getter";
        //   }
        // }
        T![get] if !p.has_nth_preceding_line_break(1) && is_nth_at_type_member_name(p, 1) => {
            parse_getter_object_member(p)
        }

        // test setter_object_member
        // let a = {
        //  set foo(value) {
        //  },
        //  set "bar"(value) {
        //  },
        //  set ["a" + "b"](value) {
        //  },
        //  set 5(value) {
        //  },
        //  set() {
        //   return "This is a method and not a setter";
        //  }
        // }

        // test_err object_expr_setter
        // let b = {
        //  set foo() {
        //     return 5;
        //  }
        // }
        T![set] if !p.has_nth_preceding_line_break(1) && is_nth_at_type_member_name(p, 1) => {
            parse_setter_object_member(p)
        }

        // test object_expr_async_method
        // let a = {
        //   async foo() {},
        //   async *foo() {}
        // }
        T![async] if is_parser_at_async_method_member(p) => parse_method_object_member(p),

        // test object_expr_spread_prop
        // let a = {...foo}
        T![...] => {
            let m = p.start();
            p.bump_any();
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
            Present(m.complete(p, JS_SPREAD))
        }

        T![*] => {
            // test object_expr_generator_method
            // let b = { *foo() {} }
            parse_method_object_member(p)
        }

        _ => {
            let m = p.start();

            if is_nth_at_reference_identifier(p, 0)
                && !token_set![T!['('], T![<], T![:]].contains(p.nth(1))
            {
                // test object_expr_ident_prop
                // ({foo})
                parse_reference_identifier(p).unwrap();

                // There are multiple places where it's first needed to parse an expression to determine if
                // it is an assignment target or not. This requires that parse expression is valid for any
                // assignment expression. Thus, it's needed that the parser silently parses over a "{ arrow = test }"
                // property
                if p.at(T![=]) {
                    // test assignment_shorthand_prop_with_initializer
                    // for ({ arrow = () => {} } of [{}]) {}
                    //
                    // test_err object_shorthand_with_initializer
                    // ({ arrow = () => {} })
                    p.error(p.err_builder("Did you mean to use a `:`? An `=` can only follow a property name when the containing object literal is part of a destructuring pattern.")
						.primary(p.cur_range(), ""));
                    p.bump(T![=]);
                    parse_assignment_expression_or_higher(p, ExpressionContext::default()).ok();
                    return Present(m.complete(p, JS_UNKNOWN_MEMBER));
                }

                return Present(m.complete(p, JS_SHORTHAND_PROPERTY_OBJECT_MEMBER));
            }

            let checkpoint = p.checkpoint();
            let member_name = parse_object_member_name(p)
                .or_add_diagnostic(p, js_parse_error::expected_object_member);

            // test object_expr_method
            // let b = {
            //   foo() {},
            //   "bar"(a, b, c) {},
            //   ["foo" + "bar"](a) {},
            //   5(...rest) {}
            // }

            // test_err object_expr_method
            // let b = { foo) }
            if p.at(T!['(']) || p.at(T![<]) {
                parse_method_object_member_body(p, SignatureFlags::empty());
                Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
            } else if member_name.is_some() {
                // test object_prop_name
                // let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}

                // test object_expr_ident_literal_prop
                // let b = { a: true }

                // If the member name was a literal OR we're at a colon
                p.expect(T![:]);

                // test object_prop_in_rhs
                // for ({ a: "x" in {} };;) {}
                parse_assignment_expression_or_higher(p, ExpressionContext::default())
                    .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
                Present(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
            } else {
                // test_err object_expr_error_prop_name
                // let a = { /: 6, /: /foo/ }
                // let b = {{}}

                // test_err object_expr_non_ident_literal_prop
                // let d = {5}

                #[allow(deprecated)]
                SingleTokenParseRecovery::new(token_set![T![:], T![,]], JS_UNKNOWN).recover(p);

                if p.eat(T![:]) {
                    parse_assignment_expression_or_higher(p, ExpressionContext::default())
                        .or_add_diagnostic(p, js_parse_error::expected_object_member);
                    Present(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
                } else {
                    // It turns out that this isn't a valid member after all. Make sure to throw
                    // away everything that has been parsed so far so that the caller can
                    // do its error recovery
                    p.rewind(checkpoint);
                    m.abandon(p);
                    Absent
                }
            }
        }
    }
}

/// Parses a getter object member: `{ get a() { return "a"; } }`
fn parse_getter_object_member(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![get]) {
        return Absent;
    }

    let m = p.start();

    p.expect(T![get]);

    parse_object_member_name(p).or_add_diagnostic(p, js_parse_error::expected_object_member_name);

    // test_err ts ts_object_getter_type_parameters
    // ({ get a<A>(): A {} });
    if let Present(type_parameters) = parse_ts_type_parameters(p) {
        p.error(ts_accessor_type_parameters_error(p, &type_parameters))
    }

    p.expect(T!['(']);
    p.expect(T![')']);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "type annotation", annotation.range(p))
        })
        .ok();

    parse_function_body(p, SignatureFlags::empty())
        .or_add_diagnostic(p, js_parse_error::expected_function_body);

    Present(m.complete(p, JS_GETTER_OBJECT_MEMBER))
}

/// Parses a setter object member like `{ set a(value) { .. } }`
fn parse_setter_object_member(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![set]) {
        return Absent;
    }
    let m = p.start();

    p.expect(T![set]);

    parse_object_member_name(p).or_add_diagnostic(p, js_parse_error::expected_object_member_name);

    // test_err ts ts_object_setter_type_parameters
    // ({ set a<A>(value: A) {} });
    if let Present(type_parameters) = parse_ts_type_parameters(p) {
        p.error(ts_accessor_type_parameters_error(p, &type_parameters))
    }

    let has_l_paren = p.expect(T!['(']);

    p.with_state(EnterParameters(SignatureFlags::empty()), |p| {
        parse_formal_parameter(
            p,
            ParameterContext::Setter,
            ExpressionContext::default().and_object_expression_allowed(has_l_paren),
        )
        .or_add_diagnostic(p, js_parse_error::expected_parameter);
        p.expect(T![')']);
    });

    // test_err ts ts_object_setter_return_type
    // ({ set a(value: string): void {} });
    if let Present(return_type_annotation) = parse_ts_return_type_annotation(p) {
        p.error(ts_set_accessor_return_type_error(
            p,
            &return_type_annotation,
        ));
    }

    parse_function_body(p, SignatureFlags::empty())
        .or_add_diagnostic(p, js_parse_error::expected_function_body);

    Present(m.complete(p, JS_SETTER_OBJECT_MEMBER))
}

// test object_member_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
/// Parses a `JsAnyObjectMemberName` and returns its completion marker
pub(crate) fn parse_object_member_name(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        T!['['] => parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    }
}

pub(crate) fn is_nth_at_type_member_name(p: &mut Parser, offset: usize) -> bool {
    let nth = p.nth(offset);

    let start_names = token_set![
        JS_STRING_LITERAL,
        JS_NUMBER_LITERAL,
        T![ident],
        T![await],
        T![yield],
        T!['[']
    ];

    nth.is_keyword() || start_names.contains(nth)
}

pub(crate) fn is_at_object_member_name(p: &mut Parser) -> bool {
    is_nth_at_type_member_name(p, 0)
}

pub(crate) fn parse_computed_member_name(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['[']);

    // test computed_member_name_in
    // for ({["x" in {}]: 3} ;;) {}
    parse_expression(p, ExpressionContext::default())
        .or_add_diagnostic(p, js_parse_error::expected_expression);

    p.expect(T![']']);
    Present(m.complete(p, JS_COMPUTED_MEMBER_NAME))
}

pub(super) fn is_at_literal_member_name(p: &mut Parser, offset: usize) -> bool {
    matches!(
        p.nth(offset),
        JS_STRING_LITERAL | JS_NUMBER_LITERAL | T![ident]
    ) || p.nth(offset).is_keyword()
}

pub(super) fn parse_literal_member_name(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        JS_STRING_LITERAL | JS_NUMBER_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    }
    Present(m.complete(p, JS_LITERAL_MEMBER_NAME))
}

/// Parses a method object member
fn parse_method_object_member(p: &mut Parser) -> ParsedSyntax {
    let is_async = is_parser_at_async_method_member(p);
    if !is_async && !p.at(T![*]) && !is_at_object_member_name(p) {
        return Absent;
    }

    let m = p.start();
    let mut flags = SignatureFlags::empty();

    // test async_method
    // class foo {
    //  async foo() {}
    //  async *foo() {}
    // }
    if is_async {
        p.eat(T![async]);
        flags |= SignatureFlags::ASYNC;
    }

    if p.eat(T![*]) {
        flags |= SignatureFlags::GENERATOR;
    }

    parse_object_member_name(p).or_add_diagnostic(p, js_parse_error::expected_object_member_name);

    parse_method_object_member_body(p, flags);

    Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
}

// test ts ts_method_object_member_body
// ({
//     x<A>(maybeA: any): maybeA is A { return true },
//     y(a: string): string { return "string"; },
//     async *id<R>(param: Promise<R>): AsyncIterableIterator<R> { yield await param },
// })

/// Parses the body of a method object member starting right after the member name.
fn parse_method_object_member_body(p: &mut Parser, flags: SignatureFlags) {
    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_parameters, |p, type_parameters| {
            ts_only_syntax_error(p, "type parameters", type_parameters.range(p))
        })
        .ok();

    parse_parameter_list(p, ParameterContext::Implementation, flags)
        .or_add_diagnostic(p, js_parse_error::expected_parameters);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "return type annotation", annotation.range(p))
        })
        .ok();

    parse_function_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_function_body);
}

fn is_parser_at_async_method_member(p: &mut Parser) -> bool {
    p.at(T![async])
        && !p.has_nth_preceding_line_break(1)
        && (is_nth_at_type_member_name(p, 1) || p.nth_at(1, T![*]))
}
