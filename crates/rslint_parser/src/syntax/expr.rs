//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-11).

use super::typescript::*;
use super::util::*;
use crate::event::rewrite_events;
use crate::event::RewriteParseEvents;
use crate::parser::{ParserProgress, RecoveryResult};
use crate::syntax::assignment::{
    expression_to_assignment, expression_to_assignment_pattern, parse_assignment,
    AssignmentExprPrecedence,
};
use crate::syntax::class::parse_class_expression;
use crate::syntax::function::{
    is_at_async_function, parse_arrow_function_with_simple_parameter, parse_function_expression,
    parse_parenthesized_arrow_function_expression, LineBreak,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    expected_expression, expected_identifier, expected_parameters,
    expected_simple_assignment_target, expected_ts_type, ts_only_syntax_error,
};
use crate::syntax::object::parse_object_expression;
use crate::syntax::stmt::{is_semi, STMT_RECOVERY_SET};
use crate::JsSyntaxFeature::{StrictMode, TypeScript};
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsSyntaxKind::*, *};
use bitflags::bitflags;
use rome_rowan::SyntaxKind;
use rslint_errors::Span;

pub const EXPR_RECOVERY_SET: TokenSet = token_set![VAR_KW, R_PAREN, L_PAREN, L_BRACK, R_BRACK];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ExpressionContext(ExpressionContextFlags);

bitflags! {
    struct ExpressionContextFlags: u8 {
        /// Whether `in` should be counted in a binary expression.
        /// This is for `for...in` statements to prevent ambiguity.
        /// Corresponds to `[+In]` in the EcmaScript spec if true
        const INCLUDE_IN = 1 << 0;

        /// If false, object expressions are not allowed to be parsed
        /// inside an expression.
        ///
        /// Also applies for object patterns
        const ALLOW_OBJECT_EXPRESSION = 1 << 1;
    }
}

impl ExpressionContext {
    pub(crate) fn and_include_in(self, include: bool) -> Self {
        self.and(ExpressionContextFlags::INCLUDE_IN, include)
    }

    pub(crate) fn and_object_expression_allowed(self, allowed: bool) -> Self {
        self.and(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION, allowed)
    }

    /// Returns true if object expressions or object patterns are valid in this context
    pub(crate) fn is_object_expression_allowed(&self) -> bool {
        self.0
            .contains(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION)
    }

    /// Returns `true` if the expression parsing includes binary in expressions.
    pub(crate) fn is_in_included(&self) -> bool {
        self.0.contains(ExpressionContextFlags::INCLUDE_IN)
    }

    /// Adds the `flag` if `set` is `true`, otherwise removes the `flag`
    fn and(self, flag: ExpressionContextFlags, set: bool) -> Self {
        ExpressionContext(if set { self.0 | flag } else { self.0 - flag })
    }
}

/// Sets the default flags for a context that parses a new root expression (for example, the condition of an if statement)
/// or sub-expression of another expression (the alternate branch of a condition expression).
impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext(
            ExpressionContextFlags::INCLUDE_IN | ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION,
        )
    }
}

/// Parses an expression or recovers to the point of where the next statement starts
pub(crate) fn parse_expression_or_recover_to_next_statement(
    p: &mut Parser,
    assign: bool,
    context: ExpressionContext,
) -> RecoveryResult {
    let func = if assign {
        syntax::expr::parse_assignment_expression_or_higher
    } else {
        syntax::expr::parse_expression
    };

    func(p, context).or_recover(
        p,
        &ParseRecovery::new(
            JsSyntaxKind::JS_UNKNOWN_EXPRESSION,
            STMT_RECOVERY_SET.union(token_set![T!['}']]),
        )
        .enable_recovery_on_line_break(),
        expected_expression,
    )
}

/// A literal expression.
///
/// `TRUE | FALSE | NUMBER | STRING | NULL`
// test literals
// 5
// true
// false
// 5n
// "foo"
// 'bar'
// null
// 0, 0.0, 0n, 0e00

// test_err literals
// 00, 012, 08, 091, 0789 // parser errors
// 01n, 0_0, 01.2 // lexer errors
pub(super) fn parse_literal_expression(p: &mut Parser) -> ParsedSyntax {
    let literal_kind = match p.cur_tok().kind {
        JsSyntaxKind::JS_NUMBER_LITERAL => {
            return parse_number_literal_expression(p)
                .or_else(|| parse_big_int_literal_expression(p));
        }
        JsSyntaxKind::JS_STRING_LITERAL => JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
        JsSyntaxKind::NULL_KW => JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
        JsSyntaxKind::TRUE_KW | JsSyntaxKind::FALSE_KW => {
            JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION
        }
        JsSyntaxKind::JS_REGEX_LITERAL => JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, literal_kind))
}

pub(crate) fn parse_big_int_literal_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(JS_NUMBER_LITERAL) || !p.cur_src().ends_with('n') {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(JsSyntaxKind::JS_BIG_INT_LITERAL);
    Present(m.complete(p, JS_BIG_INT_LITERAL_EXPRESSION))
}

pub(crate) fn parse_number_literal_expression(p: &mut Parser) -> ParsedSyntax {
    let cur_src = p.cur_src();
    if !p.at(JS_NUMBER_LITERAL) || cur_src.ends_with('n') {
        return Absent;
    }

    // Forbid legacy octal number in strict mode
    if p.state.strict().is_some()
        && cur_src.starts_with('0')
        && cur_src.chars().nth(1).filter(|c| c.is_digit(10)).is_some()
    {
        let err_msg = if cur_src.contains(['8', '9']) {
            "Decimals with leading zeros are not allowed in strict mode."
        } else {
            "\"0\"-prefixed octal literals are deprecated; use the \"0o\" prefix instead."
        };
        p.error(p.err_builder(err_msg).primary(p.cur_tok().range(), ""));
    }

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, JS_NUMBER_LITERAL_EXPRESSION))
}

/// Parses an assignment expression or any higher expression
/// https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-AssignmentExpression
pub(crate) fn parse_assignment_expression_or_higher(
    p: &mut Parser,
    context: ExpressionContext,
) -> ParsedSyntax {
    let arrow_expression = parse_arrow_function_with_simple_parameter(p)
        .or_else(|| parse_parenthesized_arrow_function_expression(p));

    if arrow_expression.is_present() {
        return arrow_expression;
    }

    parse_assignment_expression_or_higher_base(p, context)
}

