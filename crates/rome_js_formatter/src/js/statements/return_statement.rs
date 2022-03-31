use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{
    empty_element, format_elements, group_elements, soft_block_indent, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{AstNode, JsReturnStatement, JsReturnStatementFields, JsSyntaxKind};

impl ToFormatElement for JsReturnStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
                format_elements![
                    space_token(),
                    group_elements(format_elements![
                        token("("),
                        soft_block_indent(argument.format(formatter)?),
                        token(")")
                    ]),
                ]
            } else {
                format_elements![space_token(), argument.format(formatter)?]
            }
        } else {
            empty_element()
        };

        format_with_semicolon(
            formatter,
            format_elements![return_token, argument],
            semicolon_token,
        )
    }
}
