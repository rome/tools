use crate::parser::{ParsedSyntax, ParserProgress};
use crate::state::{EnterFunction, EnterParameters, SignatureFlags};
use crate::syntax::binding::{parse_binding, parse_binding_pattern};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::parse_expr_or_assignment;
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::expected_binding;
use crate::syntax::stmt::{is_semi, parse_block_impl, StatementContext};
use crate::syntax::typescript::{
    maybe_eat_incorrect_modifier, maybe_ts_type_annotation, ts_type, ts_type_or_type_predicate_ann,
    ts_type_params,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, JsSyntaxFeature, Marker, ParseRecovery, Parser, SyntaxFeature};
use rome_rowan::SyntaxKind;
use rslint_syntax::JsSyntaxKind::*;
use rslint_syntax::{JsSyntaxKind, T};

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test function_decl
// function foo1() {}
// function *foo2() {}
// async function *foo3() {}
// async function foo4() {}
// function *foo5() {
//   yield foo;
// }
//
// test function_declaration_script
// // SCRIPT
// function test(await) {}
//
// test_err function_decl_err
// function() {}
// function foo {}
// function {}
// function *() {}
// async function() {}
// async function *() {}
// function *foo2() {}
// yield foo3;
// function test2(): number {}
// function foo4(await) {}
// function foo5(yield) {}
//
// test_err function_broken
// function foo())})}{{{  {}
pub(super) fn parse_function_statement(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    let mut function = parse_function(
        p,
        m,
        FunctionKind::Statement {
            declaration: context.is_single_statement(),
        },
    );

    if context != StatementContext::StatementList && !function.kind().is_unknown() {
        if JsSyntaxFeature::StrictMode.is_supported(p) {
            // test_err function_in_single_statement_context_strict
            // if (true) function a() {}
            // label1: function b() {}
            // while (true) function c() {}
            p.error(p.err_builder("In strict mode code, functions can only be declared at top level or inside a block").primary(function.range(p), "wrap the function in a block statement"));
            function.change_to_unknown(p);
        } else if !matches!(context, StatementContext::If | StatementContext::Label) {
            // test function_in_if_or_labelled_stmt_loose_mode
            // // SCRIPT
            // label1: function a() {}
            // if (true) function b() {} else function c() {}
            // if (true) function d() {}
            // if (true) "test"; else function e() {}
            p.error(p.err_builder("In non-strict mode code, functions can only be declared at top level, inside a block, or as the body of an if or labelled statement").primary(function.range(p), "wrap the function in a block statement"));
            function.change_to_unknown(p);
        }
    }

    Present(function)
}

pub(super) fn parse_function_expression(p: &mut Parser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    Present(parse_function(p, m, FunctionKind::Expression))
}

// test export_function_clause
// export function test(a, b) {}
// export function* test2(a, b) {}
// export async function test3(a, b, ) {}
pub(super) fn parse_export_function_clause(p: &mut Parser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    Present(parse_function(p, m, FunctionKind::Export))
}

// test export_default_function_clause
// export default function test(a, b) {}
pub(super) fn parse_export_default_function_case(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T![default]) || p.nth_at(1, T![function]) || p.nth_src(1) == "async") {
        return Absent;
    }

    let m = p.start();
    p.bump(T![default]);
    Present(parse_function(p, m, FunctionKind::ExportDefault))
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
    Statement { declaration: bool },
    Expression,
    Export,
    ExportDefault,
}

impl FunctionKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, FunctionKind::Expression | FunctionKind::ExportDefault)
    }

    fn is_statement(&self) -> bool {
        matches!(self, FunctionKind::Statement { .. })
    }
}

impl From<FunctionKind> for JsSyntaxKind {
    fn from(kind: FunctionKind) -> Self {
        match kind {
            FunctionKind::Statement { .. } => JS_FUNCTION_STATEMENT,
            FunctionKind::Expression => JS_FUNCTION_EXPRESSION,
            FunctionKind::Export => JS_EXPORT_FUNCTION_CLAUSE,
            FunctionKind::ExportDefault => JS_EXPORT_DEFAULT_FUNCTION_CLAUSE,
        }
    }
}

fn is_at_function(p: &Parser) -> bool {
    p.at_ts(token_set![T![async], T![function]]) || is_at_async_function(p, LineBreak::DoNotCheck)
}

