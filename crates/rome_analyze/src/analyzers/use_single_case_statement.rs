use std::iter;

use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsAnyStatement, JsCaseClause, JsCaseClauseFields, JsSyntaxToken, TriviaPieceKind, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, TriviaPiece};

use crate::registry::{JsRuleAction, Rule, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};

/// Enforces case clauses have a single statement, emits a quick fix wrapping
/// the statements in a block
pub(crate) enum UseSingleCaseStatement {}

impl Rule for UseSingleCaseStatement {
    const NAME: &'static str = "useSingleCaseStatement";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsCaseClause;
    type State = ();

    fn run(n: &Self::Query) -> Option<Self::State> {
        if n.consequent().len() > 1 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(n: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::warning(markup! {
                "A switch case should only have a single statement. If you want more, then wrap it in a block."
            })
            .primary(n.consequent().range(), "")
        )
    }

    fn action(root: JsAnyRoot, n: &Self::Query, _: &Self::State) -> Option<JsRuleAction> {
        let JsCaseClauseFields {
            case_token,
            colon_token,
            consequent,
            ..
        } = n.as_fields();

        // Move the trailing trivia of the colon token to the opening bracket token,
        // this ensure comments stay in the right place
        let mut opening_token = String::from(" {");
        let mut trailing = Vec::new();

        if let Ok(token) = colon_token {
            for piece in token.trailing_trivia().pieces() {
                opening_token.push_str(piece.text());
                trailing.push(TriviaPiece::new(piece.kind(), piece.text_len()));
            }
        }

        // Copy the leading trivia of the case token on the closing bracket token
        // up to the first newline to align the indentation level
        let mut closing_token = String::new();
        let mut leading = Vec::new();

        if let Ok(token) = case_token {
            let leading_trivia = token.leading_trivia().pieces();
            let num_pieces = leading_trivia.len();
            let skip_count = leading_trivia
                .rev()
                .position(|piece| piece.is_newline())
                .and_then(|index| num_pieces.checked_sub(index + 1))
                .unwrap_or(0);

            for piece in token.leading_trivia().pieces().skip(skip_count) {
                closing_token.push_str(piece.text());
                leading.push(TriviaPiece::new(piece.kind(), piece.text_len()));
            }
        }

        closing_token.push('}');

        let node = n
            .clone()
            .with_consequent(make::js_statement_list(iter::once(
                JsAnyStatement::JsBlockStatement(make::js_block_statement(
                    JsSyntaxToken::new_detached(
                        T!['{'],
                        &opening_token,
                        [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
                        trailing,
                    ),
                    consequent,
                    JsSyntaxToken::new_detached(T!['}'], &closing_token, leading, []),
                )),
            )));

        let node = if let Ok(colon_token) = n.colon_token() {
            node.with_colon_token(colon_token.with_trailing_trivia(iter::empty()))
        } else {
            node
        };

        let root = root
            .replace_node(n.clone(), node)
            .expect("failed to replace node");

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the statements in a block" }.to_owned(),
            root,
        })
    }
}
