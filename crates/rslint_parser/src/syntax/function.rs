use crate::parser::{ParsedSyntax, ParserProgress};
use crate::state::{EnterFunction, EnterParameters, SignatureFlags};
use crate::syntax::binding::{
    is_at_identifier_binding, is_nth_at_identifier_binding, parse_binding, parse_binding_pattern,
};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{
    is_nth_at_identifier, parse_assignment_expression_or_higher, ExpressionContext,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    expected_binding, expected_parameter, expected_parameters, ts_only_syntax_error,
};
use crate::syntax::stmt::{is_semi, parse_block_impl, semi, StatementContext};
use crate::syntax::typescript::{
    parse_ts_return_type_annotation, parse_ts_type_annotation, parse_ts_type_parameters, try_parse,
};
use crate::syntax::util::{
    eat_contextual_keyword, expect_contextual_keyword, is_at_contextual_keyword,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, JsSyntaxFeature, Marker, ParseRecovery, Parser, SyntaxFeature};
use rome_rowan::SyntaxKind;
use rslint_errors::Span;
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
//
// test ts ts_function_statement
// function test(a: string, b?: number, c="default") {}
// function test2<A, B extends A, C = A>(a: A, b: B, c: C) {}
pub(super) fn parse_function_declaration(
    p: &mut Parser,
    context: StatementContext,
) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    let mut function = if p.state.in_ambient_context() {
        parse_ambient_function(p, m)
    } else {
        parse_function(
            p,
            m,
            FunctionKind::Declaration {
                single_statement_context: context.is_single_statement(),
            },
        )
    };

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

// test export_default_function_clause
// export default function test(a, b) {}
pub(super) fn parse_export_default_function_case(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T![default]) || p.nth_at(1, T![function]) || p.nth_src(1) == "async") {
        return Absent;
    }

    let m = p.start();
    p.bump(T![default]);

    Present(if p.state.in_ambient_context() {
        parse_ambient_function(p, m)
    } else {
        parse_function(p, m, FunctionKind::ExportDefault)
    })
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
    Declaration {
        // https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-functiondeclarations-in-ifstatement-statement-clauses
        single_statement_context: bool,
    },
    Expression,
    ExportDefault,
}

impl FunctionKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, FunctionKind::Expression | FunctionKind::ExportDefault)
    }

    fn is_expression(&self) -> bool {
        matches!(self, FunctionKind::Expression)
    }

    fn is_in_single_statement_context(&self) -> bool {
        matches!(
            self,
            FunctionKind::Declaration {
                single_statement_context: true
            }
        )
    }
}

impl From<FunctionKind> for JsSyntaxKind {
    fn from(kind: FunctionKind) -> Self {
        match kind {
            FunctionKind::Declaration { .. } => JS_FUNCTION_DECLARATION,
            FunctionKind::Expression => JS_FUNCTION_EXPRESSION,
            FunctionKind::ExportDefault => JS_EXPORT_DEFAULT_FUNCTION_CLAUSE,
        }
    }
}

fn is_at_function(p: &Parser) -> bool {
    p.at_ts(token_set![T![async], T![function]]) || is_at_async_function(p, LineBreak::DoNotCheck)
}