fn parse_assignment_expression_or_higher_base(
    p: &mut Parser,
    context: ExpressionContext,
) -> ParsedSyntax {
    if p.state.in_generator() && p.at(T![yield]) {
        return Present(parse_yield_expression(p, context));
    }

    let checkpoint = p.checkpoint();
    parse_conditional_expr(p, context)
        .and_then(|target| parse_assign_expr_recursive(p, target, checkpoint, context))
}

// test assign_expr
// foo += bar = b ??= 3;
// foo -= bar;
// (foo = bar);
// [foo, bar] = baz;
// [foo, bar = "default", ...rest] = baz;
// [,,,foo,bar] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});

// test_err assign_expr_right
// (foo = );

// test_err assign_expr_left
// ( = foo);

// test assign_eval_member_or_computed_expr
// eval.foo = 10
// arguments[1] = "baz"
// eval[2] = "Chungking Express"

// test_err assign_eval_or_arguments
// eval = 0
// eval ??= 2
// eval *= 4
// arguments = "foo"
// arguments ||= "baz"
// ({ eval } = o)
// ({ foo: { eval }}) = o
fn parse_assign_expr_recursive(
    p: &mut Parser,
    target: CompletedMarker,
    checkpoint: Checkpoint,
    context: ExpressionContext,
) -> ParsedSyntax {
    if is_assign_token(p.cur()) {
        let target = expression_to_assignment_pattern(p, target, checkpoint);
        let m = target.precede(p);
        p.bump_any(); // operator
        parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true))
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
        Present(m.complete(p, JS_ASSIGNMENT_EXPRESSION))
    } else {
        Present(target)
    }
}

fn is_assign_token(kind: JsSyntaxKind) -> bool {
    matches!(
        kind,
        T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T![%=]
            | T![<<=]
            | T![>>=]
            | T![>>>=]
            | T![&=]
            | T![|=]
            | T![^=]
            | T![&&=]
            | T![||=]
            | T![??=]
            | T![**=]
    )
}

// test yield_expr
// function *foo() {
//  yield foo;
//  yield* foo;
//  yield;
//  yield
//  yield
// }
fn parse_yield_expression(p: &mut Parser, context: ExpressionContext) -> CompletedMarker {
    let m = p.start();
    p.expect(T![yield]);

    if !is_semi(p, 0) && (p.at(T![*]) || is_at_expression(p)) {
        let argument = p.start();
        p.eat(T![*]);
        parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true)).ok();

        argument.complete(p, JS_YIELD_ARGUMENT);
    }

    let mut yield_expr = m.complete(p, JS_YIELD_EXPRESSION);

    if !p.state.is_top_level() && !p.state.in_function() {
        // test_err yield_expr_in_parameter_initializer
        // function* test(a = yield "test") {}
        // function test2(a = yield "test") {}
        p.error(
            p.err_builder("`yield` is only allowed within generator functions.")
                .primary(yield_expr.range(p), ""),
        );
        yield_expr.change_to_unknown(p);
    }

    yield_expr
}

/// A conditional expression such as `foo ? bar : baz`
// test conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz
pub(super) fn parse_conditional_expr(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    // test_err conditional_expr_err
    // foo ? bar baz
    // foo ? bar baz ? foo : bar
    // foo ? bar :
    let lhs = parse_binary_or_logical_expression(p, OperatorPrecedence::lowest(), context);

    if p.at(T![?]) {
        lhs.map(|marker| {
            let m = marker.precede(p);
            p.bump(T![?]);

            parse_assignment_expression_or_higher(p, context.and_include_in(true))
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

            p.expect(T![:]);

            parse_assignment_expression_or_higher(p, context)
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
            m.complete(p, JS_CONDITIONAL_EXPRESSION)
        })
    } else {
        lhs
    }
}

/// A binary expression such as `2 + 2` or `foo * bar + 2` or a logical expression 'a || b'
fn parse_binary_or_logical_expression(
    p: &mut Parser,
    left_precedence: OperatorPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    // test private_name_presence_check
    // class A {
    // 	#prop;
    // 	test() {
    //    #prop in this
    //  }
    // }
    let left = parse_unary_expr(p, context).or_else(|| parse_private_name(p));

    parse_binary_or_logical_expression_recursive(p, left, left_precedence, context)
}

// test binary_expressions
// 5 * 5
// 6 ** 6 ** 7
// 1 + 2 * 3
// (1 + 2) * 3
// 1 / 2
// 74 in foo
// foo instanceof Array
// foo ?? bar
// a >> b
// a >>> b
// 1 + 1 + 1 + 1
// 5 + 6 - 1 * 2 / 1 ** 6

