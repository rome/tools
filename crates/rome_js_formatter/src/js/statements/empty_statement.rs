use crate::{empty_element, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsEmptyStatementFields;
use rome_js_syntax::{JsEmptyStatement, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNode for JsEmptyStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyStatementFields { semicolon_token } = self.as_fields();
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
