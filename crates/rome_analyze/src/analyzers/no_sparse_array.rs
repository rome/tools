use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyExpression, JsAnyRoot, JsArrayExpression, TriviaPieceKind,
};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList};

use crate::registry::{JsRuleAction, Rule, RuleDiagnostic};
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
            // .filter_map(|item| item.ok())
            .find_map(|element| {
                if matches!(element.ok()?, JsAnyArrayElement::JsArrayHole(_),) {
                    Some(())
                } else {
                    None
                }
            })
        // .map(|_| ())
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

    fn action(root: JsAnyRoot, node: &Self::Query, _state: &Self::State) -> Option<JsRuleAction> {
        let mut array_element_list = node.elements();
        let hole_index_iter = array_element_list
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                if matches!(item, Ok(JsAnyArrayElement::JsArrayHole(_))) {
                    Some(i)
                } else {
                    None
                }
            });

        for index in hole_index_iter {
            let undefine_indent = if index == 0 {
                make::ident("undefined")
            } else {
                make::ident("undefined")
                    .with_leading_trivia(std::iter::once((TriviaPieceKind::Whitespace, " ")))
            };
            let ident_expr =
                make::js_identifier_expression(make::js_reference_identifier(undefine_indent));

            let n_element = array_element_list.iter().nth(index)?.ok()?;
            array_element_list = array_element_list.replace_node(
                n_element,
                JsAnyArrayElement::JsAnyExpression(JsAnyExpression::JsIdentifierExpression(
                    ident_expr,
                )),
            )?;
        }
        let root = root
            .replace_node(
                node.clone(),
                make::js_array_expression(
                    node.l_brack_token().ok()?,
                    array_element_list,
                    node.r_brack_token().ok()?,
                ),
            )
            .unwrap();

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace hole with undefined" }.to_owned(),
            root,
        })
    }
}