#[inline]
fn parse_function(p: &mut Parser, m: Marker, kind: FunctionKind) -> CompletedMarker {
    let mut flags = SignatureFlags::empty();

    let in_async = is_at_async_function(p, LineBreak::DoNotCheck);
    if in_async {
        p.bump_remap(T![async]);
        flags |= SignatureFlags::ASYNC;
    }

    p.expect(T![function]);
    let generator_range = if p.eat(T![*]) {
        flags |= SignatureFlags::GENERATOR;
        p.tokens.last_tok().map(|t| t.range())
    } else {
        None
    };

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
        .parse_exclusive_syntax(p, parse_ts_type_parameters, |p, marker| {
            p.err_builder("type parameters can only be used in TypeScript files")
                .primary(marker.range(p), "")
        })
        .ok();

    let parameter_context = if !kind.is_expression() && TypeScript.is_supported(p) {
        // It isn't known at this point if this is a function overload definition (body is missing)
        // or a regular function implementation.
        // Let's go with the laxer of the two. Ideally, these verifications should be part of
        // a second compiler pass.
        ParameterContext::Declaration
    } else {
        ParameterContext::Implementation
    };

    parse_parameter_list(p, parameter_context, flags)
        .or_add_diagnostic(p, js_parse_error::expected_parameters);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, marker| {
            p.err_builder("return types can only be used in TypeScript files")
                .primary(marker.range(p), "")
        })
        .ok();

    let body = parse_function_body(p, flags);

    // test ts ts_function_overload
    // function test(a: string): void;
    // function test(a: string | undefined): void {}
    // function no_semi(a: string)
    // function no_semi(a: string) {}
    // async function async_overload(a: string)
    // async function async_overload(a: string) {}
    if body.is_absent()
        && TypeScript.is_supported(p)
        && is_semi(p, 0)
        && !kind.is_in_single_statement_context()
        && !kind.is_expression()
    {
        p.eat(T![;]);

        // test_err ts ts_function_overload_generator
        // function* test(a: string);
        // function* test(a: string) {}
        if let Some(generator_range) = generator_range {
            p.error(
                p.err_builder("An overload signature cannot be declared as a generator.")
                    .primary(generator_range, ""),
            );
        }

        m.complete(p, TS_DECLARE_FUNCTION_DECLARATION)
    } else {
        body.or_add_diagnostic(p, js_parse_error::expected_function_body);

        let mut function = m.complete(p, kind.into());

        // test_err async_or_generator_in_single_statement_context
        // if (true) async function t() {}
        // if (true) function* t() {}
        if kind.is_in_single_statement_context() && (in_async || generator_range.is_some()) {
            p.error(p.err_builder("`async` and generator functions can only be declared at top level or inside a block").primary(function.range(p), ""));
            function.change_to_unknown(p);
        }

        function
    }
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

// test ts ts_declare_function
// declare function test<A, B, R>(a: A, b: B): R;
// declare function test2({ a }?: { a: "string" })
// declare
// function not_a_declaration() {}
//
// test_err ts ts_declare_function_with_body
// declare function test<A>(a: A): string { return "ambient function with a body"; }
//
// test ts ts_ambient_function
// declare module a {
//   function test(): string;
// }
pub(crate) fn parse_ambient_function(p: &mut Parser, m: Marker) -> CompletedMarker {
    let stmt_start = p.cur_tok().start();

    // test_err ts ts_declare_async_function
    // declare async function test();
    let is_async = is_at_contextual_keyword(p, "async");
    if is_async {
        p.error(
            p.err_builder("'async' modifier cannot be used in an ambient context.")
                .primary(p.cur_tok().range(), ""),
        );
        p.bump_remap(T![async]);
    }

    p.expect(T![function]);
    parse_binding(p).or_add_diagnostic(p, expected_binding);
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    parse_ts_return_type_annotation(p).ok();

    if let Present(body) = parse_function_body(p, SignatureFlags::empty()) {
        p.error(
            p.err_builder("A 'declare' function cannot have a function body")
                .primary(body.range(p), "remove this body"),
        );
    }

    semi(p, stmt_start..p.cur_tok().start());

    if is_async {
        m.complete(p, JS_UNKNOWN_STATEMENT)
    } else {
        m.complete(p, TS_DECLARE_FUNCTION_DECLARATION)
    }
}

