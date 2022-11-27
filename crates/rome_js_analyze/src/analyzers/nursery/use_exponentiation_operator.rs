use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::{make, syntax::T};
use rome_js_syntax::{AnyJsExpression, JsBinaryOperator, JsCallExpression, OperatorPrecedence};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Disallow the use of `Math.pow` in favor of the `**` operator.
    ///
    /// > Introduced in ES2016, the infix exponentiation operator ** is an alternative for the standard Math.pow function.
    /// > Infix notation is considered to be more readable and thus more preferable than the function notation.
    ///
    /// Source: https://eslint.org/docs/latest/rules/prefer-exponentiation-operator
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = Math.pow(2, 8);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const bar = Math.pow(a, b);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let baz = Math.pow(a + b, c + d);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let quux = Math.pow(-1, n);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 2 ** 8;
    ///
    /// const bar = a ** b;
    ///
    /// let baz = (a + b) ** (c + d);
    ///
    /// let quux = (-1) ** n;
    /// ```
    ///
    pub(crate) UseExponentiationOperator {
        version: "11.0.0",
        name: "useExponentiationOperator",
        recommended: false,
    }
}

pub struct MathPowCall {
    base: AnyJsExpression,
    exponent: AnyJsExpression,
}

impl MathPowCall {
    fn get_base(&self) -> Option<AnyJsExpression> {
        Some(if self.does_base_need_parens()? {
            parenthesize_any_js_expression(&self.base)
        } else {
            self.base.clone()
        })
    }

    fn get_exponent(&self) -> Option<AnyJsExpression> {
        Some(if self.does_exponent_need_parens()? {
            parenthesize_any_js_expression(&self.exponent)
        } else {
            self.exponent.clone()
        })
    }

    /// Determines whether the base expression needs parens in an exponentiation binary expression.
    fn does_base_need_parens(&self) -> Option<bool> {
        Some(
            // '**' is right-associative, parens are needed when Math.pow(a ** b, c) is converted to (a ** b) ** c
            self.base.precedence().ok()? <= OperatorPrecedence::Exponential
				// An unary operator cannot be used immediately before an exponentiation expression
				|| self.base.as_js_unary_expression().is_some()
				|| self.base.as_js_await_expression().is_some(),
        )
    }

    /// Determines whether the exponent expression needs parens in an exponentiation binary expression.
    fn does_exponent_need_parens(&self) -> Option<bool> {
        Some(self.exponent.precedence().ok()? < OperatorPrecedence::Exponential)
    }
}

impl Rule for UseExponentiationOperator {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let has_math_pow = match node.callee().ok()?.omit_parentheses() {
            AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
                let has_math = static_member_expr
                    .object()
                    .ok()?
                    .omit_parentheses()
                    .as_reference_identifier()?
                    .has_name("Math");
                let has_pow = static_member_expr
                    .member()
                    .ok()?
                    .as_js_name()?
                    .value_token()
                    .ok()?
                    .token_text_trimmed()
                    == "pow";

                has_math && has_pow
            }
            AnyJsExpression::JsComputedMemberExpression(computed_member_expr) => {
                let has_math = computed_member_expr
                    .object()
                    .ok()?
                    .omit_parentheses()
                    .as_reference_identifier()?
                    .has_name("Math");
                let has_pow = computed_member_expr
                    .member()
                    .ok()?
                    .is_string_constant("pow");

                has_math && has_pow
            }
            _ => false,
        };

        if has_math_pow {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Use the '**' operator instead of 'Math.pow'.",
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if !should_suggest_fix(node)? {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let [base, exponent] = node.get_arguments_by_index([0, 1]);

        let math_pow_call = MathPowCall {
            base: base?.as_js_any_expression()?.clone().omit_parentheses(),
            exponent: exponent?.as_js_any_expression()?.clone().omit_parentheses(),
        };

        let new_node = make::js_binary_expression(
            math_pow_call.get_base()?,
            make::token(T![**]),
            math_pow_call.get_exponent()?,
        );

        if let Some(parent) = does_parent_expression_need_parens(node) {
            mutation.replace_node(
                AnyJsExpression::from(node.clone()),
                parenthesize_any_js_expression(&AnyJsExpression::from(new_node)),
            );
            mutation.replace_node(parent.clone(), parenthesize_any_js_expression(&parent));
        } else {
            mutation.replace_node(
                AnyJsExpression::from(node.clone()),
                AnyJsExpression::from(new_node),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use the '**' operator instead of 'Math.pow'." }.to_owned(),
            mutation,
        })
    }
}

/// Verify if the autofix is safe to be applied and won't remove comments.
/// Argument list is considered valid if there's no spread arg and leading/trailing comments.
fn should_suggest_fix(node: &JsCallExpression) -> Option<bool> {
    let arguments = node.arguments().ok()?;
    let args_count = arguments.args().len();

    Some(
        args_count == 2
            && !arguments.l_paren_token().ok()?.has_leading_comments()
            && !arguments.l_paren_token().ok()?.has_trailing_comments()
            && !arguments.r_paren_token().ok()?.has_leading_comments()
            && !arguments.r_paren_token().ok()?.has_trailing_comments()
            && arguments.args().into_iter().flatten().all(|arg| {
                !arg.syntax().has_leading_comments()
                    && !arg.syntax().has_trailing_comments()
                    && arg.as_js_spread().is_none()
            }),
    )
}

/// Wraps a [AnyJsExpression] in paretheses
fn parenthesize_any_js_expression(expr: &AnyJsExpression) -> AnyJsExpression {
    AnyJsExpression::from(make::js_parenthesized_expression(
        make::token(T!['(']),
        expr.clone(),
        make::token(T![')']),
    ))
}

/// Determines whether the given parent node needs parens if used as the exponent in an exponentiation binary expression.
fn does_parent_expression_need_parens(node: &JsCallExpression) -> Option<AnyJsExpression> {
    let parent = node.parent::<AnyJsExpression>()?;

    let needs_parentheses = match &parent {
        // Skips already parenthesized expressions
        AnyJsExpression::JsParenthesizedExpression(_) => return None,
        AnyJsExpression::JsBinaryExpression(bin_expr) => {
            bin_expr.operator().ok()? != JsBinaryOperator::Exponent
                || bin_expr.right().ok()?.as_js_call_expression()? != node
        }
        AnyJsExpression::JsCallExpression(call_expr) => !call_expr
            .arguments()
            .ok()?
            .args()
            .iter()
            .filter_map(|arg| {
                let binding = arg.ok()?;
                return binding
                    .as_any_js_expression()?
                    .as_js_call_expression()
                    .cloned();
            })
            .any(|arg| &arg == node),
        AnyJsExpression::JsNewExpression(new_expr) => !new_expr
            .arguments()?
            .args()
            .iter()
            .filter_map(|arg| {
                let binding = arg.ok()?;
                return binding
                    .as_any_js_expression()?
                    .as_js_call_expression()
                    .cloned();
            })
            .any(|arg| &arg == node),
        AnyJsExpression::JsComputedMemberExpression(member_expr) => {
            member_expr.member().ok()?.as_js_call_expression()? != node
        }
        AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsUnaryExpression(_)
        | AnyJsExpression::JsTemplateExpression(_) => true,
        _ => false,
    };

    if needs_parentheses && parent.precedence().ok()? >= OperatorPrecedence::Exponential {
        return Some(parent);
    }

    None
}
