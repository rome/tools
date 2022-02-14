use crate::event::{rewrite_events, RewriteParseEvents};
use crate::parser::{expected_any, ParsedSyntax, ToDiagnostic};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{
    is_at_identifier, parse_conditional_expr, parse_unary_expr, ExpressionContext,
};
use crate::syntax::js_parse_error::{
    expected_assignment_target, expected_identifier, expected_object_member_name,
};
use crate::syntax::object::{is_at_object_member_name, parse_object_member_name};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Parser};
use crate::{JsSyntaxKind::*, *};

// test assignment_target
// foo += bar = b ??= 3;
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count === 3
// a['b'] = c[d] = "test"

// test_err invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;
// a?.b = b;
// a?.["b"] = b;
// (a +) = b;

// test ts ts_non_null_assignment
// let a;
// a! &= 2;
// let b = { a: null };
// b.a! &= 5

/// Converts the passed in lhs expression to an assignment pattern
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_pattern(
    p: &mut Parser,
    target: CompletedMarker,
    checkpoint: Checkpoint,
) -> CompletedMarker {
    match target.kind() {
        JS_OBJECT_EXPRESSION => {
            p.rewind(checkpoint);
            ObjectAssignmentPattern.parse_object_pattern(p).unwrap()
        }
        JS_ARRAY_EXPRESSION => {
            p.rewind(checkpoint);
            ArrayAssignmentPattern.parse_array_pattern(p).unwrap()
        }
        _ => expression_to_assignment(p, target, checkpoint),
    }
}

// test array_or_object_member_assignment
// [{
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42] = [23];
// ({ x: {
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42 } = { x: 23 });
pub(crate) fn parse_assignment_pattern(p: &mut Parser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = parse_conditional_expr(p, ExpressionContext::default());

    assignment_expression
        .map(|expression| expression_to_assignment_pattern(p, expression, checkpoint))
}

/// Re-parses an expression as an assignment.
pub(crate) fn expression_to_assignment(
    p: &mut Parser,
    target: CompletedMarker,
    checkpoint: Checkpoint,
) -> CompletedMarker {
    try_expression_to_assignment(p, target, checkpoint).unwrap_or_else(|checkpoint| {
        // Doesn't seem to be a valid assignment target. Recover and create an error.
        let expression_end = p.token_pos();
        p.rewind(checkpoint);
        wrap_expression_in_invalid_assignment(p, expression_end)
    })
}

pub(crate) enum AssignmentExprPrecedence {
    Unary,
    Conditional,
}

impl AssignmentExprPrecedence {
    fn parse_expression(&self, p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
        match self {
            AssignmentExprPrecedence::Unary => parse_unary_expr(p, context),
            AssignmentExprPrecedence::Conditional => parse_conditional_expr(p, context),
        }
    }
}

pub(crate) fn parse_assignment(
    p: &mut Parser,
    expr_kind: AssignmentExprPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = expr_kind.parse_expression(p, context);

    assignment_expression.map(|expr| expression_to_assignment(p, expr, checkpoint))
}

struct AssignmentPatternWithDefault;

impl ParseWithDefaultPattern for AssignmentPatternWithDefault {
    #[inline]
    fn pattern_with_default_kind() -> JsSyntaxKind {
        JS_ASSIGNMENT_WITH_DEFAULT
    }

    #[inline]
    fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
        expected_assignment_target(p, range)
    }

    #[inline]
    fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        parse_assignment_pattern(p)
    }
}

struct ArrayAssignmentPattern;

// test array_assignment_target
// [foo, bar] = baz;
// [,,,b,,c,] = baz;
// [a = "test", a.b, call().b] = baz;
// [((a))] = baz;
//
// test_err array_assignment_target_err
// [a a, ++b, ] = test;
// [a, c, ...rest,] = test;
// [a = , = "test"] = test;
// [[a b] [c]]= test;
// [a: b] = c
impl ParseArrayPattern<AssignmentPatternWithDefault> for ArrayAssignmentPattern {
    #[inline]
    fn unknown_pattern_kind() -> JsSyntaxKind {
        JS_UNKNOWN_ASSIGNMENT
    }

    #[inline]
    fn array_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN
    }

    // test array_assignment_target_rest
    // ([ ...abcd ] = a);
    // ([ ...(abcd) ] = a);
    // ([ ...m.test ] = c);
    // ([ ...m[call()] ] = c);
    // ([ ...any.expression().b ] = c);
    // ([ ...[x, y] ] = b);
    // ([ ...[ ...a ] ] = c);
    //
    // test_err array_assignment_target_rest_err
    // ([ ... ] = a);
    // ([ ...c = "default" ] = a);
    // ([ ...rest, other_assignment ] = a);
    #[inline]
    fn rest_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
    }

    fn list_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST
    }

    #[inline]
    fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic {
        expected_any(&["assignment target", "rest element", "comma"], range).to_diagnostic(p)
    }

    #[inline]
    fn pattern_with_default(&self) -> AssignmentPatternWithDefault {
        AssignmentPatternWithDefault
    }
}

