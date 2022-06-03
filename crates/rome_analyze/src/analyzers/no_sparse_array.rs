use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyRoot,
    JsArrayElementList, JsArrayExpression, JsArrayHole, JsComputedMemberExpression,
    JsComputedMemberExpressionFields, JsStaticMemberExpression, JsStaticMemberExpressionFields,
    JsSyntaxKind, JsUnaryExpression, JsUnaryOperator, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList, SyntaxElement};

use crate::registry::{Rule, RuleAction, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};

pub(crate) enum NoSparseArray {}

impl Rule for NoSparseArray {
    const NAME: &'static str = "noSparseArray";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsArrayExpression;
    type State = ();

    fn run(node: &Self::Query) -> Option<Self::State> {
        // We defer collect `JsHole` index until user want to apply code action.
        node.elements()
            .iter()
            .filter_map(|item| item.ok())
            .position(|element| matches!(element, JsAnyArrayElement::JsArrayHole(_),))
            .map(|_| ())
    }

    fn diagnostic(node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            range: node.syntax().text_trimmed_range(),
            message: markup! {
                "This "<Emphasis>"array"</Emphasis>" contains an "<Emphasis>"empty slot"</Emphasis>"."
            }
            .to_owned(),
        })
    }

    fn action(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<RuleAction> {
        let mut syntax = node.elements().clone().into_syntax();
        let mut hole_index_iter = syntax
            .children_with_tokens()
            .enumerate()
            .filter_map(|(i, a)| {
                if matches!(a.kind(), JsSyntaxKind::JS_ARRAY_HOLE) {
                    Some(i)
                } else {
                    None
                }
            });
            // syntax.
        for index in hole_index_iter {
            let ident_expr = make::js_identifier_expression(make::js_reference_identifier(
                make::ident("undefined"),
            ));
            let ident_expr_syntax = ident_expr.into_syntax();
            syntax = syntax.splice_slots(
                index..=index,
                [Some(SyntaxElement::Node(ident_expr_syntax.clone()))].into_iter(),
            );
        }

        let root = root
            .replace_node(
                node.clone(),
                make::js_array_expression(
                    node.l_brack_token().ok()?,
                    JsArrayElementList::unwrap_cast(syntax),
                    node.r_brack_token().ok()?,
                ),
            )
            .unwrap();

        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with undefined" }.to_owned(),
            root,
        })
    }
}
