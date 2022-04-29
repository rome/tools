use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::{format_elements, group_elements, token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsStaticMemberExpression;
use rome_js_syntax::JsStaticMemberExpressionFields;

impl FormatNode for JsStaticMemberExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = self.as_fields();

        let object_syntax = object.clone()?.syntax().clone();

        let is_object_number_literal =
            object_syntax.kind() == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION;

        let has_object_trailing_whitespace = object_syntax
            .text()
            .to_string()
            .chars()
            .last()
            .unwrap()
            .is_whitespace();

        let formatted_object = object?.format(formatter)?;

        if is_object_number_literal && has_object_trailing_whitespace {
            Ok(group_elements(format_elements![
                token("("),
                formatted_object,
                token(")"),
                operator_token.format(formatter)?,
                member.format(formatter)?,
            ]))
        } else {
            Ok(group_elements(format_elements![
                formatted_object,
                operator_token.format(formatter)?,
                member.format(formatter)?,
            ]))
        }
    }
}