struct ObjectAssignmentPattern;

// test object_assignment_target
// ({} = {});
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
impl ParseObjectPattern for ObjectAssignmentPattern {
    #[inline]
    fn unknown_pattern_kind() -> JsSyntaxKind {
        JS_UNKNOWN_ASSIGNMENT
    }

    #[inline]
    fn object_pattern_kind() -> JsSyntaxKind {
        JS_OBJECT_ASSIGNMENT_PATTERN
    }

    fn list_kind() -> JsSyntaxKind {
        JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST
    }

    #[inline]
    fn expected_property_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
        expected_any(&["assignment target", "rest property"], range).to_diagnostic(p)
    }

    // test property_assignment_target
    // ({x}= {});
    // ({x: y}= {});
    // ({x: y.test().z}= {});
    // ({x: ((z))}= {});
    // ({x: z["computed"]}= {});
    // ({x = "default"}= {});
    // ({x: y = "default"}= {});
    // ({0: y, [computed]: z} = {});
    //
    // test_err property_assignment_target_err
    // ({:y} = {});
    // ({=y} = {});
    // ({:="test"} = {});
    // ({:=} = {});
    // ({ a b } = {});
    fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        let m = p.start();

        let kind = if (is_at_identifier(p) || p.at(T![=])) && !p.nth_at(1, T![:]) {
            parse_assignment(
                p,
                AssignmentExprPrecedence::Conditional,
                ExpressionContext::default(),
            )
            .or_add_diagnostic(p, expected_identifier);
            JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
        } else if is_at_object_member_name(p) || p.at(T![:]) || p.nth_at(1, T![:]) {
            parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
            p.expect(T![:]);
            parse_assignment_pattern(p).or_add_diagnostic(p, expected_assignment_target);
            JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
        } else {
            m.abandon(p);
            return Absent;
        };

        parse_initializer_clause(p, ExpressionContext::default()).ok();

        Present(m.complete(p, kind))
    }

    // test rest_property_assignment_target
    // ({ ...abcd } = a);
    // ({ ...(abcd) } = a);
    // ({ ...m.test } = c);
    // ({ ...m[call()] } = c);
    // ({ ...any.expression().b } = c);
    // ({ b: { ...a } } = c);
    //
    // test_err rest_property_assignment_target_err
    // ({ ... } = a);
    // ({ ...c = "default" } = a);
    // ({ ...{a} } = b);
    // ({ ...rest, other_assignment } = a);
    // ({ ...rest, } = a);
    fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
        if !p.at(T![...]) {
            return Absent;
        }

        let m = p.start();
        p.bump(T![...]);

        let target = parse_assignment_pattern(p).or_add_diagnostic(p, expected_assignment_target);

        if let Some(mut target) = target {
            if matches!(
                target.kind(),
                JS_OBJECT_ASSIGNMENT_PATTERN | JS_ARRAY_ASSIGNMENT_PATTERN
            ) {
                target.change_kind(p, JS_UNKNOWN_ASSIGNMENT);
                p.error(
                    p.err_builder(
                        "object and array assignment targets are not allowed in rest patterns",
                    )
                    .primary(target.range(p), ""),
                );
            }
        }

        Present(m.complete(p, JS_OBJECT_ASSIGNMENT_PATTERN_REST))
    }
}

fn try_expression_to_assignment(
    p: &mut Parser,
    target: CompletedMarker,
    checkpoint: Checkpoint,
) -> Result<CompletedMarker, Checkpoint> {
    if !matches!(
        target.kind(),
        JS_PARENTHESIZED_EXPRESSION
            | JS_STATIC_MEMBER_EXPRESSION
            | JS_COMPUTED_MEMBER_EXPRESSION
            | JS_IDENTIFIER_EXPRESSION
            | TS_NON_NULL_ASSERTION_EXPRESSION
    ) {
        return Err(checkpoint);
    }

    // At this point it's guaranteed that the root node can be mapped to a assignment
    // but it's not yet guaranteed if it is valid or not (for example, a static member expression
    // is valid, except if it uses optional chaining).
    let mut reparse_assignment = ReparseAssignment::new();
    rewrite_events(&mut reparse_assignment, checkpoint, p);

    Ok(reparse_assignment.result.unwrap())
}

