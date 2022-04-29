use crate::{format_elements, hard_line_break, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsAnyStatement;
use rome_js_syntax::JsBlockStatement;

use rome_js_syntax::JsBlockStatementFields;
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::{AstNode, AstNodeList};

impl FormatNode for JsBlockStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
    // add extra branch to avoid formatting the same code twice and generating different code,
    // here is a example:
    // ```js
    //     try
    // /* missing comment */
    // {;}
    // finally {}
    // ```
    // if we don't add the extra branch, this function will return false, because  `block.statement` has one empty statement,
    // and would be formatted as :
    // ```js
    //     try
    // /* missing comment */
    // {}
    // finally {}
    // ```
    // for the second time, the function would return true, because the block is empty and `parent.syntax.kind` is  `JS_TRY_FINALLY_STATEMENT`, which would hit the branch `Some(_) => true`,
    // finally the code would be formatted as:
    // ```js
    // try
    /* missing comment */
    // {
    // } finally {
    // }
    // ```
    if !block.statements().is_empty()
        && block
            .statements()
            .iter()
            .any(|s| !matches!(s, JsAnyStatement::JsEmptyStatement(_)))
    {
        return false;
    }
    // reference https://github.com/prettier/prettier/blob/main/src/language-js/print/block.js#L19
    match block.syntax().parent().map(|p| p.kind()) {
        Some(
            JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
            | JsSyntaxKind::JS_DO_WHILE_STATEMENT
            | JsSyntaxKind::TS_MODULE_DECLARATION
            | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
        ) => false,

        Some(_) => true,
        None => false,
    }
}
