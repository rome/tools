use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    block_indent, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsBlockStatement;

use rslint_parser::{AstNode, AstNodeList, JsSyntaxKind};

impl ToFormatElement for JsBlockStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let stmts = formatter.format_list(self.statements());

        if is_non_collapsable_empty_block(self) {
            Ok(format_elements![
                self.l_curly_token().format(formatter)?,
                hard_line_break(),
                self.r_curly_token().format(formatter)?
            ])
        } else {
            formatter.format_delimited(
                &self.l_curly_token()?,
                |open_token_trailing, close_token_leading| {
                    Ok(block_indent(format_elements![
                        open_token_trailing,
                        stmts,
                        close_token_leading
                    ]))
                },
                &self.r_curly_token()?,
            )
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
