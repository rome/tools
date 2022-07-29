use crate::prelude::*;
use rome_formatter::{format_args, write};

use rome_js_syntax::JsIfStatementFields;
use rome_js_syntax::{JsAnyStatement, JsIfStatement};

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
                FormatConsequentClause::new(&consequent),
            ]),]
        )?;

        if let Some(else_clause) = else_clause {
            let else_on_same_line = matches!(consequent, JsBlockStatement(_));

            if else_on_same_line {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }
}

pub(crate) struct FormatConsequentClause<'a> {
    statement: &'a JsAnyStatement,
    force_space: bool,
}

impl<'a> FormatConsequentClause<'a> {
    pub fn new(consequent: &'a JsAnyStatement) -> Self {
        Self {
            statement: consequent,
            force_space: false,
        }
    }

    /// Prevents that the consequent is formatted on its own line and indented by one level and
    /// instead gets separated by a space.
    pub fn with_forced_space(mut self, forced: bool) -> Self {
        self.force_space = forced;
        self
    }
}

impl Format<JsFormatContext> for FormatConsequentClause<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyStatement::*;

        if let JsEmptyStatement(empty) = &self.statement {
            write!(f, [empty.format()])
        } else if matches!(&self.statement, JsBlockStatement(_)) || self.force_space {
            write!(f, [space(), self.statement.format()])
        } else {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.statement.format()
                ])]
            )
        }
    }
}
