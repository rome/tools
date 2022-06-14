use crate::context::{JsRuleContext, RuleContext};
use crate::registry::{JsRuleAction, Rule, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyArrayElement, JsAnyExpression, JsArrayExpression, TriviaPieceKind};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList};

pub(crate) enum NoSparseArray {}

impl Rule for NoSparseArray {
    const NAME: &'static str = "noSparseArray";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsArrayExpression;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        // We defer collect `JsHole` index until user want to apply code action.
        ctx.query().elements().iter().find_map(|element| {
            if matches!(element.ok()?, JsAnyArrayElement::JsArrayHole(_),) {
                Some(())
            } else {
                None
            }
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
            ctx.query().syntax().text_trimmed_range(),
        markup! {
                "This "<Emphasis>"array"</Emphasis>" contains an "<Emphasis>"empty slot"</Emphasis>"."
            }
            .to_owned()
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut final_array_element_list = ctx.query().elements();

        for (i, item) in final_array_element_list.iter().enumerate() {
            if matches!(item, Ok(JsAnyArrayElement::JsArrayHole(_))) {
                let undefine_indent = if i == 0 {
                    make::ident("undefined")
                } else {
                    make::ident("undefined")
                        .with_leading_trivia(std::iter::once((TriviaPieceKind::Whitespace, " ")))
                };
                let ident_expr =
                    make::js_identifier_expression(make::js_reference_identifier(undefine_indent));
                // Why we need to use `final_array_element_list.iter().nth(i)` instead of `item`, because every time we
                // call `replace_node` the previous iteration `item` is not the descent child of current `final_array_element_list` any more.
                let n_element = final_array_element_list.iter().nth(i)?.ok()?;
                final_array_element_list = final_array_element_list.replace_node(
                    n_element,
                    JsAnyArrayElement::JsAnyExpression(JsAnyExpression::JsIdentifierExpression(
                        ident_expr,
                    )),
                )?;
            }
        }

        let node = ctx.query();

        let root = ctx.root().clone().replace_node(
            node.clone(),
            make::js_array_expression(
                node.l_brack_token().ok()?,
                final_array_element_list,
                node.r_brack_token().ok()?,
            ),
        )?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace hole with undefined" }.to_owned(),
            root,
        })
    }
}