pub(crate) fn parse_ts_type_annotation_or_error(p: &mut Parser) -> ParsedSyntax {
    TypeScript.parse_exclusive_syntax(p, parse_ts_type_annotation, |p, annotation| {
        p.err_builder("return types can only be used in TypeScript files")
            .primary(annotation.range(p), "remove this type annotation")
    })
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq)]
#[repr(u8)]
pub(crate) enum LineBreak {
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

/// There are cases where the parser must speculatively parse a syntax. For example,
/// parsing `<string>(test)` very much looks like an arrow expression *except* that it isn't followed
/// by a `=>`. This enum tells a parse function if ambiguity should be tolerated or if it should stop if it is not.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Ambiguity {
    /// Ambiguity is allowed. A parse method should continue even if an expected character is missing.
    Allowed,

    /// Ambiguity isn't allowed. A parse method should stop parsing if an expected character is missing
    /// and let the caller decide what to do in this case.
    Disallowed,
}

impl Ambiguity {
    fn is_disallowed(&self) -> bool {
        matches!(self, Ambiguity::Disallowed)
    }
}

pub(crate) fn parse_arrow_function_expression(p: &mut Parser) -> ParsedSyntax {
    parse_parenthesized_arrow_function_expression(p)
        .or_else(|| parse_arrow_function_with_single_parameter(p))
}

/// Parses the header of a parenthesized arrow function expression.
///
/// The header is everything coming before the body: `async (a) =>`.
///
/// Returns `Err` if `ambiguity` is [Ambiguity::Disallowed] and the syntax
/// is ambiguous. For example, the parser speculatively tries to parse `<string>(test)` as an arrow
/// function because the start very much looks like one, except that the `=>` token is missing
/// (it's a TypeScript `<string>` cast followed by a parenthesized expression).
fn parse_parenthesized_arrow_function_head(
    p: &mut Parser,
    ambiguity: Ambiguity,
) -> Result<(Marker, SignatureFlags), Marker> {
    let m = p.start();
    let flags = if eat_contextual_keyword(p, "async", T![async]) {
        SignatureFlags::ASYNC
    } else {
        SignatureFlags::empty()
    };

    if p.at(T![<]) {
        parse_ts_type_parameters(p).ok();

        if ambiguity.is_disallowed() && p.tokens.last_tok().map(|t| t.kind) != Some(T![>]) {
            return Err(m);
        }
    }

    if !p.at(T!['(']) && ambiguity.is_disallowed() {
        return Err(m);
    }

    parse_parameter_list(
        p,
        ParameterContext::Arrow,
        arrow_function_parameter_flags(p, flags),
    )
    .or_add_diagnostic(p, expected_parameters);

    if p.tokens.last_tok().map(|t| t.kind) != Some(T![')']) && ambiguity.is_disallowed() {
        return Err(m);
    }

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "return type annotation", annotation.range(p).as_range())
        })
        .ok();

    if p.has_linebreak_before_n(0) {
        p.error(
            p.err_builder("Line terminator not permitted before arrow.")
                .primary(p.cur_tok().range(), ""),
        );
    }

    if !p.expect(T![=>]) && ambiguity.is_disallowed() {
        return Err(m);
    }

    Ok((m, flags))
}

// test ts ts_arrow_function_type_parameters
// let a = <A, B extends A, C = string>(a: A, b: B, c: C) => "hello";
// let b = async <A, B>(a: A, b: B): Promise<string> => "hello";
fn parse_possible_parenthesized_arrow_function_expression(p: &mut Parser) -> ParsedSyntax {
    let start_pos = p.token_pos();

    // Test if we already tried to parse this position as an arrow function and failed.
    // If so, bail out immediately.
    if p.state.not_parenthesized_arrow.contains(&start_pos) {
        return Absent;
    }

    match try_parse(p, |p| {
        parse_parenthesized_arrow_function_head(p, Ambiguity::Disallowed)
    }) {
        Ok((m, flags)) => {
            parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);

            Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
        }
        Err(m) => {
            // SAFETY: Abandoning the marker here is safe because `try_parse` rewinds if
            // the callback returns `Err` (which is the case that this branch is handling).
            m.abandon(p);

            p.state.not_parenthesized_arrow.insert(start_pos);
            Absent
        }
    }
}

