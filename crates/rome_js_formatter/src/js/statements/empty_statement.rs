use crate::{empty_element, FormatElement, FormatNode, Formatter};
use rome_formatter::{format_elements, hard_line_break, token, FormatResult};
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
            let new_body = format_elements![token("{"), hard_line_break(), token("}")];
            Ok(formatter.format_replaced(&semicolon_token?, new_body))
        } else {
            Ok(formatter.format_replaced(&semicolon_token?, empty_element()))
        }
    }
}