struct ReparseAssignment {
    // Stores the unfinished parents
    // Index 0: Re-mapped kind of the node
    // Index 1: Started marker. A `None` marker means that this node should be dropped
    //          from the re-written tree
    parents: Vec<(JsSyntaxKind, Option<Marker>)>,
    // Stores the completed assignment node (valid or invalid).
    result: Option<CompletedMarker>,
    // Tracks if the visitor is still inside of an assignment
    inside_assignment: bool,
    // Tracks if the visitor is inside a member expression or computed expression
    // for eval/arguments validation
    inside_member_or_computed_expression: bool,
}

impl ReparseAssignment {
    pub fn new() -> Self {
        Self {
            parents: Vec::default(),
            result: None,
            inside_assignment: true,
            inside_member_or_computed_expression: false,
        }
    }
}

/// Rewrites expressions to assignments
/// * Converts parenthesized expression to parenthesized assignment
/// * Converts computed/static member expressions to computed/static member assignment.
///   Validates that the operator isn't `?.` .
/// * Converts identifier expressions to identifier assignment, drops the inner reference identifier
impl RewriteParseEvents for ReparseAssignment {
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut Parser) {
        if !self.inside_assignment {
            self.parents.push((kind, Some(p.start())));
            return;
        }

        let mapped_kind = match kind {
            JS_PARENTHESIZED_EXPRESSION => JS_PARENTHESIZED_ASSIGNMENT,
            JS_STATIC_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                self.inside_member_or_computed_expression = true;
                JS_STATIC_MEMBER_ASSIGNMENT
            }
            JS_COMPUTED_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                self.inside_member_or_computed_expression = true;
                JS_COMPUTED_MEMBER_ASSIGNMENT
            }
            JS_IDENTIFIER_EXPRESSION => JS_IDENTIFIER_ASSIGNMENT,
            TS_NON_NULL_ASSERTION_EXPRESSION => TS_NON_NULL_ASSERTION_ASSIGNMENT,
            JS_REFERENCE_IDENTIFIER => {
                self.parents.push((kind, None)); // Omit reference identifiers
                return;
            }
            _ => {
                self.inside_assignment = false;
                JS_UNKNOWN_ASSIGNMENT
            }
        };

        self.parents.push((mapped_kind, Some(p.start())));
    }

    fn finish_node(&mut self, p: &mut Parser) {
        let (kind, m) = self.parents.pop().unwrap();

        if let Some(m) = m {
            let completed = m.complete(p, kind);

            if kind == JS_UNKNOWN_ASSIGNMENT {
                p.error(invalid_assignment_error(p, completed.range(p)));
            }
            self.result = Some(completed);
        }
    }

    fn token(&mut self, kind: JsSyntaxKind, p: &mut Parser) {
        let parent = self.parents.last_mut();

        if let Some((parent_kind, _)) = parent {
            if matches!(
                *parent_kind,
                JS_COMPUTED_MEMBER_ASSIGNMENT | JS_STATIC_MEMBER_ASSIGNMENT
            ) && kind == T![?.]
            {
                *parent_kind = JS_UNKNOWN_ASSIGNMENT
            }

            let src_str = p.cur_src();
            if kind == IDENT
                && (src_str == "eval" || src_str == "arguments")
                && p.state.strict().is_some()
                // If we're inside a member or computed expression then we do not error
                && !self.inside_member_or_computed_expression
            {
                // Cloning because cannot keep immutable ref to p
                // and mutable ref with p.error()
                let src = src_str.to_string();

                *parent_kind = JS_UNKNOWN;
                p.error(
                    p.err_builder(&format!(
                        "Illegal use of `{}` as an identifier in strict mode",
                        src
                    ))
                    .primary(p.cur_tok().range(), ""),
                );
            }
        }

        p.bump_remap(kind);
    }
}

fn wrap_expression_in_invalid_assignment(p: &mut Parser, expression_end: usize) -> CompletedMarker {
    let unknown = p.start();
    // Eat all tokens until we reached the end of the original expression. This is better than
    // any other error recovery because it's already know where the expression ends.
    while p.token_pos() < expression_end {
        p.bump_any();
    }

    let completed = unknown.complete(p, JS_UNKNOWN_ASSIGNMENT);

    p.error(invalid_assignment_error(p, completed.range(p)));

    completed
}

fn invalid_assignment_error(p: &Parser, range: TextRange) -> Diagnostic {
    p.err_builder(&format!("Invalid assignment to `{}`", p.source(range)))
        .primary(range, "This expression cannot be assigned to")
}
