use crate::utils::format_with_semicolon;
use crate::{
    empty_element, formatted, group_elements, soft_block_indent, space_token,
    token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsReturnStatement, JsReturnStatementFields, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNode for JsReturnStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let return_token = return_token.format(formatter)?;

        let argument = if let Some(argument) = argument {
            if matches!(
                argument.syntax().kind(),
                JsSyntaxKind::JS_SEQUENCE_EXPRESSION
            ) {
                formatted![
                    formatter,
                    space_token(),
                    group_elements(formatted![
                        formatter,
                        token("("),
                        soft_block_indent(argument.format(formatter)?),
                        token(")")
                    ]?),
                ]?
            } else {
                formatted![formatter, space_token(), argument.format(formatter)?]?
            }
        } else {
            empty_element()
        };

        format_with_semicolon(
            formatter,
            formatted![formatter, return_token, argument]?,
            semicolon_token,
        )
    }
}
