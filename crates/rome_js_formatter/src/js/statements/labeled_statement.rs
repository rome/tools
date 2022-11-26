use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsLabeledStatementFields;
use rome_js_syntax::{JsAnyStatement, JsLabeledStatement};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLabeledStatement;

impl FormatNodeRule<JsLabeledStatement> for FormatJsLabeledStatement {
    fn fmt_fields(&self, node: &JsLabeledStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = node.as_fields();

        write!(f, [label_token.format(), colon_token.format()])?;

        match body? {
            JsAnyStatement::JsEmptyStatement(empty) => {
                // If the body is an empty statement, force semicolon insertion
                write!(f, [empty.format(), text(";")])
            }
            body => {
                write!(f, [space(), body.format()])
            }
        }
    }
}
