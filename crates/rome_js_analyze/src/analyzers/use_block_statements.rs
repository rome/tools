use rome_analyze::context::RuleContext;
use rome_analyze::{ActionCategory, Rule, RuleAction, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsAnyStatement, JsBinaryExpression,
    JsElseClauseFields, JsIfStatementFields, TextRange, T,
};
use rome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeExt, SyntaxResult, SyntaxToken};

use crate::JsRuleAction;

pub(crate) enum UseBlockStatements {}

impl Rule for UseBlockStatements {
    const NAME: &'static str = "useBlockStatements";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyStatement;
    type State = Vec<JsAnyStatement>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            JsAnyStatement::JsIfStatement(stmt) => {
                let JsIfStatementFields {
                    if_token: _,
                    l_paren_token: _,
                    test: _,
                    r_paren_token: _,
                    consequent,
                    else_clause,
                } = stmt.as_fields();
                let mut nodes_need_to_replaced = vec![];
                let consequent = consequent.ok()?;
                // if `IfStatement` has not consequent then it must has no else clause,
                // so this `?` operation here is safe
                if !matches!(&consequent, JsAnyStatement::JsBlockStatement(_)) {
                    nodes_need_to_replaced.push(consequent.clone());
                }
                if else_clause.is_some() {
                    // SAFETY: because we know the variant of `else_clause` is `Some(_)`
                    let JsElseClauseFields {
                        else_token: _,
                        alternate,
                    } = else_clause.unwrap().as_fields();
                    let alternate = alternate.ok()?;
                    if !matches!(
                        alternate,
                        JsAnyStatement::JsBlockStatement(_) | JsAnyStatement::JsIfStatement(_)
                    ) {
                        nodes_need_to_replaced.push(alternate);
                    }
                }
                Some(nodes_need_to_replaced)
            }
            JsAnyStatement::JsBlockStatement(_)
            | JsAnyStatement::JsBreakStatement(_)
            | JsAnyStatement::JsClassDeclaration(_)
            | JsAnyStatement::JsContinueStatement(_)
            | JsAnyStatement::JsDebuggerStatement(_)
            | JsAnyStatement::JsDoWhileStatement(_)
            | JsAnyStatement::JsEmptyStatement(_)
            | JsAnyStatement::JsExpressionStatement(_)
            | JsAnyStatement::JsForInStatement(_)
            | JsAnyStatement::JsForOfStatement(_)
            | JsAnyStatement::JsForStatement(_)
            | JsAnyStatement::JsFunctionDeclaration(_)
            | JsAnyStatement::JsLabeledStatement(_)
            | JsAnyStatement::JsReturnStatement(_)
            | JsAnyStatement::JsSwitchStatement(_)
            | JsAnyStatement::JsThrowStatement(_)
            | JsAnyStatement::JsTryFinallyStatement(_)
            | JsAnyStatement::JsTryStatement(_)
            | JsAnyStatement::JsUnknownStatement(_)
            | JsAnyStatement::JsVariableStatement(_)
            | JsAnyStatement::JsWhileStatement(_)
            | JsAnyStatement::JsWithStatement(_)
            | JsAnyStatement::TsDeclareFunctionDeclaration(_)
            | JsAnyStatement::TsDeclareStatement(_)
            | JsAnyStatement::TsEnumDeclaration(_)
            | JsAnyStatement::TsExternalModuleDeclaration(_)
            | JsAnyStatement::TsGlobalDeclaration(_)
            | JsAnyStatement::TsImportEqualsDeclaration(_)
            | JsAnyStatement::TsInterfaceDeclaration(_)
            | JsAnyStatement::TsModuleDeclaration(_)
            | JsAnyStatement::TsTypeAliasDeclaration(_) => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        None
        // Some(RuleDiagnostic:: {
        //     severity: Severity::Error,
        //     message: markup! {
        //         "Block statements are preferred in this position."
        //     }
        //     .to_owned(),
        //     range: if state.len() == 1 {
        //         state[0].range()
        //     } else {
        //         node.range()
        //     },
        // })
    }

    fn action(
        ctx: &RuleContext<Self>,
        nodes_need_to_replaced: &Self::State,
    ) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();
        let mut next_node = node.clone();
        // let mut root = root;
        for node in nodes_need_to_replaced.iter() {
            next_node = next_node
                .replace_node(
                    node.clone(),
                    JsAnyStatement::JsBlockStatement(make::js_block_statement(
                        SyntaxToken::new_detached(T!['{'], "{", [], []),
                        make::js_statement_list(std::iter::once(node.clone())),
                        SyntaxToken::new_detached(T!['}'], "}", [], []),
                    )),
                )
                .unwrap();
        }
        let root = root.replace_node(node.clone(), next_node)?;
        println!("{}\n------------------------------", root);
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with strict equality" }.to_owned(),
            root,
        })
    }
}

fn is_null_literal(res: SyntaxResult<JsAnyExpression>) -> bool {
    matches!(
        res,
        Ok(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsNullLiteralExpression(_)
        ))
    )
}