fn parse_function(p: &mut Parser, m: Marker, kind: FunctionKind) -> CompletedMarker {
    let mut uses_invalid_syntax =
        kind.is_statement() && p.eat(T![declare]) && TypeScript.is_unsupported(p);
    let mut flags = SignatureFlags::empty();

    let in_async = is_at_async_function(p, LineBreak::DoNotCheck);
    if in_async {
        p.bump_remap(T![async]);
        flags |= SignatureFlags::ASYNC;
    }

    p.expect(T![function]);

    let in_generator = p.eat(T![*]);
    if in_generator {
        flags |= SignatureFlags::GENERATOR;
    }

    let id = parse_function_id(p, kind, flags);

    if !kind.is_id_optional() {
        id.or_add_diagnostic(p, |p, range| {
            p.err_builder(
                "expected a name for the function in a function declaration, but found none",
            )
            .primary(range, "")
        });
    }

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_parameter_types, |p, marker| {
            p.err_builder("type parameters can only be used in TypeScript files")
                .primary(marker.range(p), "")
        })
        .ok();

    parse_parameter_list(p, flags).or_add_diagnostic(p, js_parse_error::expected_parameters);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_annotation_or_error, |p, marker| {
            p.err_builder("return types can only be used in TypeScript files")
                .primary(marker.range(p), "")
        })
        .ok();

    if kind.is_statement() {
        function_body_or_declaration(p, flags);
    } else {
        parse_function_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_function_body);
    }

    let mut function = m.complete(p, kind.into());

    // test_err async_or_generator_in_single_statement_context
    // if (true) async function t() {}
    // if (true) function* t() {}
    if kind == (FunctionKind::Statement { declaration: true }) && (in_async || in_generator) {
        p.error(p.err_builder("`async` and generator functions can only be declared at top level or inside a block").primary(function.range(p), ""));
        uses_invalid_syntax = true;
    }

    if uses_invalid_syntax {
        function.change_to_unknown(p);
    }

    function
}

// test_err break_in_nested_function
// while (true) {
//   function helper() {
//     break;
//   }
// }
pub(super) fn parse_function_body(p: &mut Parser, flags: SignatureFlags) -> ParsedSyntax {
    p.with_state(EnterFunction(flags), |p| {
        parse_block_impl(p, JS_FUNCTION_BODY)
    })
}

fn parse_function_id(p: &mut Parser, kind: FunctionKind, flags: SignatureFlags) -> ParsedSyntax {
    match kind {
        // Takes the async and generator restriction from the expression
        FunctionKind::Expression => {
            // test function_expression_id
            // // SCRIPT
            // (function await() {});
            // (function yield() {});
            // (async function yield() {});
            // (function* await() {})
            //
            // test_err function_expression_id_err
            // (async function await() {});
            // (function* yield() {});
            // function* test() { function yield() {} }
            p.with_state(EnterFunction(flags), parse_binding)
        }
        // Inherits the async and generator from the parent
        _ => {
            // test function_id
            // // SCRIPT
            // function test() {}
            // function await(test) {}
            // async function await(test) {}
            // function yield(test) {}
            // function* yield(test) {}
            //
            //
            // test_err function_id_err
            // function* test() {
            //   function yield(test) {}
            // }
            parse_binding(p)
        }
    }
}

// TODO 1725 This is probably not ideal (same with the `declare` keyword). We should
// use a different AST type for function declarations. For example, a function declaration should
// never have a body but that would be allowed with this approach. Same for interfaces, interface
// methods should never have a body
/// Either parses a typescript declaration body or the function body
pub(super) fn function_body_or_declaration(p: &mut Parser, flags: SignatureFlags) {
    // omitting the body is allowed in ts
    if p.typescript() && !p.at(T!['{']) && is_semi(p, 0) {
        p.eat(T![;]);
    } else {
        let body = parse_function_body(p, flags);
        body.or_add_diagnostic(p, js_parse_error::expected_function_body);
    }
}

pub(crate) fn parse_ts_parameter_types(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![<]) {
        Present(ts_type_params(p).unwrap())
    } else {
        Absent
    }
}

pub(crate) fn ts_parameter_types(p: &mut Parser) {
    if p.at(T![<]) {
        if let Some(ref mut ty) = ts_type_params(p) {
            ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
        }
    }
}

pub(crate) fn parse_ts_type_annotation_or_error(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![:]) {
        let return_type = p.start();
        if let Some(ref mut ty) = ts_type_or_type_predicate_ann(p, T![:]) {
            ty.err_if_not_ts(p, "return types can only be used in TypeScript files");
        }
        Present(return_type.complete(p, TS_TYPE_ANNOTATION))
    } else {
        Absent
    }
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq)]
#[repr(u8)]
pub(super) enum LineBreak {
    // check line breaks
    DoCheck,
    // do not check line break
    DoNotCheck,
}

#[inline]
/// Checks if the parser is inside a "async function"
pub(super) fn is_at_async_function(p: &Parser, should_check_line_break: LineBreak) -> bool {
    let async_function_tokens = p.cur_src() == "async" && p.nth_at(1, T![function]);
    if should_check_line_break == LineBreak::DoCheck {
        async_function_tokens && !p.has_linebreak_before_n(1)
    } else {
        async_function_tokens
    }
}

pub(super) fn parse_arrow_function_parameters(
    p: &mut Parser,
    mut flags: SignatureFlags,
) -> ParsedSyntax {
    if p.state.in_generator() {
        // Arrow functions inherit whatever yield is a valid identifier name from the parent.
        flags |= SignatureFlags::GENERATOR;
    }

    // The arrow function is in an async context if the outer function is in an async context or itself is
    // declared async
    if !flags.contains(SignatureFlags::ASYNC) && p.state.in_async() {
        flags |= SignatureFlags::ASYNC;
    }

    if p.at(T!['(']) {
        parse_parameter_list(p, flags)
    } else {
        // test_err async_arrow_expr_await_parameter
        // let a = async await => {}
        // async() => { (a = await) => {} };
        p.with_state(
            EnterParameters {
                signature_flags: flags,
                allow_object_expressions: false,
            },
            parse_binding,
        )
    }
}