// test_err binary_expressions_err
// foo(foo +);
// foo + * 2;
// !foo * bar;
fn parse_binary_or_logical_expression_recursive(
    p: &mut Parser,
    mut left: ParsedSyntax,
    left_precedence: OperatorPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    // Use a loop to eat all binary expressions with the same precedence.
    // At first, the algorithm makes the impression that it recurse for every right-hand side expression.
    // This is true, but `parse_binary_or_logical_expression` immediately returns if the
    // current operator has the same or a lower precedence than the left-hand side expression. Thus,
    // the algorithm goes at most `count(OperatorPrecedence)` levels deep.
    loop {
        let op = match p.cur() {
            T![>] if p.nth_at(1, T![>]) && p.nth_at(2, T![>]) => T![>>>],
            T![>] if p.nth_at(1, T![>]) => T![>>],
            T![in] if !context.is_in_included() => {
                break;
            }
            _ if is_at_contextual_keyword(p, "as") && !p.has_linebreak_before_n(0) => T![as],
            k => k,
        };

        let new_precedence = match OperatorPrecedence::try_from_binary_operator(op) {
            Ok(precedence) => precedence,
            // Not a binary operator
            Err(_) => break,
        };

        let stop_at_current_operator = if new_precedence.is_right_to_left() {
            new_precedence < left_precedence
        } else {
            new_precedence <= left_precedence
        };

        if stop_at_current_operator {
            break;
        }

        let op_tok = p.cur_tok();

        let mut is_unknown = false;
        if let Present(left) = &left {
            // test exponent_unary_parenthesized
            // (delete a.b) ** 2;
            // (void ident) ** 2;
            // (typeof ident) ** 2;
            // (-3) ** 2;
            // (+3) ** 2;
            // (~3) ** 2;
            // (!true) ** 2;

            // test_err exponent_unary_unparenthesized
            // delete a.b ** 2;
            // void ident ** 2;
            // typeof ident ** 2;
            // -3 ** 2;
            // +3 ** 2;
            // ~3 ** 2;
            // !true ** 2;

            if op == T![**] && left.kind() == JS_UNARY_EXPRESSION {
                let err = p
					.err_builder(
						"unparenthesized unary expression can't appear on the left-hand side of '**'",
					)
					.secondary(op_tok.range(), "")
					.primary(left.range(p), "");

                p.error(err);
                is_unknown = true;
            }
        }

        let m = left.precede(p);
        if op == T![>>] {
            p.bump_multiple(2, T![>>]);
        } else if op == T![>>>] {
            p.bump_multiple(3, T![>>>]);
        } else {
            p.bump_remap(op);
        }

        // test ts ts_as_expression
        // let x: any = "string";
        // let y = x as string;
        // let z = x as const;
        // let not_an_as_expression = x
        // as;
        // let precedence = "hello" as const + 3 as number as number;
        if op == T![as] {
            parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
            let mut as_expression = m.complete(p, TS_AS_EXPRESSION);

            if TypeScript.is_unsupported(p) {
                p.error(ts_only_syntax_error(
                    p,
                    "as expression",
                    as_expression.range(p).as_range(),
                ));
                as_expression.change_to_unknown(p);
            }
            left = Present(as_expression);
            continue;
        }

        // This is a hack to allow us to effectively recover from `foo + / bar`
        let right = if OperatorPrecedence::try_from_binary_operator(p.cur()).is_ok()
            && !p.at_ts(token_set![T![-], T![+], T![<]])
        {
            let err = p.err_builder(&format!("Expected an expression for the right hand side of a `{}`, but found an operator instead", p.token_src(op_tok)))
				.secondary(op_tok.range(), "This operator requires a right hand side value")
				.primary(p.cur_tok().range(), "But this operator was encountered instead");

            p.error(err);

            parse_binary_or_logical_expression_recursive(
                p,
                Absent,
                OperatorPrecedence::lowest(),
                context,
            )
        } else if p.at(T![#]) {
            // test_err private_name_presence_check_recursive
            // class A {
            // 	#prop;
            // 	test() {
            //    #prop in #prop in this
            //  }
            // }
            let mut private_name = parse_private_name(p).unwrap();
            private_name.change_kind(p, JS_UNKNOWN_EXPRESSION);
            p.error(
                p.err_builder(
                    "Private names are only allowed on the left side of a binary expression",
                )
                .primary(private_name.range(p), ""),
            );
            Present(private_name)
        } else {
            parse_binary_or_logical_expression(p, new_precedence, context)
        };

        right.or_add_diagnostic(p, expected_expression);

        let expression_kind = if is_unknown {
            JS_UNKNOWN_EXPRESSION
        } else {
            match op {
                // test logical_expressions
                // foo ?? bar
                // a || b
                // a && b
                //
                // test_err logical_expressions_err
                // foo ?? * 2;
                // !foo && bar;
                // foo(foo ||)
                T![??] | T![||] | T![&&] => JS_LOGICAL_EXPRESSION,
                T![instanceof] => JS_INSTANCEOF_EXPRESSION,
                T![in] => JS_IN_EXPRESSION,
                _ => JS_BINARY_EXPRESSION,
            }
        };

        left = Present(m.complete(p, expression_kind));
    }

    left
}

/// A member or new expression with subscripts. e.g. `new foo`, `new Foo()`, `foo`, or `foo().bar[5]`
// test new_exprs
// new Foo()
// new foo;
// new.target
// new new new new Foo();
// new Foo(bar, baz, 6 + 6, foo[bar] + ((foo) => {}) * foo?.bar)

// test_err new_exprs
// new;
fn parse_member_expression_or_higher(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    parse_primary_expression(p, context)
        .map(|lhs| parse_member_expression_rest(p, lhs, context, true, &mut false))
}

// test_err subscripts_err
// foo()?.baz[].;
// BAR`b
fn parse_member_expression_rest(
    p: &mut Parser,
    lhs: CompletedMarker,
    context: ExpressionContext,
    allow_optional_chain: bool,
    in_optional_chain: &mut bool,
) -> CompletedMarker {
    let mut progress = ParserProgress::default();
    let mut lhs = lhs;

    while !p.at(EOF) {
        progress.assert_progressing(p);
        lhs = match p.cur() {
            T![.] => parse_static_member_expression(p, lhs, T![.]).unwrap(),
            T!['['] => parse_computed_member_expression(p, lhs, false, context).unwrap(),
            T![?.] if allow_optional_chain => {
                let completed = if p.nth_at(1, T!['[']) {
                    parse_computed_member_expression(p, lhs, true, context).unwrap()
                } else if is_nth_at_any_name(p, 1) {
                    parse_static_member_expression(p, lhs, T![?.]).unwrap()
                } else if p.nth_at(1, BACKTICK) {
                    let m = lhs.precede(p);
                    p.bump(T![?.]);
                    let template_literal = p.start();
                    parse_template_literal(p, template_literal, true);
                    m.complete(p, JS_UNKNOWN_EXPRESSION)
                } else {
                    // '(' or any other unexpected character
                    break;
                };
                *in_optional_chain = true;
                completed
            }
            T![!] if !p.has_linebreak_before_n(0) => {
                // test ts ts_non_null_assertion_expression
                // let a = { b: {} };
                // a!;
                // function test() {}
                // test()!
                // 	a.b.c!;
                // a!!!!!!;
                let m = lhs.precede(p);
                p.bump(T![!]);

                let mut non_null = m.complete(p, TS_NON_NULL_ASSERTION_EXPRESSION);

                if TypeScript.is_unsupported(p) {
                    non_null.change_to_unknown(p);
                    p.error(ts_only_syntax_error(
                        p,
                        "non-null assertions",
                        non_null.range(p).as_range(),
                    ));
                }

                non_null
            }
            BACKTICK => {
                // test ts ts_optional_chain_call
                // (<A, B>() => {})?.<A, B>();
                let m = lhs.precede(p);
                parse_template_literal(p, m, *in_optional_chain)
            }
            _ => break,
        }
    }

    lhs
}

fn parse_new_expr(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![new]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();

    // new.target
    if p.at(T![.]) && p.token_src(p.nth_tok(1)) == "target" {
        p.bump_any();
        p.bump_remap(T![target]);
        return Present(m.complete(p, NEW_TARGET));
    }

    let expression = parse_primary_expression(p, context).or_add_diagnostic(p, expected_expression);

    if let Some(lhs) = expression {
        parse_member_expression_rest(p, lhs, context, false, &mut false);
    }

    // test ts ts_new_with_type_arguments
    // class Test<A, B, C> {}
    // new Test<A, B, C>();
    let type_arguments = if TypeScript.is_supported(p) {
        parse_ts_type_arguments_in_expression(p)
    } else {
        Absent
    };

    if p.at(T!['(']) {
        parse_arguments(p, context).unwrap();
    } else if let Present(type_arguments) = type_arguments {
        let error = p.err_builder("A 'new' expression with type arguments must always be followed by a parenthesized argument list.").primary(type_arguments.range(p), "");
        p.error(error);
    }

    Present(m.complete(p, JS_NEW_EXPRESSION))
}

// test super_expression
// class Test extends B {
//   constructor() {
//     super();
//   }
//   test() {
//     super.test(a, b);
//     super[1];
//   }
// }
//
// test_err super_expression_err
// class Test extends B {
//   test() {
//     super();
//     super?.test();
//   }
// }
// super();
fn parse_super_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![super]) {
        return Absent;
    }
    let super_marker = p.start();
    p.expect(T![super]);
    let mut super_expression = super_marker.complete(p, JS_SUPER_EXPRESSION);

    if p.at(T![?.]) {
        super_expression.change_kind(p, JS_UNKNOWN_EXPRESSION);
        p.error(
            p.err_builder("Super doesn't support optional chaining as super can never be null")
                .primary(super_expression.range(p), ""),
        );
    } else if p.at(T!['(']) && !p.state.in_constructor() {
        p.error(
            p.err_builder("`super` is only valid inside of a class constructor of a subclass.")
                .primary(super_expression.range(p), ""),
        );
        super_expression.change_kind(p, JS_UNKNOWN_EXPRESSION);
    }

    match p.cur() {
        T![.] | T!['['] | T!['('] | T![?.] => Present(super_expression),
        _ => parse_static_member_expression(p, super_expression, T![.]),
    }
}

