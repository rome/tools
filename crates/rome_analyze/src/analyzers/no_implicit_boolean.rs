use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyLiteralExpression, JsAnyRoot, JsSyntaxKind, JsxAnyAttributeValue, JsxAttribute, T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::registry::{JsRuleAction, Rule, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};

pub(crate) enum NoImplicitBoolean {}

impl Rule for NoImplicitBoolean {
    const NAME: &'static str = "noImplicitBoolean";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsxAttribute;
    type State = ();

    fn run(n: &Self::Query) -> Option<Self::State> {
        match n.initializer() {
            Some(_) => None,
            None => Some(()),
        }
    }

    fn diagnostic(n: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            message: markup! {
                "Use explicit boolean values for boolean JSX props."
            }
            .to_owned(),
            range: n.range(),
        })
    }

    fn action(root: JsAnyRoot, n: &Self::Query, _: &Self::State) -> Option<JsRuleAction> {
        let attr_value = make::jsx_expression_attribute_value(
            make::token(JsSyntaxKind::L_CURLY),
            rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsBooleanLiteralExpression(
                    make::js_boolean_literal_expression(make::token(T![true])),
                ),
            ),
            make::token(JsSyntaxKind::R_CURLY),
        );
        let next_attr = n
            .clone()
            .with_initializer(Some(make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                JsxAnyAttributeValue::JsxExpressionAttributeValue(attr_value),
            )));
        let root = root.replace_node(n.clone(), next_attr)?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Add explicit `true` literal for this attribute" }.to_owned(),
            root,
        })
    }
}
