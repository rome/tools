use crate::prelude::*;

use rome_js_syntax::JsAnyStatement;
use rome_js_syntax::JsBlockStatement;

use crate::FormatNodeFields;
use rome_js_syntax::JsBlockStatementFields;
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::{AstNode, AstNodeList};

impl FormatNodeFields<JsBlockStatement> for FormatNodeRule<JsBlockStatement> {
    fn format_fields(
        node: &JsBlockStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsBlockStatementFields {
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        let stmts = formatter.format_list_with_hard_line(&statements);

        if is_non_collapsable_empty_block(node) {
            formatted![
                formatter,
                [
                    l_curly_token.format(),
                    hard_line_break(),
                    r_curly_token.format()
                ]
            ]
        } else {
            formatter
                .delimited(&l_curly_token?, stmts, &r_curly_token?)
                .block_indent()
                .finish()
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
    // here is an example:
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
    // reference https://github.com/prettier/prettier/blob/b188c905cfaeb238a122b4a95c230da83f2f3226/src/language-js/print/block.js#L19
    let parent = block.syntax().parent();
    match parent.clone().map(|p| p.kind()) {
        Some(
            JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
            | JsSyntaxKind::JS_DO_WHILE_STATEMENT
            | JsSyntaxKind::TS_MODULE_DECLARATION
            | JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
        ) => false,
        // prettier collapse the catch block when it don't have `finalizer`, insert a new line when it has `finalizer`
        Some(JsSyntaxKind::JS_CATCH_CLAUSE) => {
            // SAFETY: since parent node have `Some(kind)`, this must not be `None`
            let parent_unwrap = parent.unwrap();
            let finally_clause = parent_unwrap.next_sibling();
            matches!(
                finally_clause.map(|finally| finally.kind()),
                Some(JsSyntaxKind::JS_FINALLY_CLAUSE),
            )
        }
        Some(_) => true,
        None => false,
    }
}