fn parse_parenthesized_arrow_function_expression(p: &mut Parser) -> ParsedSyntax {
    let is_parenthesized = is_parenthesized_arrow_function_expression(p);

    match is_parenthesized {
        IsParenthesizedArrowFunctionExpression::True => {
            let (m, flags) = parse_parenthesized_arrow_function_head(p, Ambiguity::Allowed).expect("'CompletedMarker' because function should never return 'Err' if called with 'Ambiguity::Allowed'.");
            parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);
            Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
        }
        IsParenthesizedArrowFunctionExpression::Unknown => {
            parse_possible_parenthesized_arrow_function_expression(p)
        }
        IsParenthesizedArrowFunctionExpression::False => Absent,
    }
}

#[derive(Debug, Copy, Clone)]
enum IsParenthesizedArrowFunctionExpression {
    True,
    False,
    Unknown,
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
// (a, b) =>;
// (a: string;
// (a, b)
//  => {}

fn is_parenthesized_arrow_function_expression(
    p: &Parser,
) -> IsParenthesizedArrowFunctionExpression {
    match p.cur() {
        // These could be the start of a parenthesized arrow function expression but needs further verification
        T!['('] | T![<] => {
            is_parenthesized_arrow_function_expression_impl(p, SignatureFlags::empty())
        }
        T![ident] if is_at_contextual_keyword(p, "async") => {
            // test async_arrow_expr
            // let a = async foo => {}
            // let b = async (bar) => {}
            // async (foo, bar, ...baz) => foo
            if p.has_linebreak_before_n(1) {
                IsParenthesizedArrowFunctionExpression::False
            } else if matches!(p.nth(1), T!['('] | T![<]) {
                is_parenthesized_arrow_function_expression_impl(p, SignatureFlags::ASYNC)
            } else {
                IsParenthesizedArrowFunctionExpression::False
            }
        }

        // Not entirely correct but that's probably what the user intended
        T![=>] => IsParenthesizedArrowFunctionExpression::True,
        _ => IsParenthesizedArrowFunctionExpression::False,
    }
}

// Tests if the parser is at an arrow function expression
fn is_parenthesized_arrow_function_expression_impl(
    p: &Parser,
    flags: SignatureFlags,
) -> IsParenthesizedArrowFunctionExpression {
    let n = if flags.contains(SignatureFlags::ASYNC) {
        1
    } else {
        0
    };

    match p.nth(n) {
        T!['('] => {
            match p.nth(n + 1) {
                T![')'] => {
                    // '()' is an arrow expression if followed by an '=>', a type annotation or body.
                    // Otherwise, a parenthesized expression with a missing inner expression
                    match p.nth(n + 2) {
                        T![=>] | T![:] | T!['{'] => IsParenthesizedArrowFunctionExpression::True,
                        _ => IsParenthesizedArrowFunctionExpression::False,
                    }
                }
                // Rest parameter '(...a' is certainly not a parenthesized expression
                T![...] => IsParenthesizedArrowFunctionExpression::True,
                // '([ ...', '({ ... } can either be a parenthesized object or array expression or a destructing parameter
                T!['['] | T!['{'] => IsParenthesizedArrowFunctionExpression::Unknown,

                // '(a...'
                _ if is_nth_at_identifier_binding(p, n + 1) || p.nth_at(n + 1, T![this]) => {
                    match p.nth(n + 2) {
                        // '(a: ' must be a type annotation
                        T![:] => IsParenthesizedArrowFunctionExpression::True,

                        // Unclear because it could either be
                        // * '(a = ': an initializer or a parenthesized assignment expression
                        // * '(a, ': separator to next parameter or a parenthesized sequence expression
                        // * '(a)': a single parameter OR a parenthesized expression
                        T![=] | T![,] | T![')'] => IsParenthesizedArrowFunctionExpression::Unknown,

                        T![?] => {
                            // Disambiguate between an optional parameter and a parenthesized conditional expression
                            match p.nth(n + 3) {
                                // '(a?:' | '(a?,' | '(a?=' | '(a?)'
                                T![:] | T![,] | T![=] | T![')'] => {
                                    IsParenthesizedArrowFunctionExpression::True
                                }
                                _ => IsParenthesizedArrowFunctionExpression::False,
                            }
                        }
                        _ => IsParenthesizedArrowFunctionExpression::False,
                    }
                }
                _ => IsParenthesizedArrowFunctionExpression::False,
            }
        }
        // potential start of type parameters
        T![<] => {
            // <a...
            if is_nth_at_identifier(p, n + 1) {
                IsParenthesizedArrowFunctionExpression::Unknown
            } else {
                IsParenthesizedArrowFunctionExpression::False
            }
        }
        _ => unreachable!(),
    }
}

/// Computes the signature flags for parsing the parameters of an arrow expression. These
/// have different semantics from parsing the body
fn arrow_function_parameter_flags(p: &Parser, mut flags: SignatureFlags) -> SignatureFlags {
    if p.state.in_generator() {
        // Arrow functions inherit whatever yield is a valid identifier name from the parent.
        flags |= SignatureFlags::GENERATOR;
    }

    // The arrow function is in an async context if the outer function is in an async context or itself is
    // declared async
    if p.state.in_async() {
        flags |= SignatureFlags::ASYNC;
    }

    flags
}

// test arrow_expr_single_param
// // SCRIPT
// foo => {}
// yield => {}
// await => {}
// baz =>
// {}
fn parse_arrow_function_with_single_parameter(p: &mut Parser) -> ParsedSyntax {
    if !is_arrow_function_with_single_parameter(p) {
        return Absent;
    }

    let m = p.start();
    let is_async = is_at_contextual_keyword(p, "async") && is_nth_at_identifier_binding(p, 1);

    let flags = if is_async {
        expect_contextual_keyword(p, "async", T![async]);
        SignatureFlags::ASYNC
    } else {
        SignatureFlags::empty()
    };

    // test_err async_arrow_expr_await_parameter
    // let a = async await => {}
    // async() => { (a = await) => {} };
    p.with_state(EnterParameters(arrow_function_parameter_flags(p, flags)), parse_binding)
        .expect("Expected function parameter to be present as guaranteed by is_arrow_function_with_simple_parameter");

    p.bump(T![=>]);
    parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);

    Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
}

