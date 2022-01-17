use rslint_parser::ast::JsBlockStatement;
use rslint_parser::{AstNode, AstNodeList, JsSyntaxKind};

use crate::ts::statements::format_statements;
use crate::{
    block_indent, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

impl ToFormatElement for JsBlockStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let stmts = format_statements(self.statements(), formatter);

        if is_non_collapsable_empty_block(self) {
            Ok(format_elements![
                formatter.format_token(&self.l_curly_token()?)?,
                hard_line_break(),
                formatter.format_token(&self.r_curly_token()?)?
            ])
        } else {
            formatter.format_delimited(
                &self.l_curly_token()?,
                |leading, trailing| Ok(block_indent(format_elements![leading, stmts, trailing])),
                &self.r_curly_token()?,
            )
        }
    }
}

// Formatting of curly braces for an:
// * empty block: same line `{}`,
// * empty block that is the 'cons' or 'alt' of an if statement: two lines `{\n}`
// * non empty block: put each stmt on its own line: `{\nstmt1;\nstmt2;\n}`
fn is_non_collapsable_empty_block(block: &JsBlockStatement) -> bool {
    if !block.statements().is_empty() {
        return false;
    }

    matches!(
        block.syntax().parent().map(|p| p.kind()),
        Some(JsSyntaxKind::JS_IF_STATEMENT | JsSyntaxKind::JS_ELSE_CLAUSE)
    )
}
