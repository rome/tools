use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsBinaryExpression,
    JsBinaryExpressionFields, JsBinaryOperator, JsUnaryOperator,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::{
    registry::{Rule, RuleAction, RuleDiagnostic},
    ActionCategory, RuleCategory,
};

pub(crate) enum ValidTypeof {}

impl Rule for ValidTypeof {
    const NAME: &'static str = "validTypeof";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsBinaryExpression;
    type State = Option<(JsAnyExpression, JsTypeName)>;

    fn run(n: &Self::Query) -> Option<Self::State> {
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

        match (&left, &right) {
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
                    let literal = literal
                        .text_trimmed()
                        .trim_start_matches(['"', '\''])
                        .trim_end_matches(['"', '\''])
                        .to_lowercase();

                    if JsTypeName::from_str(&literal).is_some() {
                        return None;
                    }

                    // Try to fix the casing of the literal eg. "String" -> "string"
                    let literal = literal.to_lowercase();
                    return Some(
                        JsTypeName::from_str(&literal).map(|type_name| (lit.clone(), type_name)),
                    );
                }
            }

            // Check for `typeof $expr == typeof $expr`
            (
                JsAnyExpression::JsUnaryExpression(left),
                JsAnyExpression::JsUnaryExpression(right),
            ) => {
                let is_typeof_left = left.operator().ok()? != JsUnaryOperator::Typeof;
                let is_typeof_right = right.operator().ok()? != JsUnaryOperator::Typeof;
                if (is_typeof_left && is_typeof_right) || (!is_typeof_left && !is_typeof_right) {
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
                return Some(ident.name().ok().and_then(|name| {
                    let value = name.value_token().ok()?;

                    let to_lower = value.text_trimmed().to_lowercase();
                    let as_type = JsTypeName::from_str(&to_lower)?;

                    Some((id.clone(), as_type))
                }));
            }

            // Check for `typeof $expr == $expr`
            (JsAnyExpression::JsUnaryExpression(unary), _)
            | (_, JsAnyExpression::JsUnaryExpression(unary)) => {
                if unary.operator().ok()? != JsUnaryOperator::Typeof {
                    return None;
                }
            }

            _ => return None,
        }

        Some(None)
    }

    fn diagnostic(node: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "Invalid typeof comparison value"
            }
            .to_owned(),
            range: node.range(),
        })
    }

    fn action(root: JsAnyRoot, _node: &Self::Query, state: &Self::State) -> Option<RuleAction> {
        let (expr, type_name) = state.as_ref()?;

        let root = root.replace_node(
            expr.clone(),
            JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::from(
                make::js_string_literal_expression(make::js_string_literal(type_name.as_str())),
            )),
        )?;

        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Compare the result of `typeof` with a valid type name" }.to_owned(),
            root,
        })
    }
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
