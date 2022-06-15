use std::iter;

use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsModuleItemList, JsStatementList, JsSyntaxToken, JsVariableDeclarationFields,
    JsVariableDeclaratorList, JsVariableStatement, JsVariableStatementFields,
};
use rome_rowan::{AstNode, AstSeparatedList};

use crate::JsRuleAction;

pub(crate) enum UseSingleVarDeclarator {}

impl Rule for UseSingleVarDeclarator {
    const NAME: &'static str = "useSingleVarDeclarator";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsVariableStatement;
    type State = (
        JsSyntaxToken,
        JsVariableDeclaratorList,
        Option<JsSyntaxToken>,
    );

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query_result();

        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        let JsVariableDeclarationFields { kind, declarators } = declaration.ok()?.as_fields();

        let kind = kind.ok()?;

        if declarators.len() < 2 {
            return None;
        }

        Some((kind, declarators, semicolon_token))
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query_result();
        
        Some(RuleDiagnostic::warning(
            node.range(),
            "Declare variables separately",
        ))
    }

    fn action(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<JsRuleAction> {
        let (kind, declarators, semicolon_token) = state;

        let prev_parent = node.syntax().parent()?;
        if !JsStatementList::can_cast(prev_parent.kind())
            && !JsModuleItemList::can_cast(prev_parent.kind())
        {
            return None;
        }

        let index = prev_parent
            .children()
            .position(|slot| &slot == node.syntax())?;

        let mut is_first = true;
        let next_parent = prev_parent.clone().splice_slots(
            index..=index,
            declarators.iter().filter_map(|declarator| {
                let declarator = declarator.ok()?;

                // Clone the entire leading trivia for the first statement, but
                // trim it to the first newline for the following lines
                let kind = if is_first {
                    is_first = false;
                    kind.clone()
                } else {
                    make::clone_token_up_to_first_newline(kind)
                };

                let mut builder = make::js_variable_statement(make::js_variable_declaration(
                    kind,
                    make::js_variable_declarator_list(iter::once((declarator, None))),
                ));

                if let Some(semicolon_token) = semicolon_token {
                    builder = builder.with_semicolon_token(semicolon_token.clone());
                }

                Some(Some(builder.build().into_syntax().into()))
            }),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Break out into multiple declarations" }.to_owned(),
            root: JsAnyRoot::unwrap_cast(
                root.into_syntax()
                    .replace_child(prev_parent.into(), next_parent.into())?,
            ),
        })
    }
}
