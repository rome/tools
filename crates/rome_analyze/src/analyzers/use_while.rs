use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyRoot, JsAnyStatement, JsForStatement, JsForStatementFields, T};
use rome_rowan::{AstNode, AstNodeExt};

use crate::analysis_server::{Rule, RuleCodeFix, RuleDiagnostic};

pub(crate) enum UseWhile {}

impl Rule for UseWhile {
    const NAME: &'static str = "useWhile";
    const ACTION_CATEGORIES: &'static [crate::ActionCategory] = &[];

    type Query = JsForStatement;
    type Result = ();

    fn run(n: &Self::Query) -> Option<Self::Result> {
        let JsForStatementFields {
            for_token: _,
            l_paren_token,
            initializer,
            first_semi_token: _,
            test,
            second_semi_token: _,
            update,
            r_paren_token,
            body,
        } = n.as_fields();

        if l_paren_token.is_err()
            || initializer.is_some()
            || test.is_none()
            || update.is_some()
            || r_paren_token.is_err()
            || body.is_err()
        {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(node: &Self::Query, _: &Self::Result) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "Use a while loop instead of a for loop"
            }
            .to_owned(),
            range: node.range(),
        })
    }

    fn code_fix(root: &JsAnyRoot, node: &Self::Query, _: &Self::Result) -> Option<RuleCodeFix> {
        let JsForStatementFields {
            for_token: _,
            l_paren_token,
            initializer: _,
            first_semi_token: _,
            test,
            second_semi_token: _,
            update: _,
            r_paren_token,
            body,
        } = node.as_fields();

        let root = root.clone().replace_node_retain_trivia(
            JsAnyStatement::from(node.clone()),
            JsAnyStatement::from(make::js_while_statement(
                make::token_with_space(T![while]),
                l_paren_token.ok()?,
                test?,
                r_paren_token.ok()?,
                body.ok()?,
            )),
        )?;

        Some(RuleCodeFix { root })
    }
}