// test subscripts
// foo`bar`
// foo(bar)(baz)(baz)[bar]

/// A static member expression for accessing a property
// test static_member_expression
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
// class Test {
//   #bar
//   test(other) {
//     this.#bar;
//     this?.#bar;
//     other.#bar;
//     other?.#bar;
//   }
// }
fn parse_static_member_expression(
    p: &mut Parser,
    lhs: CompletedMarker,
    operator: JsSyntaxKind,
) -> ParsedSyntax {
    let m = lhs.precede(p);
    p.expect(operator);

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, JS_STATIC_MEMBER_EXPRESSION))
}

fn parse_private_name(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();
    let hash_end = p.cur_tok().range().end;
    p.expect(T![#]);

    if p.at(T![ident]) && hash_end != p.cur_tok().start() {
        // test_err private_name_with_space
        // class A {
        // 	# test;
        // }
        p.error(
            p.err_builder("Unexpected space or comment between `#` and identifier")
                .primary(hash_end..p.cur_tok().start(), "remove the space here"),
        );
        Present(m.complete(p, JS_UNKNOWN))
    } else {
        p.expect(T![ident]);
        Present(m.complete(p, JS_PRIVATE_NAME))
    }
}

pub(super) fn parse_any_name(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        T![#] => parse_private_name(p),
        _ => parse_name(p),
    }
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test computed_member_expression
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
fn parse_computed_member_expression(
    p: &mut Parser,
    lhs: CompletedMarker,
    optional_chain: bool,
    context: ExpressionContext,
) -> ParsedSyntax {
    // test_err bracket_expr_err
    // foo[]
    // foo?.[]
    // foo[
    let m = lhs.precede(p);
    if optional_chain {
        p.expect(T![?.]);
    }

    p.expect(T!['[']);
    // test computed_member_in
    // for ({}["x" in {}];;) {}
    parse_expression(p, context.and_include_in(true)).or_add_diagnostic(p, expected_expression);

    p.expect(T![']']);

    Present(m.complete(p, JS_COMPUTED_MEMBER_EXPRESSION))
}

/// An identifier name, either an ident or a keyword
pub(super) fn parse_name(p: &mut Parser) -> ParsedSyntax {
    if is_at_name(p) {
        let m = p.start();
        p.bump_remap(T![ident]);
        Present(m.complete(p, JS_NAME))
    } else {
        Absent
    }
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`

// test_err invalid_arg_list
// foo(a,b;
// foo(a,b var;
// foo (,,b)
fn parse_arguments(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    // test in_expr_in_arguments
    // function foo() {}
    // for (foo("call" in foo);;) {}

    let m = p.start();
    p.bump(T!['(']);
    let args_list = p.start();
    let mut first = true;
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

        let argument = if p.at(T![...]) {
            // already do a check on "..." so it's safe to unwrap
            parse_spread_element(p, context.and_include_in(true))
        } else {
            parse_assignment_expression_or_higher(p, context.and_include_in(true))
        };

        if argument.is_absent() && p.at(T![,]) {
            argument.or_add_diagnostic(p, js_parse_error::expected_expression);
            // missing element
            continue;
        }

        if argument
            .or_recover(
                p,
                &ParseRecovery::new(
                    JS_UNKNOWN_EXPRESSION,
                    EXPR_RECOVERY_SET.union(token_set!(T![')'], T![;])),
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_expression,
            )
            .is_err()
        {
            break;
        }
    }

    args_list.complete(p, JS_CALL_ARGUMENT_LIST);
    p.expect(T![')']);
    Present(m.complete(p, JS_CALL_ARGUMENTS))
}

// test parenthesized_sequence_expression
// (a, b);
// (a, b, c);
// (a, b, c, d, e, f);
// (a, b, c, d, e, f)
// (a, b, c)

// test_err incomplete_parenthesized_sequence_expression
// (a,;
// (a, b, c;

// test js_parenthesized_expression
// ((foo))
// (foo)

fn parse_parenthesized_expression(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    if p.at(T![')']) {
        // test_err empty_parenthesized_expression
        // ();
        p.error(
            p.err_builder("Parenthesized expression didnt contain anything")
                .primary(p.cur_tok().range(), "Expected an expression here"),
        );
    } else {
        let first =
            parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true));

        if p.at(T![,]) {
            parse_sequence_expression_recursive(p, first, context)
                .or_add_diagnostic(p, expected_expression);
        }
    }

    p.expect(T![')']);
    Present(m.complete(p, JS_PARENTHESIZED_EXPRESSION))
}

pub fn parse_expression_snipped(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    parse_expression(p, ExpressionContext::default()).or_add_diagnostic(p, expected_expression);
    m.complete(p, JS_EXPRESSION_SNIPPED).into()
}

/// A general expression.
pub(crate) fn parse_expression(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    let first = parse_assignment_expression_or_higher(p, context);

    if p.at(T![,]) {
        parse_sequence_expression_recursive(p, first, context)
    } else {
        first
    }
}

// test sequence_expr
// 1, 2, 3, 4, 5

// test_err sequence_expr
// 1, 2, , 4
fn parse_sequence_expression_recursive(
    p: &mut Parser,
    left: ParsedSyntax,
    context: ExpressionContext,
) -> ParsedSyntax {
    if !p.at(T![,]) {
        return left;
    }

    let mut left = left;

    while p.at(T![,]) {
        let sequence_expr_marker =
            left.precede_or_add_diagnostic(p, js_parse_error::expected_expression);
        p.bump(T![,]);
        parse_assignment_expression_or_higher(p, context).or_add_diagnostic(p, expected_expression);

        left = Present(sequence_expr_marker.complete(p, JS_SEQUENCE_EXPRESSION))
    }

    left
}

#[inline]
pub(crate) fn is_at_expression(p: &Parser) -> bool {
    is_nth_at_expression(p, 0)
}

pub(crate) fn is_nth_at_expression(p: &Parser, n: usize) -> bool {
    match p.nth(n) {
        T![!]
        | T!['(']
        | T!['[']
        | T!['{']
        | T![++]
        | T![--]
        | T![~]
        | T![+]
        | T![-]
        | T![throw]
        | T![new]
        | T![typeof]
        | T![void]
        | T![delete]
        | T![ident]
        | T![...]
        | T![this]
        | T![yield]
        | T![await]
        | T![function]
        | T![class]
        | T![import]
        | T![super]
        | T![#]
        | T![<]
        | BACKTICK
        | TRUE_KW
        | FALSE_KW
        | JS_NUMBER_LITERAL
        | JS_STRING_LITERAL
        | NULL_KW
        | JS_REGEX_LITERAL => true,
        T![enum] if !p.has_linebreak_before_n(n + 1) => true,
        _ => false,
    }
}

/// A primary expression such as a literal, an object, an array, or `this`.
fn parse_primary_expression(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    let parsed_literal_expression = parse_literal_expression(p);
    if parsed_literal_expression.is_present() {
        return parsed_literal_expression;
    }

    let complete = match p.cur() {
        T![this] => {
            // test this_expr
            // this
            // this.foo
            let m = p.start();
            p.bump_any();
            m.complete(p, JS_THIS_EXPRESSION)
        }
        T![class] => {
            // test class_expr
            // let a = class {};
            // let b = class foo {
            //  constructor() {}
            // }
            // foo[class {}]
            parse_class_expression(p).unwrap()
        }
        // test async_ident
        // let a = async;
        T![ident] if is_at_async_function(p, LineBreak::DoCheck) => {
            // test async_function_expr
            // let a = async function() {};
            // let b = async function foo() {};
            parse_function_expression(p).unwrap()
        }
        T![function] => {
            // test function_expr
            // let a = function() {}
            // let b = function foo() {}

            parse_function_expression(p).unwrap()
        }
        T![ident] | T![yield] | T![await] | T![enum] => {
            // test identifier_reference
            // // SCRIPT
            // foo;
            // yield;
            // await;
            parse_identifier_expression(p).unwrap()
        }
        // test grouping_expr
        // ((foo))
        // (foo)
        T!['('] => parse_parenthesized_expression(p, context).unwrap(),
        T!['['] => parse_array_expr(p).unwrap(),
        T!['{'] if context.is_object_expression_allowed() => parse_object_expression(p).unwrap(),
        T![import] => {
            let m = p.start();
            p.bump_any();

            // test import_meta
            // import.meta
            if p.eat(T![.]) {
                // test_err import_no_meta
                // import.foo
                // import.metaa
                if p.at(T![ident]) && p.token_src(p.cur_tok()) == "meta" {
                    p.bump_remap(T![meta]);
                    m.complete(p, IMPORT_META)
                } else if p.at(T![ident]) {
                    let err = p
                        .err_builder(&format!(
                            "Expected `meta` following an import keyword, but found `{}`",
                            p.token_src(p.cur_tok())
                        ))
                        .primary(p.cur_tok().range(), "");

                    p.err_and_bump(err, JS_UNKNOWN);
                    m.complete(p, IMPORT_META)
                } else {
                    let err = p
                        .err_builder("Expected `meta` following an import keyword, but found none")
                        .primary(p.cur_tok().range(), "");

                    p.error(err);
                    m.complete(p, JS_UNKNOWN)
                }
            } else if p.at(T!['(']) {
                // test import_call
                // import("foo")
                // import("foo", { assert: { type: 'json' } })

                // test_err import_invalid_args
                // import()
                // import(...["foo"])
                // import("foo", { assert: { type: 'json' } }, "bar")

                let args = p.start();
                p.bump(T!['(']);
                let args_list = p.start();

                let mut progress = ParserProgress::default();
                let mut error_range_start = p.cur_tok().start();
                let mut args_count = 0;

                while !p.at(EOF) && !p.at(T![')']) {
                    progress.assert_progressing(p);
                    args_count += 1;

                    if args_count == 3 {
                        error_range_start = p.cur_tok().start();
                    }

                    if p.at(T![...]) {
                        let err = p
                            .err_builder("`...` is not allowed in `import()`")
                            .primary(p.cur_tok().range(), "");
                        p.error(err);
                    } else {
                        parse_assignment_expression_or_higher(
                            p,
                            context
                                .and_include_in(true)
                                .and_object_expression_allowed(true),
                        )
                        .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
                    }

                    if p.at(T![,]) {
                        p.bump_any();
                    } else {
                        break;
                    }
                }

                args_list.complete(p, JS_CALL_ARGUMENT_LIST);
                if args_count == 0 || args_count > 2 {
                    let err = p
                        .err_builder("`import()` requires exactly one or two arguments. ")
                        .primary(error_range_start..p.cur_tok().end(), "");
                    p.error(err);
                }

                p.expect(T![')']);
                args.complete(p, JS_CALL_ARGUMENTS);
                m.complete(p, JS_IMPORT_CALL_EXPRESSION)
            } else {
                return Absent;
            }
        }
        T![new] => parse_new_expr(p, context).unwrap(),

        BACKTICK => {
            let m = p.start();
            parse_template_literal(p, m, false)
        }
        ERROR_TOKEN => {
            let m = p.start();
            p.bump_any();
            m.complete(p, JS_UNKNOWN)
        }
        // test_err primary_expr_invalid_recovery
        // let a = \; foo();
        _ => {
            return Absent;
        }
    };

    Present(complete)
}

fn parse_identifier_expression(p: &mut Parser) -> ParsedSyntax {
    parse_reference_identifier(p)
        .map(|identifier| identifier.precede(p).complete(p, JS_IDENTIFIER_EXPRESSION))
}

// test_err identifier
// yield;
// await;
pub(crate) fn parse_reference_identifier(p: &mut Parser) -> ParsedSyntax {
    parse_identifier(p, JS_REFERENCE_IDENTIFIER)
}

pub(crate) fn is_nth_at_reference_identifier(p: &Parser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
}

// test identifier_loose_mode
// // SCRIPT
// foo;
// yield;
// await;
//
// test identifier
// foo;
//
// test_err identifier_err
// yield;
// await;
// async function test(await) {}
// function* test(yield) {}
// enum;
// implements;
// interface;

/// Parses an identifier if it is valid in this context or returns `Invalid` if the context isn't valid in this context.
/// An identifier is invalid if:
/// * It is named `await` inside of an async function
/// * It is named `yield` inside of a generator function or in strict mode
pub(super) fn parse_identifier(p: &mut Parser, kind: JsSyntaxKind) -> ParsedSyntax {
    match p.cur() {
        T![yield] | T![await] | T![ident] | T![enum] => {
            let m = p.start();
            let name = p.cur_src();

            let error = match name {
                "await" if p.state.in_async() => Some(
                    p.err_builder("Illegal use of `await` as an identifier in an async context")
                        .primary(p.cur_tok().range(), ""),
                ),
                "await" if p.syntax.file_kind == FileKind::Module => Some(
                    p.err_builder("Illegal use of `await` as an identifier inside of a module")
                        .primary(p.cur_tok().range(), ""),
                ),
                "yield" if p.state.in_generator() => Some(
                    p.err_builder("Illegal use of `yield` as an identifier in generator function")
                        .primary(p.cur_tok().range(), ""),
                ),

                "yield" | "let" | "public" | "protected" | "private" | "package" | "implements"
                | "interface" | "static"
                    if StrictMode.is_supported(p) =>
                {
                    Some(
                        p.err_builder(&format!(
                            "Illegal use of reserved keyword `{}` as an identifier in strict mode",
                            name
                        ))
                        .primary(p.cur_tok().range(), ""),
                    )
                }
                _ if p.cur() == T![enum] => Some(
                    p.err_builder("Illegal use of reserved keyword `enum` as an identifier")
                        .primary(p.cur_tok().range(), ""),
                ),
                _ => None,
            };

            p.bump_remap(T![ident]);
            let mut identifier = m.complete(p, kind);

            if let Some(error) = error {
                p.error(error);
                identifier.change_kind(p, kind.to_unknown());
            }

            Present(identifier)
        }
        _ => Absent,
    }
}

#[inline]
pub(crate) fn is_at_identifier(p: &Parser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &Parser, n: usize) -> bool {
    matches!(p.nth(n), T![ident] | T![await] | T![yield] | T![enum])
}

#[inline]
pub(crate) fn is_nth_at_identifier_or_keyword(p: &Parser, n: usize) -> bool {
    p.nth(n).is_keyword() || is_nth_at_identifier(p, n)
}

/// A template literal such as "`abcd ${efg}`"
// test template_literal
// let a = `foo ${bar}`;
// let b = ``;
// let c = `${foo}`;
// let d = `foo`;
// let e = `${{ a: "string" }}`;

// test_err template_literal
// let a = `foo ${}`
// let b = `${a a}`
fn parse_template_literal(
    p: &mut Parser,
    marker: Marker,
    in_optional_chain: bool,
) -> CompletedMarker {
    debug_assert!(p.at(BACKTICK));

    p.expect(BACKTICK);
    let elements_list = p.start();
    parse_template_elements(p, JS_TEMPLATE_CHUNK_ELEMENT, JS_TEMPLATE_ELEMENT, |p| {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression)
    });

    elements_list.complete(p, JS_TEMPLATE_ELEMENT_LIST);

    // test_err template_literal_unterminated
    // let a = `${foo} bar

    // The lexer already should throw an error for unterminated template literal
    p.eat(BACKTICK);
    let mut completed = marker.complete(p, JS_TEMPLATE);

    // test_err template_after_optional_chain
    // obj.val?.prop`template`
    // obj.val?.[expr]`template`
    // obj.func?.(args)`template`
    if in_optional_chain {
        p.error(
            p.err_builder("Tagged template expressions are not permitted in an optional chain.")
                .primary(completed.range(p), ""),
        );
        completed.change_kind(p, JS_UNKNOWN_EXPRESSION);
    }

    completed
}

#[inline]
pub(crate) fn parse_template_elements<P>(
    p: &mut Parser,
    chunk_kind: JsSyntaxKind,
    element_kind: JsSyntaxKind,
    parse_expression: P,
) where
    P: Fn(&mut Parser) -> Option<CompletedMarker>,
{
    while !p.at(EOF) && !p.at(BACKTICK) {
        match p.cur() {
            TEMPLATE_CHUNK => {
                let m = p.start();
                p.bump_any();
                m.complete(p, chunk_kind);
            },
            DOLLAR_CURLY => {
                let e = p.start();
                p.bump_any();

                parse_expression(p);

                if !p.expect(T!['}']) {
                    // Seems there's more. For example a `${a a}`. We must eat all tokens away to avoid a panic because of an unexpected token
                    if ParseRecovery::new(JS_UNKNOWN, token_set![T!['}'], TEMPLATE_CHUNK, DOLLAR_CURLY, ERROR_TOKEN, BACKTICK]).recover(p).is_ok() {
                        p.eat(T!['}']); // eat the closing paren if we successfully recovered
                    }
                }
                e.complete(p, element_kind);
            }
            ERROR_TOKEN => {
                let err = p.err_builder("Invalid template literal")
                    .primary(p.cur_tok().range(), "");
                p.err_and_bump(err, JsSyntaxKind::JS_UNKNOWN);
            }
            t => unreachable!("Anything not template chunk or dollarcurly should have been eaten by the lexer, but {:?} was found", t),
        }
    }
}

struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        match p.cur() {
            T![...] => parse_spread_element(p, ExpressionContext::default()),
            T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
            _ => parse_assignment_expression_or_higher(p, ExpressionContext::default()),
        }
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_EXPRESSION,
                EXPR_RECOVERY_SET.union(token_set!(T![']'])),
            ),
            js_parse_error::expected_array_element,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_ARRAY_ELEMENT_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An array literal such as `[foo, bar, ...baz]`.
// test array_expr
// [foo, bar];
// [foo];
// [,foo];
// [foo,];
// [,,,,,foo,,,,];
// [...a, ...b];
fn parse_array_expr(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['[']);

    // test array_element_in_expr
    // for(["a" in {}];;) {}
    ArrayElementsList.parse_list(p);

    p.expect(T![']']);
    Present(m.complete(p, JS_ARRAY_EXPRESSION))
}

// test_err spread
// [...]
/// A spread element consisting of three dots and an assignment expression such as `...foo`
fn parse_spread_element(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![...]);
    parse_assignment_expression_or_higher(p, context)
        .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
    Present(m.complete(p, JS_SPREAD))
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub(super) fn parse_lhs_expr(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    // super.foo and super[bar]
    // test super_property_access
    // super.foo
    // super[bar]
    // super[foo][bar]
    let lhs = if p.at(T![super]) {
        parse_super_expression(p)
    } else {
        parse_member_expression_or_higher(p, context)
    };

    lhs.map(|lhs_marker| parse_call_expression_rest(p, lhs_marker, context))
}

fn parse_call_expression_rest(
    p: &mut Parser,
    lhs: CompletedMarker,
    context: ExpressionContext,
) -> CompletedMarker {
    let mut lhs = lhs;
    let mut in_optional_chain = false;
    loop {
        lhs = parse_member_expression_rest(p, lhs, context, true, &mut in_optional_chain);

        let m = lhs.precede(p);
        let optional_chain_call = p.eat(T![?.]);
        in_optional_chain = in_optional_chain || optional_chain_call;

        // test ts ts_call_expr_with_type_arguments
        // function a<A, B, C>() {}
        // a<A, B, C>();
        // (() => { a }).a<A, B, C>()
        // (() => a)<A, B, C>();
        if TypeScript.is_supported(p) && p.at(T![<]) {
            // rewinds automatically if not a valid type arguments
            let type_arguments = parse_ts_type_arguments_in_expression(p).ok();

            if type_arguments.is_some() {
                if p.at(BACKTICK) {
                    // test ts ts_tagged_template_literal
                    // html<A, B>`abcd`
                    // html<A, B>`abcd`._string
                    lhs = parse_template_literal(p, m, optional_chain_call);
                    continue;
                }

                parse_arguments(p, context).or_add_diagnostic(p, expected_parameters);
                lhs = m.complete(p, JS_CALL_EXPRESSION);
                continue;
            }
        } else if p.at(T!['(']) {
            parse_arguments(p, context).or_add_diagnostic(p, expected_parameters);
            lhs = m.complete(p, JS_CALL_EXPRESSION);
            continue;
        }

        if optional_chain_call {
            p.error(expected_identifier(p, p.cur_tok().range()));
        }

        m.abandon(p);
        break;
    }

    lhs
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test postfix_expr
// foo++
// foo--
fn parse_postfix_expr(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let lhs = parse_lhs_expr(p, context);
    lhs.map(|marker| {
        if !p.has_linebreak_before_n(0) {
            // test post_update_expr
            // foo++
            // foo--
            match p.cur() {
                T![++] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![++]);
                    m.complete(p, JS_POST_UPDATE_EXPRESSION)
                }
                T![--] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![--]);
                    m.complete(p, JS_POST_UPDATE_EXPRESSION)
                }
                _ => marker,
            }
        } else {
            marker
        }
    })
}

