use crate::parser::{ParsedSyntax, ParserProgress};
use crate::state::{EnterFunction, EnterParameters, SignatureFlags};
use crate::syntax::binding::{is_at_identifier_binding, parse_binding, parse_binding_pattern};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{parse_assignment_expression_or_higher, ExpressionContext};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    expected_binding, expected_parameter, expected_parameters, ts_only_syntax_error,
};
use crate::syntax::stmt::{is_semi, parse_block_impl, StatementContext};
use crate::syntax::typescript::{
    parse_ts_return_type_annotation, parse_ts_type_annotation, parse_ts_type_parameters,
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
//
// test_err ts ts_optional_pattern_parameter
// function test({a, b}?) {}
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
        .parse_exclusive_syntax(p, parse_ts_type_parameters, |p, marker| {
            p.err_builder("type parameters can only be used in TypeScript files")
                .primary(marker.range(p), "")
        })
        .ok();

    parse_parameter_list(p, ParameterContext::Implementation, flags)
        .or_add_diagnostic(p, js_parse_error::expected_parameters);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, marker| {
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

// TODO #2058 This is probably not ideal (same with the `declare` keyword). We should
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

pub(crate) fn parse_ts_type_annotation_or_error(p: &mut Parser) -> ParsedSyntax {
    TypeScript.parse_exclusive_syntax(p, parse_ts_type_annotation, |p, annotation| {
        p.err_builder("return types can only be used in TypeScript files")
            .primary(annotation.range(p), "remove this type annotation")
    })
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

pub(super) fn parse_arrow_function(
    p: &mut Parser,
    m: Marker,
    flags: SignatureFlags,
) -> CompletedMarker {
    let parameters = parse_arrow_function_parameters(p, flags);

    if parameters.kind() == Some(JS_PARAMETERS) {
        TypeScript
            .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, annotation| {
                ts_only_syntax_error(p, "return type annotation", annotation.range(p).as_range())
            })
            .ok();
    }
    parameters.or_add_diagnostic(p, expected_parameters);

    p.expect(T![=>]);

    parse_arrow_body(p, SignatureFlags::empty())
        .or_add_diagnostic(p, js_parse_error::expected_arrow_body);
    m.complete(p, JS_ARROW_FUNCTION_EXPRESSION)
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
        parse_parameter_list(p, ParameterContext::Arrow, flags)
    } else {
        // test_err async_arrow_expr_await_parameter
        // let a = async await => {}
        // async() => { (a = await) => {} };
        p.with_state(EnterParameters(flags), parse_binding)
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

    /// Paramter of an arrow function
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
