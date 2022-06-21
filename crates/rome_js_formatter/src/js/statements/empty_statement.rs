use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::{JsEmptyStatement, JsEmptyStatementFields, JsSyntaxKind};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsEmptyStatement;

impl FormatNodeRule<JsEmptyStatement> for FormatJsEmptyStatement {
    fn fmt_fields(&self, node: &JsEmptyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().map(|p| p.kind());

        if matches!(
            parent_kind,
            Some(JsSyntaxKind::JS_DO_WHILE_STATEMENT)
                | Some(JsSyntaxKind::JS_IF_STATEMENT)
                | Some(JsSyntaxKind::JS_ELSE_CLAUSE)
        ) {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }
}
