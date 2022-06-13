use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyLiteralExpression, JsAnyRoot, JsSyntaxKind, JsxAnyAttributeName, JsxAnyAttributeValue,
    JsxAttribute, JsxAttributeFields, JsxName, JsxNamespaceName, T,
};
use rome_rowan::{AstNode, AstNodeExt, SyntaxElement};

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
        let JsxAttributeFields { name, initializer } = n.as_fields();
        let name = name.ok()?.clone();
        let is_jsx_name = matches!(name, JsxAnyAttributeName::JsxName(_));
        let mut name_syntax = name.into_syntax();

        let trailing_trivia = name_syntax.last_trailing_trivia().map(|trivia| {
            trivia
                .pieces()
                .map(|p| (p.kind(), p.text()))
        });
        let next_last_token = name_syntax
            .last_token()
            .map(|tok| tok.with_trailing_trivia(std::iter::empty()))?;
        // name_syntax = name_syntax.splice_slots(
        //     token_length - 1..=token_length - 1,
        //     std::iter::once(Some(SyntaxElement::Token(last_token))),
        // );
        let last_token = name_syntax.last_token()?;
        name_syntax = name_syntax
            .replace_child(
                SyntaxElement::Token(last_token),
                SyntaxElement::Token(next_last_token),
            )
            .unwrap();
        let next_name = match is_jsx_name {
            true => JsxAnyAttributeName::JsxName(JsxName::unwrap_cast(name_syntax)),
            false => {
                JsxAnyAttributeName::JsxNamespaceName(JsxNamespaceName::unwrap_cast(name_syntax))
            }
        };
        let attr_value = make::jsx_expression_attribute_value(
            make::token(JsSyntaxKind::L_CURLY),
            rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsBooleanLiteralExpression(
                    make::js_boolean_literal_expression(make::token(T![true])),
                ),
            ),
            make::token(JsSyntaxKind::R_CURLY),
        );
        let next_attr = make::jsx_attribute(next_name).with_initializer(
            make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                JsxAnyAttributeValue::JsxExpressionAttributeValue(attr_value),
            ),
        );
        let next_attr = next_attr.build();
        // let next_attr =
        // let next_attr =
        //     n.clone().with_initializer(Some(make::jsx_attribute_initializer_clause(
        //         make::token(T![=]),
        //         JsxAnyAttributeValue::JsxExpressionAttributeValue(attr_value),
        //     )));
        // next_attr.replace_node_discard_trivia(prev_node, next_node)

        // next_attr_syntax.children_with_tokens().position(|item| {

        // });
        // next_attr.tri

        let root = root.replace_node_discard_trivia(n.clone(), next_attr)?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Add explicit `true` literal for this attribute" }.to_owned(),
            root,
        })
    }
}
