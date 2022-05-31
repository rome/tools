use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsEmptyStatement, JsEmptyStatementFields, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNodeFields<JsEmptyStatement> for FormatNodeRule<JsEmptyStatement> {
    fn format_fields(
        node: &JsEmptyStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().map(|p| p.kind());
        if matches!(
            parent_kind,
            Some(JsSyntaxKind::JS_DO_WHILE_STATEMENT)
                | Some(JsSyntaxKind::JS_IF_STATEMENT)
                | Some(JsSyntaxKind::JS_ELSE_CLAUSE)
        ) {
            formatted![formatter, [semicolon_token.format()]]
        } else {
            Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
        }
    }
}
