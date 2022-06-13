use crate::prelude::*;
use rome_formatter::write;

use crate::builders::format_inserted;
use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyStatement, JsLabeledStatement};
use rome_js_syntax::{JsLabeledStatementFields, JsSyntaxKind};

impl FormatNodeFields<JsLabeledStatement> for FormatNodeRule<JsLabeledStatement> {
    fn fmt_fields(node: &JsLabeledStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = node.as_fields();

        write!(f, [label_token.format(), colon_token.format()])?;

        match body? {
            JsAnyStatement::JsEmptyStatement(empty) => {
                // If the body is an empty statement, force semicolon insertion
                write!(
                    f,
                    [empty.format(), format_inserted(JsSyntaxKind::SEMICOLON)]
                )
            }
            body => {
                write!(f, [space_token(), body.format()])
            }
        }
    }
}