/// A unary expression such as `!foo` or `++bar`
pub(super) fn parse_unary_expr(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    const UNARY_SINGLE: TokenSet =
        token_set![T![delete], T![void], T![typeof], T![+], T![-], T![~], T![!]];

    if (p.state.in_async()) && p.at(T![await]) {
        // test await_expression
        // async function test() {
        //   await inner();
        //   await (inner()) + await inner();
        // }
        // async function inner() {
        //   return 4;
        // }
        // await test();

        // test_err no_top_level_await_in_scripts
        // // SCRIPT
        // async function test() {}
        // await test();
        let m = p.start();
        p.bump(T![await]);

        parse_unary_expr(p, context)
            .or_add_diagnostic(p, js_parse_error::expected_unary_expression);

        let mut expr = m.complete(p, JS_AWAIT_EXPRESSION);

        if !p.state.is_top_level() && !p.state.in_function() {
            // test_err await_in_parameter_initializer
            // async function test(a = await b()) {}
            // function test2(a = await b()) {}

            // test_err await_in_static_initialization_block_member
            // // SCRIPT
            // class A {
            //   static {
            //     await;
            //   }
            // }
            p.error(
                p.err_builder("`await` is only allowed within async functions and at the top levels of modules.")
                    .primary(expr.range(p), ""),
            );
            expr.change_to_unknown(p);
        }

        return Present(expr);
    }

    if p.at(T![<]) {
        return TypeScript.parse_exclusive_syntax(
            p,
            |p| parse_ts_type_assertion_expression(p, context),
            |p, assertion| ts_only_syntax_error(p, "type assertion", assertion.range(p).as_range()),
        );
    }

    // test pre_update_expr
    // ++foo
    // --foo
    if p.at(T![++]) {
        let m = p.start();
        p.bump(T![++]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }
    if p.at(T![--]) {
        let m = p.start();
        p.bump(T![--]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }

    // test js_unary_expressions
    // delete a['test'];
    // void a;
    // typeof a;
    // +1;
    // -1;
    // ~1;
    // !true;
    // -a + -b + +a;

    // test_err unary_expr
    // ++ ;
    // -- ;
    // -;

    if p.at_ts(UNARY_SINGLE) {
        let m = p.start();
        let op = p.cur();
        p.bump_any();

        // test unary_delete
        // delete obj.key;
        // delete (obj).key;
        // delete obj.#member.key;
        // delete (obj.#member).key;
        // delete func().#member.key;
        // delete (func().#member).key;
        // delete obj?.#member.key;
        // delete (obj?.#member).key;
        // delete obj?.inner.#member.key;
        // delete (obj?.inner.#member).key;
        // delete obj[key];
        // delete (obj)[key];
        // delete obj.#member[key];
        // delete (obj.#member)[key];
        // delete func().#member[key];
        // delete (func().#member)[key];
        // delete obj?.#member[key];
        // delete (obj?.#member)[key];
        // delete obj?.inner.#member[key];
        // delete (obj?.inner.#member)[key];
        // delete (obj.#key, obj.key);
        // delete (#key in obj);

        // test unary_delete_nested
        // class TestClass { #member = true; method() { delete func(this.#member) } }
        // class TestClass { #member = true; method() { delete [this.#member] } }
        // class TestClass { #member = true; method() { delete { key: this.#member } } }
        // class TestClass { #member = true; method() { delete (() => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (param => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (async () => { this.#member; }) } }

        // test_err unary_delete
        // delete ident;
        // delete obj.#member;
        // delete func().#member;
        // delete obj?.#member;
        // delete obj?.inner.#member;

        // test_err unary_delete_parenthesized
        // delete (ident);
        // delete ((ident));
        // delete (obj.key, ident);
        // delete (obj.#member);
        // delete (func().#member);
        // delete (obj?.#member);
        // delete (obj?.inner.#member);
        // delete (obj.key, obj.#key);

        let is_delete = op == T![delete];
        let mut kind = JS_UNARY_EXPRESSION;

        let res = if is_delete {
            let checkpoint = p.checkpoint();
            parse_unary_expr(p, context).ok();

            let mut rewriter = DeleteExpressionRewriter::default();
            rewrite_events(&mut rewriter, checkpoint, p);

            rewriter.result.take().map(|res| {
                if StrictMode.is_supported(p) {
                    if let Some(range) = rewriter.exited_ident_expr {
                        kind = JS_UNKNOWN_EXPRESSION;
                        p.error(
                            p.err_builder(
                                "the target for a delete operator cannot be a single identifier",
                            )
                            .primary(range, ""),
                        );
                    }
                }

                if let Some(range) = rewriter.exited_private_member_expr {
                    kind = JS_UNKNOWN_EXPRESSION;
                    p.error(
                        p.err_builder(
                            "the target for a delete operator cannot be a private member",
                        )
                        .primary(range, ""),
                    );
                }

                res
            })
        } else {
            parse_unary_expr(p, context).ok()
        };

        if is_delete && kind != JS_UNKNOWN_EXPRESSION && p.typescript() {
            if let Some(res) = res {
                match res.kind() {
                    JS_STATIC_MEMBER_EXPRESSION | JS_COMPUTED_MEMBER_EXPRESSION => {}
                    _ => {
                        kind = JS_UNKNOWN_EXPRESSION;
                        p.error(
                            p.err_builder(
                                "the target for a delete operator must be a property access",
                            )
                            .primary(res.range(p), ""),
                        );
                    }
                }
            }
        }

        return Present(m.complete(p, kind));
    }

    parse_postfix_expr(p, context)
}

#[derive(Default)]
struct DeleteExpressionRewriter {
    stack: Vec<(Marker, JsSyntaxKind)>,
    result: Option<CompletedMarker>,
    /// Set to true immediately after the rewriter exits an identifier expression
    exited_ident_expr: Option<TextRange>,
    /// Set to true immediately after the rewriter exits a private name
    exited_private_name: bool,
    /// Set to true immediately after the rewriter exits a member expresison with a private name
    exited_private_member_expr: Option<TextRange>,
}

impl RewriteParseEvents for DeleteExpressionRewriter {
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut Parser) {
        self.stack.push((p.start(), kind));
        self.exited_ident_expr.take();
        self.exited_private_name = false;
        self.exited_private_member_expr.take();
    }

    fn finish_node(&mut self, p: &mut Parser) {
        let (m, kind) = self.stack.pop().expect("stack depth mismatch");
        let node = m.complete(p, kind);

        if kind != JS_PARENTHESIZED_EXPRESSION && kind != JS_SEQUENCE_EXPRESSION {
            self.exited_private_member_expr =
                if self.exited_private_name && kind == JS_STATIC_MEMBER_EXPRESSION {
                    Some(node.range(p))
                } else {
                    None
                };

            self.exited_ident_expr = if kind == JS_IDENTIFIER_EXPRESSION {
                Some(node.range(p))
            } else {
                None
            };

            self.exited_private_name = kind == JS_PRIVATE_NAME;
        }

        self.result = Some(node);
    }
}

pub(super) fn is_at_name(p: &Parser) -> bool {
    is_nth_at_name(p, 0)
}

pub(super) fn is_nth_at_name(p: &Parser, offset: usize) -> bool {
    p.nth_at(offset, T![ident]) || p.nth(offset).is_keyword()
}

pub(super) fn is_nth_at_any_name(p: &Parser, n: usize) -> bool {
    is_nth_at_name(p, n) || p.nth_at(n, T![#])
}
