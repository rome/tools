use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::*;
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow comparison of expressions modifying the string case with non-compliant value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (s.toUpperCase() === "Abc") {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (s.toLowerCase() === "Abc") {}
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// if (s.toUpperCase() === "ABC") {}
    /// while (s.toLowerCase() === "abc") {}
    /// for (;s.toLocaleLowerCase() === "ABC";) {}
    /// while (s.toLocaleUpperCase() === "abc") {}
    /// for (let s = "abc"; s === "abc"; s = s.toUpperCase()) {}
    /// ```
    pub(crate) NoStringCaseMismatch {
        version: "11.0.0",
        name: "noStringCaseMismatch",
        recommended: false,
    }
}

impl Rule for NoStringCaseMismatch {
    type Query = Ast<QueryCandidate>;
    type State = CaseMismatchInfo;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let query = ctx.query();
        match query {
            QueryCandidate::JsBinaryExpression(expr) => CaseMismatchInfo::from_binary_expr(expr)
                .into_iter()
                .collect(),
            QueryCandidate::JsSwitchStatement(stmt) => CaseMismatchInfo::from_switch_stmt(stmt),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        let mut diagnostic = match query {
            QueryCandidate::JsBinaryExpression(expr) => RuleDiagnostic::new(
                rule_category!(),
                expr.range(),
                markup! { "This expression always returns false." },
            ),
            QueryCandidate::JsSwitchStatement(..) => RuleDiagnostic::new(
                rule_category!(),
                state.literal.range(),
                markup! { "This case will never match." },
            ),
        };
        diagnostic = diagnostic
            .detail(
                state.call.range(),
                markup! {
                    "This call convert the string to " { state.expected_case.description() }
                },
            )
            .detail(
                state.literal.range(),
                markup! {
                    "... but this value is not in " { state.expected_case.description() }
                },
            );
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            state.literal.clone(),
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal(
                        &state.expected_value,
                    )),
                ),
            ),
        );
        Some(JsRuleAction {
            mutation,
            message: markup! {"Use "<Emphasis>{state.expected_case.description()}</Emphasis>" string value."}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
    }
}

declare_node_union! {
    pub(crate) QueryCandidate = JsBinaryExpression | JsSwitchStatement
}

pub(crate) struct CaseMismatchInfo {
    expected_case: StringCase,
    expected_value: String,
    call: JsCallExpression,
    literal: JsAnyExpression,
}

impl CaseMismatchInfo {
    fn from_binary_expr(expr: &JsBinaryExpression) -> Option<Self> {
        let JsBinaryExpressionFields { left: Ok(left), right: Ok(right), operator_token: Ok(op) } = expr.as_fields() else { return None; };
        if !matches!(op.kind(), JsSyntaxKind::EQ2 | JsSyntaxKind::EQ3) {
            return None;
        }
        let (call, literal) = match (left, right) {
            (JsAnyExpression::JsCallExpression(call), other)
            | (other, JsAnyExpression::JsCallExpression(call)) => (call, other),
            _ => return None,
        };
        Self::compare_call_with_literal(call, literal)
    }

    fn from_switch_stmt(stmt: &JsSwitchStatement) -> Vec<Self> {
        let JsSwitchStatementFields { discriminant: Ok(JsAnyExpression::JsCallExpression(call)), cases, .. } = stmt.as_fields() else {
             return Vec::new();
        };

        cases
            .into_iter()
            .flat_map(|case| case.as_js_case_clause().and_then(|case| case.test().ok()))
            .flat_map(|test| Self::compare_call_with_literal(call.clone(), test))
            .collect()
    }

    fn compare_call_with_literal(call: JsCallExpression, literal: JsAnyExpression) -> Option<Self> {
        let expected_case = get_string_case_modification(&call)?;
        let literal_value = literal.as_string_constant()?;
        let expected_value = expected_case.convert(&literal_value);
        if literal_value == expected_value {
            return None;
        }
        Some(Self {
            expected_case,
            expected_value,
            call,
            literal,
        })
    }
}

fn get_string_case_modification(call: &JsCallExpression) -> Option<StringCase> {
    if call.arguments().ok()?.args().len() != 0 {
        return None;
    }

    let callee = call.callee().ok()?;
    let member_expr = JsAnyMemberExpression::cast_ref(callee.syntax())?;
    if member_expr.has_member_name("toLowerCase") {
        return Some(StringCase::Lower);
    }

    if member_expr.has_member_name("toUpperCase") {
        return Some(StringCase::Upper);
    }

    None
}

enum StringCase {
    Upper,
    Lower,
}

impl StringCase {
    fn convert(&self, s: &str) -> String {
        match self {
            StringCase::Upper => s.to_uppercase(),
            StringCase::Lower => s.to_lowercase(),
        }
    }

    fn description(&self) -> &str {
        match self {
            StringCase::Upper => "upper case",
            StringCase::Lower => "lower case",
        }
    }
}
