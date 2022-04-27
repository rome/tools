use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsEmptyStatement;
use rome_js_syntax::JsEmptyStatementFields;
use rome_js_syntax::{JsEmptyStatement, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNodeFields<JsEmptyStatement> for FormatNodeRule<JsEmptyStatement> {
    fn format_fields(
        node: &JsEmptyStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = self.syntax().parent().map(|p| p.kind());
        if matches!(
            parent_kind,
            Some(JsSyntaxKind::JS_DO_WHILE_STATEMENT)
                | Some(JsSyntaxKind::JS_IF_STATEMENT)
                | Some(JsSyntaxKind::JS_ELSE_CLAUSE)
        ) {
            semicolon_token.format(formatter)
        } else {
            Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
        }
    }
}
