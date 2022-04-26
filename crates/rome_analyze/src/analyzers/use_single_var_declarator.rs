use std::iter;

use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsAnyStatement, JsStatementList, JsSyntaxToken, JsVariableDeclarationFields,
    JsVariableDeclaratorList, JsVariableStatement, JsVariableStatementFields,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, AstNodeListExt, AstSeparatedList};

use crate::categories::ActionCategory;

use crate::registry::{Rule, RuleCodeFix, RuleDiagnostic};

pub(crate) enum UseSingleVarDeclarator {}

impl Rule for UseSingleVarDeclarator {
    const NAME: &'static str = "useSingleVarDeclarator";
    const ACTION_CATEGORIES: &'static [ActionCategory] = &[];

    type Query = JsVariableStatement;
    type State = (
        JsSyntaxToken,
        JsVariableDeclaratorList,
        Option<JsSyntaxToken>,
    );

    fn run(node: &Self::Query) -> Option<Self::State> {
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

    fn diagnostic(node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            range: node.syntax().text_trimmed_range(),
            message: markup! {
                "Declare variables separately."
            }
            .to_owned(),
        })
    }

    fn code_fix(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<RuleCodeFix> {
        let (kind, declarators, semicolon_token) = state;

        let prev_parent = JsStatementList::cast(node.syntax().parent()?)?;
        let index = prev_parent
            .iter()
            .position(|slot| slot.syntax() == node.syntax())?;

        let mut is_first = true;
        let next_parent = prev_parent.clone().splice(
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

                Some(JsAnyStatement::from(builder.build()))
            }),
        );

        Some(RuleCodeFix {
            root: root.replace_node(prev_parent, next_parent)?,
        })
    }
}
