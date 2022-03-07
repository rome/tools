use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsBlockStatement;

use rome_js_syntax::JsBlockStatementFields;
use rome_js_syntax::{AstNode, AstNodeList, JsSyntaxKind};

impl ToFormatElement for JsBlockStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBlockStatementFields {
            l_curly_token,
            statements,
            r_curly_token,
        } = self.as_fields();

        let stmts = formatter.format_list(statements);

        if is_non_collapsable_empty_block(self) {
            Ok(format_elements![
                l_curly_token.format(formatter)?,
                hard_line_break(),
                r_curly_token.format(formatter)?
            ])
        } else {
            formatter.format_delimited_block_indent(&l_curly_token?, stmts, &r_curly_token?)
        }
    }
}

// Formatting of curly braces for an:
// * empty block: same line `{}`,
// * empty block that is the 'cons' or 'alt' of an if statement: two lines `{\n}`
// * non empty block: put each stmt on its own line: `{\nstmt1;\nstmt2;\n}`
// * non empty block with comments (trailing comments on {, or leading comments on })
fn is_non_collapsable_empty_block(block: &JsBlockStatement) -> bool {
    if block
        .l_curly_token()
        .map_or_else(|_| false, |token| token.has_trailing_comments())
        || block
            .r_curly_token()
            .map_or_else(|_| false, |token| token.has_leading_comments())
    {
        return false;
    }

    if !block.statements().is_empty() {
        return false;
    }

    matches!(
        block.syntax().parent().map(|p| p.kind()),
        Some(JsSyntaxKind::JS_IF_STATEMENT | JsSyntaxKind::JS_ELSE_CLAUSE)
    )
}