fn is_arrow_function_with_single_parameter(p: &Parser) -> bool {
    if is_at_contextual_keyword(p, "async") && !p.has_linebreak_before_n(1) {
        is_nth_at_identifier_binding(p, 1) && p.nth_at(2, T![=>]) && !p.has_linebreak_before_n(2)
    } else {
        is_at_identifier_binding(p) && p.nth_at(1, T![=>]) && !p.has_linebreak_before_n(1)
    }
}

fn parse_arrow_body(p: &mut Parser, mut flags: SignatureFlags) -> ParsedSyntax {
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
        p.with_state(EnterFunction(flags), |p| {
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
        })
    }
}

pub(crate) fn parse_any_parameter(
    p: &mut Parser,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    let parameter = match p.cur() {
        T![...] => parse_rest_parameter(p, expression_context),
        T![this] => parse_ts_this_parameter(p),
        _ => parse_formal_parameter(p, parameter_context, expression_context),
    };

    parameter.map(|mut parameter| {
        if parameter.kind() == TS_THIS_PARAMETER {
            if TypeScript.is_unsupported(p) {
                parameter.change_to_unknown(p);
                p.error(ts_only_syntax_error(
                    p,
                    "this parameter",
                    parameter.range(p).as_range(),
                ));
            } else if parameter_context.is_arrow_function() {
                // test_err ts ts_arrow_function_this_parameter
                // let a = (this: string) => {}
                parameter.change_to_unknown(p);
                p.error(
                    p.err_builder("An arrow function cannot have a 'this' parameter.")
                        .primary(parameter.range(p), ""),
                );
            }
        }

        parameter
    })
}

