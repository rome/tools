use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsBinaryExpressionFields,
    JsBinaryOperator, JsUnaryOperator, TextRange,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

declare_rule! {
    "0.7.0",
    /// This rule verifies the result of `typeof $expr` unary expressions is being
    /// compared to valid values, either string literals containing valid type
    /// names or other `typeof` expressions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// typeof foo === "strnig"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo == "undefimed"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof bar != "nunber"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof bar !== "fucntion"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo === undefined
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof bar == Object
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo === baz
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo == 5
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo == -5
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// typeof foo === "string"
    /// ```
    ///
    /// ```js
    /// typeof bar == "undefined"
    /// ```
    ///
    /// ```js
    /// typeof bar === typeof qux
    /// ```
    pub(crate) UseValidTypeof = "useValidTypeof"
}

impl Rule for UseValidTypeof {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsBinaryExpression>;
    type State = (TypeofError, Option<(JsAnyExpression, JsTypeName)>);
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        let JsBinaryExpressionFields {
            left,
            operator_token: _,
            right,
        } = n.as_fields();

        if !matches!(
            n.operator().ok()?,
            JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality
        ) {
            return None;
        }

        let left = left.ok()?;
        let right = right.ok()?;

        let range = match (&left, &right) {
            // Check for `typeof $expr == $lit` and `$lit == typeof $expr`
            (
                JsAnyExpression::JsUnaryExpression(unary),
                lit @ JsAnyExpression::JsAnyLiteralExpression(literal),
            )
            | (
                lit @ JsAnyExpression::JsAnyLiteralExpression(literal),
                JsAnyExpression::JsUnaryExpression(unary),
            ) => {
                if unary.operator().ok()? != JsUnaryOperator::Typeof {
                    return None;
                }

                if let JsAnyLiteralExpression::JsStringLiteralExpression(literal) = literal {
                    let literal = literal.value_token().ok()?;
                    let range = literal.text_trimmed_range();

                    let literal = literal
                        .text_trimmed()
                        .trim_start_matches(['"', '\''])
                        .trim_end_matches(['"', '\''])
                        .to_lowercase();

                    if JsTypeName::from_str(&literal).is_some() {
                        return None;
                    }

                    // Try to fix the casing of the literal eg. "String" -> "string"
                    let suggestion = literal.to_lowercase();
                    return Some((
                        TypeofError::InvalidLiteral(range, literal),
                        JsTypeName::from_str(&suggestion).map(|type_name| (lit.clone(), type_name)),
                    ));
                }

                lit.range()
            }

            // Check for `typeof $expr == typeof $expr`
            (
                JsAnyExpression::JsUnaryExpression(left),
                JsAnyExpression::JsUnaryExpression(right),
            ) => {
                let is_typeof_left = left.operator().ok()? == JsUnaryOperator::Typeof;
                let is_typeof_right = right.operator().ok()? == JsUnaryOperator::Typeof;

                if is_typeof_left && !is_typeof_right {
                    right.range()
                } else if is_typeof_right && !is_typeof_left {
                    left.range()
                } else {
                    return None;
                }
            }

            // Check for `typeof $expr == $ident`
            (
                JsAnyExpression::JsUnaryExpression(unary),
                id @ JsAnyExpression::JsIdentifierExpression(ident),
            )
            | (
                JsAnyExpression::JsIdentifierExpression(ident),
                id @ JsAnyExpression::JsUnaryExpression(unary),
            ) => {
                if unary.operator().ok()? != JsUnaryOperator::Typeof {
                    return None;
                }

                // Try to convert the identifier to a string literal eg. String -> "string"
                let suggestion = ident.name().ok().and_then(|name| {
                    let value = name.value_token().ok()?;

                    let to_lower = value.text_trimmed().to_lowercase();
                    let as_type = JsTypeName::from_str(&to_lower)?;

                    Some((id.clone(), as_type))
                });

                return Some((TypeofError::InvalidExpression(ident.range()), suggestion));
            }

            // Check for `typeof $expr == $expr`
            (JsAnyExpression::JsUnaryExpression(unary), expr)
            | (expr, JsAnyExpression::JsUnaryExpression(unary)) => {
                if unary.operator().ok()? != JsUnaryOperator::Typeof {
                    return None;
                }

                expr.range()
            }

            _ => return None,
        };

        Some((TypeofError::InvalidExpression(range), None))
    }

    fn diagnostic(_: &RuleContext<Self>, (err, _): &Self::State) -> Option<RuleDiagnostic> {
        const TITLE: &str = "Invalid `typeof` comparison value";

        Some(match err {
            TypeofError::InvalidLiteral(range, literal) => RuleDiagnostic::warning(range, TITLE)
                .primary("not a valid type name")
                .summary(format!("{TITLE}: \"{literal}\" is not a valid type name")),
            TypeofError::InvalidExpression(range) => RuleDiagnostic::warning(range, TITLE)
                .primary("not a string literal")
                .summary(format!("{TITLE}: this expression is not a string literal",)),
        })
    }

    fn action(ctx: &RuleContext<Self>, (_, suggestion): &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();

        let (expr, type_name) = suggestion.as_ref()?;

        let root = root.replace_node(
            expr.clone(),
            JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(
                make::js_string_literal_expression(make::js_string_literal(type_name.as_str())),
            )),
        )?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Compare the result of `typeof` with a valid type name" }.to_owned(),
            root,
        })
    }
}

pub enum TypeofError {
    InvalidLiteral(TextRange, String),
    InvalidExpression(TextRange),
}

pub enum JsTypeName {
    Undefined,
    Object,
    Boolean,
    Number,
    String,
    Function,
    Symbol,
    BigInt,
}

impl JsTypeName {
    /// construct a [JsTypeName] from the textual name of a JavaScript type
    fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "undefined" => Self::Undefined,
            "object" => Self::Object,
            "boolean" => Self::Boolean,
            "number" => Self::Number,
            "string" => Self::String,
            "function" => Self::Function,
            "symbol" => Self::Symbol,
            "bigint" => Self::BigInt,
            _ => return None,
        })
    }

    /// Convert a [JsTypeName] to a JS string literal
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Object => "object",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
            Self::Function => "function",
            Self::Symbol => "symbol",
            Self::BigInt => "bigint",
        }
    }
}
