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

        let has_object_trailing_trivia =
            object_syntax.last_trailing_trivia().unwrap().pieces().len() > 0;
        let has_operator_leading_trivia =
            operator_token.clone()?.leading_trivia().pieces().len() > 0;

        let formatted_object = object?.format(formatter)?;

        if is_object_number_literal && (has_object_trailing_trivia || has_operator_leading_trivia) {
            let (object_leading, object_content, object_trailing) = formatted_object.split_trivia();

            Ok(group_elements(format_elements![
                object_leading,
                token("("),
                object_content,
                token(")"),
                object_trailing,
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
