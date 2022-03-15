use rome_js_syntax::{AstNode, JsBinaryExpression};

use crate::{Action, ActionCategory, Analysis, AssistContext, SyntaxEdit};

use super::AssistProvider;

pub const ASSIST: AssistProvider = AssistProvider {
    name: "flipBinExp",
    action_categories: &[ActionCategory::Refactor],
    analyze,
};

fn analyze(ctx: &AssistContext) -> Option<Analysis> {
    let node = ctx.find_node_at_cursor_range::<JsBinaryExpression>()?;

    let op_range = node.operator().ok()?.text_trimmed_range();
    if !op_range.contains_range(ctx.cursor_range) {
        return None;
    }

    let lhs = node.left().ok()?;
    let rhs = node.right().ok()?;
    let edits = vec![
        SyntaxEdit::Replace {
            target: lhs.clone().into(),
            replacement: rhs.clone().into(),
            trimmed: true,
        },
        SyntaxEdit::Replace {
            target: rhs.into(),
            replacement: lhs.into(),
            trimmed: true,
        },
    ];

    let action = Action {
        title: "rome: flip binary expression".into(),
        range: node.syntax().text_trimmed_range(),
        edits,
        category: ActionCategory::Refactor,
    };
    Some(action.into())
}
