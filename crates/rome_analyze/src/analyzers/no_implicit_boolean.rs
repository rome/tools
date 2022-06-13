use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyLiteralExpression, JsAnyRoot, JsSyntaxKind, JsSyntaxToken, JsxAnyAttributeName,
    JsxAnyAttributeValue, JsxAttribute, JsxAttributeFields, JsxName, JsxNamespaceName, T,
};
use rome_rowan::{AstNode, AstNodeExt, SyntaxElement, TriviaPiece};

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
        let JsxAttributeFields {
            name,
            initializer: _,
        } = n.as_fields();

        let name = name.ok()?.clone();
        // we use this variable for constructing `JsxAnyAttributeName` without clone the name, so we pre compute the type here.
        let is_jsx_name = matches!(name, JsxAnyAttributeName::JsxName(_));

        let mut name_syntax = name.into_syntax();
        
        // we need to move trailing_trivia of name_syntax to close_curly_token
        // <div disabled /**test*/ /> ->    <div disabled={true}/**test*/ />
        let mut close_curly_token = String::from("}");
        let mut trailing = Vec::new();
        if let Some(trivia) = name_syntax.last_trailing_trivia() {
            for piece in trivia.pieces() {
                trailing.push(TriviaPiece::new(piece.kind(), piece.text_len()));
                close_curly_token += piece.text();
            }
        }
        let last_token_of_name_syntax = name_syntax.last_token()?;
        // drop the trailing trivia of name_syntax, at CST level it means
        // clean the trailing trivia of last token of name_syntax
        let next_last_token_of_name_syntax = name_syntax
            .last_token()
            .map(|tok| tok.with_trailing_trivia(std::iter::empty()))?;

        name_syntax = name_syntax
            .replace_child(
                SyntaxElement::Token(last_token_of_name_syntax),
                SyntaxElement::Token(next_last_token_of_name_syntax),
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
            JsSyntaxToken::new_detached(JsSyntaxKind::R_CURLY, &close_curly_token, [], trailing),
        );
        let next_attr = make::jsx_attribute(next_name).with_initializer(
            make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                JsxAnyAttributeValue::JsxExpressionAttributeValue(attr_value),
            ),
        );
        let next_attr = next_attr.build();

        let root = root.replace_node_discard_trivia(n.clone(), next_attr)?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Add explicit `true` literal for this attribute" }.to_owned(),
            root,
        })
    }
}