pub(super) fn parse_arrow_body(p: &mut Parser, mut flags: SignatureFlags) -> ParsedSyntax {
    // test arrow_in_constructor
    // class A {
    //   constructor() {
    //     () => { super() };
    //     () => super();
    //  }
    // }
    if p.state.in_constructor() {
        flags |= SignatureFlags::CONSTRUCTOR
    }

    if p.at(T!['{']) {
        parse_function_body(p, flags)
    } else {
        p.with_state(EnterFunction(flags), parse_expr_or_assignment)
    }
}

pub(super) fn parse_parameter(p: &mut Parser) -> ParsedSyntax {
    if p.typescript() {
        if let Some(modifier) = maybe_eat_incorrect_modifier(p) {
            let err = p
                .err_builder("modifiers on parameters are only allowed in constructors")
                .primary(modifier.range(p), "");

            p.error(err);
        }
    }

    parse_binding_pattern(p).map(|binding| {
        let m = binding.precede(p);
        maybe_ts_type_annotation(p);
        parse_initializer_clause(p).ok();
        m.complete(p, JS_PARAMETER)
    })
}

// test parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(p: &mut Parser, flags: SignatureFlags) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    parse_parameters_list(p, flags, parse_parameter, JS_PARAMETER_LIST);

    Present(m.complete(p, JS_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
    p: &mut Parser,
    flags: SignatureFlags,
    parse_parameter: impl Fn(&mut Parser) -> ParsedSyntax,
    list_kind: JsSyntaxKind,
) {
    let mut first = true;
    let has_l_paren = p.expect(T!['(']);

    p.with_state(
        EnterParameters {
            signature_flags: flags,
            allow_object_expressions: has_l_paren,
        },
        |p| {
            let parameters_list = p.start();
            let mut progress = ParserProgress::default();

            while !p.at(EOF) && !p.at(T![')']) {
                progress.assert_progressing(p);

                if first {
                    first = false;
                } else {
                    p.expect(T![,]);
                }

                if p.at(T![')']) {
                    break;
                }

                if p.at(T![...]) {
                    let m = p.start();
                    p.bump_any();
                    parse_binding_pattern(p).or_add_diagnostic(p, expected_binding);

                    // TODO #1725 Review error handling and recovery
                    // rest patterns cannot be optional: `...foo?: number[]`
                    if p.at(T![?]) {
                        let err = p
                            .err_builder("rest patterns cannot be optional")
                            .primary(p.cur_tok().range(), "");

                        p.error(err);
                        let m = p.start();
                        p.bump_any();
                        m.complete(p, JS_UNKNOWN_PARAMETER);
                    }

                    // type annotation `...foo: number[]`
                    if p.eat(T![:]) {
                        let complete = ts_type(p);
                        if let Some(mut res) = complete {
                            res.err_if_not_ts(
                                p,
                                "type annotations can only be used in TypeScript files",
                            );
                        }
                    }

                    if p.at(T![=]) {
                        let start = p.cur_tok().start();
                        let m = p.start();
                        p.bump_any();

                        let end = parse_expr_or_assignment(&mut *p)
                            .ok()
                            .map(|marker| usize::from(marker.range(p).end()))
                            .unwrap_or_else(|| p.cur_tok().start());

                        let err = p
                            .err_builder("rest elements may not have default initializers")
                            .primary(start..end, "");

                        p.error(err);
                        m.complete(p, JS_UNKNOWN);
                    }

                    m.complete(p, JS_REST_PARAMETER);

                    // FIXME: this should be handled better, we should keep trying to parse params but issue an error for each one
                    // which would allow for better recovery from `foo, ...bar, foo`
                    if p.at(T![,]) {
                        let m = p.start();
                        let range = p.cur_tok().range();
                        p.bump_any();
                        m.complete(p, JS_UNKNOWN);
                        let err = p
                            .err_builder("rest elements may not have trailing commas")
                            .primary(range, "");

                        p.error(err);
                    }
                } else {
                    // test_err formal_params_no_binding_element
                    // function foo(true) {}

                    // test_err formal_params_invalid
                    // function (a++, c) {}
                    let recovered_result = parse_parameter(p).or_recover(
                        p,
                        &ParseRecovery::new(
                            JS_UNKNOWN_PARAMETER,
                            token_set![
                                T![ident],
                                T![await],
                                T![yield],
                                T![,],
                                T!['['],
                                T![...],
                                T![')'],
                            ],
                        )
                        .enable_recovery_on_line_break(),
                        js_parse_error::expected_parameter,
                    );

                    if recovered_result.is_err() {
                        break;
                    }
                }
            }

            parameters_list.complete(p, list_kind);
        },
    );

    p.expect(T![')']);
}