pub(crate) fn parse_rest_parameter(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);
    parse_binding_pattern(p, context).or_add_diagnostic(p, expected_binding);

    let mut valid = true;

    if p.eat(T![?]) {
        let err = p
            .err_builder("rest patterns cannot be optional")
            .primary(p.cur_tok().range(), "");

        p.error(err);
        valid = false;
    }

    // type annotation `...foo: number[]`
    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "type annotation", annotation.range(p).as_range())
        })
        .ok();

    if let Present(initializer) = parse_initializer_clause(p, ExpressionContext::default()) {
        // test_err arrow_rest_in_expr_in_initializer
        // for ((...a = "b" in {}) => {};;) {}
        let err = p
            .err_builder("rest elements may not have default initializers")
            .primary(initializer.range(p), "");

        p.error(err);
        valid = false;
    }

    let mut rest_parameter = m.complete(p, JS_REST_PARAMETER);

    if p.at(T![,]) {
        let err = p
            .err_builder("rest elements may not have trailing commas")
            .primary(rest_parameter.range(p), "");

        p.error(err);
        valid = false;
    }

    if !valid {
        rest_parameter.change_to_unknown(p);
    }

    Present(rest_parameter)
}

// test ts ts_this_parameter
// function a(this) {}
// function b(this: string) {}
pub(crate) fn parse_ts_this_parameter(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![this]) {
        return Absent;
    }

    let parameter = p.start();
    p.bump(T![this]);
    parse_ts_type_annotation(p).ok();
    Present(parameter.complete(p, TS_THIS_PARAMETER))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ParameterContext {
    /// Regular parameter in a function / method implementation: `function x(a) {}`
    Implementation,

    /// Parameter of a function/method declaration: `declare function x(a);`
    Declaration,

    /// Parameter of a setter function: `set a(b: string)`
    Setter,

    /// Parameter of an arrow function
    Arrow,

    /// Parameter inside a TS property parameter: `constructor(private a)`
    ParameterProperty,
}

impl ParameterContext {
    pub fn is_setter(&self) -> bool {
        self == &ParameterContext::Setter
    }

    pub fn is_implementation(&self) -> bool {
        self == &ParameterContext::Implementation
    }

    pub fn is_parameter_property(&self) -> bool {
        self == &ParameterContext::ParameterProperty
    }

    pub fn is_arrow_function(&self) -> bool {
        self == &ParameterContext::Arrow
    }
}

// test ts ts_formal_parameter
// function a(x) {}
// function b({ x, y } = {}) {}
// function c(x: string, y?: number, z: string = "test") {}
//
// test_err ts ts_formal_parameter_error
// function a(x?: string = "test") {}
// function b(...rest: string[] = "init") {}
// function c(...rest, b: string) {}
//
// test_err js_formal_parameter_error
// function a(x: string) {}
// function b(x?) {}
pub(crate) fn parse_formal_parameter(
    p: &mut Parser,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    parse_binding_pattern(p, expression_context).map(|binding| {
        let m = binding.precede(p);
        let mut valid = true;

        let is_optional = if p.at(T![?]) {
            if TypeScript.is_unsupported(p) {
                p.error(ts_only_syntax_error(
                    p,
                    "optional parameters",
                    p.cur_tok().range(),
                ));
                valid = false;
            } else if parameter_context.is_setter() {
                p.error(
                    p.err_builder("A 'set' accessor cannot have an optional parameter.")
                        .primary(p.cur_tok().range(), ""),
                );
                valid = false;
            }

            p.bump(T![?]);
            true
        } else {
            false
        };

        if valid
            && matches!(
                binding.kind(),
                JS_OBJECT_BINDING_PATTERN | JS_ARRAY_BINDING_PATTERN
            )
        {
            if parameter_context.is_parameter_property() {
                valid = false;
                p.error(
                    p.err_builder(
                        "A parameter property may not be declared using a binding pattern.",
                    )
                    .primary(binding.range(p), ""),
                );
            } else if parameter_context.is_implementation() && is_optional {
                valid = false;
                p.error(
					p.err_builder(
						"A binding pattern parameter cannot be optional in an implementation signature.",
					)
						.primary(binding.range(p), ""),
				);
            }
        }

        TypeScript
            .parse_exclusive_syntax(p, parse_ts_type_annotation, |p, annotation| {
                ts_only_syntax_error(p, "Type annotations", annotation.range(p).as_range())
            })
            .ok();

        if let Present(initializer) = parse_initializer_clause(p, expression_context) {
            if valid && parameter_context.is_setter() && TypeScript.is_supported(p) {
                p.error(
                    p.err_builder("A 'set' accessor parameter cannot have an initializer.")
                        .primary(initializer.range(p), ""),
                );
            } else if is_optional && valid {
                p.error(
                    p.err_builder("Parameter cannot have question mark and initializer")
                        .primary(initializer.range(p), ""),
                );
            }
        }

        let mut parameter = m.complete(p, JS_FORMAL_PARAMETER);

        if !valid {
            parameter.change_to_unknown(p);
        }

        parameter
    })
}

