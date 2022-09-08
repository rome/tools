use crate::prelude::*;
use rome_formatter::{format_args, write, CstFormatContext};

use crate::builders::format_delimited;
use crate::utils::FormatStatementBody;
use rome_js_syntax::JsIfStatement;
use rome_js_syntax::JsIfStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsIfStatement;

impl FormatNodeRule<JsIfStatement> for FormatJsIfStatement {
    fn fmt_fields(&self, node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        use rome_js_syntax::JsAnyStatement::*;

        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;
        let consequent = consequent?;

        write!(
            f,
            [group(&format_args![
                if_token.format(),
                space(),
                format_delimited(&l_paren_token, &test.format(), &r_paren_token)
                    .soft_block_indent(),
                FormatStatementBody::new(&consequent),
            ]),]
        )?;

        if let Some(else_clause) = else_clause {
            let comments = f.context().comments();
            let dangling_comments = comments.dangling_comments(node.syntax());
            let dangling_line_comment = dangling_comments
                .last()
                .map_or(false, |comment| comment.kind().is_line());
            let has_dangling_comments = !dangling_comments.is_empty();

            let trailing_line_comment = comments
                .trailing_comments(consequent.syntax())
                .iter()
                .any(|comment| comment.kind().is_line());

            let else_on_same_line = matches!(consequent, JsBlockStatement(_))
                && !trailing_line_comment
                && !dangling_line_comment;

            if else_on_same_line {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            if has_dangling_comments {
                write!(f, [format_dangling_comments(node.syntax())])?;

                if trailing_line_comment || dangling_line_comment {
                    write!(f, [hard_line_break()])?
                } else {
                    write!(f, [space()])?;
                }
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }
}
