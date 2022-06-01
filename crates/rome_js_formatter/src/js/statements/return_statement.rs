use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{JsReturnStatement, JsReturnStatementFields, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNodeFields<JsReturnStatement> for FormatNodeRule<JsReturnStatement> {
    fn format_fields(
        node: &JsReturnStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        let return_token = return_token.format();

        let argument = if let Some(argument) = argument {
            if matches!(
                argument.syntax().kind(),
                JsSyntaxKind::JS_SEQUENCE_EXPRESSION
            ) {
                formatted![
                    formatter,
                    [
                        space_token(),
                        group_elements(formatted![
                            formatter,
                            [
                                token("("),
                                soft_block_indent(formatted![formatter, [argument.format()]]?),
                                token(")")
                            ]
                        ]?),
                    ]
                ]?
            } else {
                formatted![formatter, [space_token(), argument.format()]]?
            }
        } else {
            empty_element()
        };

        format_with_semicolon(
            formatter,
            formatted![formatter, [return_token, argument]]?,
            semicolon_token,
        )
    }
}