/// Skips over the binding token of a parameter. Useful in the context of lookaheads to determine
/// if any typescript specific syntax like `:` is present after the parameter name.
/// Returns `true` if the function skipped over a valid binding, returns false if the parser
/// is not positioned at a binding.
pub(super) fn skip_parameter_start(p: &mut Parser) -> bool {
    if is_at_identifier_binding(p) || p.at(T![this]) {
        // a
        p.bump_any();
        return true;
    }

    if p.at(T!['[']) || p.at(T!['{']) {
        // Array or object pattern. Try to parse it and return true if there were no parsing errors
        let previous_error_count = p.errors.len();
        let pattern = parse_binding_pattern(
            p,
            ExpressionContext::default().and_object_expression_allowed(true),
        );
        pattern.is_present() && p.errors.len() == previous_error_count
    } else {
        false
    }
}

// test parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(
    p: &mut Parser,
    parameter_context: ParameterContext,
    flags: SignatureFlags,
) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    parse_parameters_list(
        p,
        flags,
        |p, expression_context| parse_any_parameter(p, parameter_context, expression_context),
        JS_PARAMETER_LIST,
    );

    Present(m.complete(p, JS_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
    p: &mut Parser,
    flags: SignatureFlags,
    parse_parameter: impl Fn(&mut Parser, ExpressionContext) -> ParsedSyntax,
    list_kind: JsSyntaxKind,
) {
    let mut first = true;
    let has_l_paren = p.expect(T!['(']);

    p.with_state(EnterParameters(flags), |p| {
        let parameters_list = p.start();
        let mut progress = ParserProgress::default();

        while !p.at(EOF) && !p.at(T![')']) {
            if first {
                first = false;
            } else {
                p.expect(T![,]);
            }

            if p.at(T![')']) {
                break;
            }

            progress.assert_progressing(p);

            let parameter = parse_parameter(
                p,
                ExpressionContext::default().and_object_expression_allowed(!first || has_l_paren),
            );

            if parameter.is_absent() && p.at(T![,]) {
                // a missing parameter,
                parameter.or_add_diagnostic(p, expected_parameter);
                continue;
            }

            // test_err formal_params_no_binding_element
            // function foo(true) {}

            // test_err formal_params_invalid
            // function (a++, c) {}
            let recovered_result = parameter.or_recover(
                p,
                &ParseRecovery::new(
                    JS_UNKNOWN_PARAMETER,
                    token_set![
                        T![ident],
                        T![await],
                        T![yield],
                        T![this],
                        T![,],
                        T!['['],
                        T![...],
                        T!['{'],
                        T![')'],
                        T![;],
                    ],
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_parameter,
            );

            if recovered_result.is_err() {
                break;
            }
        }

        parameters_list.complete(p, list_kind);
    });

    p.expect(T![')']);
}
