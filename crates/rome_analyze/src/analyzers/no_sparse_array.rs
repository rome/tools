use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyRoot,
    JsArrayExpression, JsArrayHole, JsComputedMemberExpression, JsComputedMemberExpressionFields,
    JsStaticMemberExpression, JsStaticMemberExpressionFields, JsSyntaxKind, JsUnaryExpression,
    JsUnaryOperator, T,
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
        // We defer collect `JsHole` index until user want to fix this issue.
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
        let mut syntax = node.clone().into_syntax();
        let hole_index_iter = syntax.children().enumerate().filter_map(|(i, a)| {
            if matches!(a.kind(), JsSyntaxKind::JS_ARRAY_HOLE) {
                Some(i)
            } else {
                None
            }
        });

        let ident_expr =
            make::js_identifier_expression(make::js_reference_identifier(make::ident("undefined")));
        let ident_expr_syntax = ident_expr.into_syntax();
        for index in hole_index_iter {
            syntax = syntax.splice_slots(
                index..=index,
                [Some(SyntaxElement::Node(ident_expr_syntax.clone()))].into_iter(),
            );
        }

        let root = root.replace_node(node.clone(), JsArrayExpression::unwrap_cast(syntax))?;
        // syntax.splice_slots(range, replace_with)
        // let root = root.replace_node(
        //     JsAnyExpression::from(node.clone()),
        //     JsAnyExpression::from(make::js_assignment_expression(
        //         state.clone().try_into().ok()?,
        //         make::token_decorated_with_space(T![=]),
        //         JsAnyExpression::from(make::js_identifier_expression(
        //             make::js_reference_identifier(make::ident("undefined")),
        //         )),
        //     )),
        // )?;

        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with undefined assignment" }.to_owned(),
            root,
        })
    }
}
