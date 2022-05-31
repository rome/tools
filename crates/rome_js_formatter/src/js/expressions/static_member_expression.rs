use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsStaticMemberExpression;
use rome_js_syntax::JsStaticMemberExpressionFields;

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn format_fields(
        node: &JsStaticMemberExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = node.as_fields();

        let object_syntax = object.clone()?.syntax().clone();

        let is_object_number_literal =
            object_syntax.kind() == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION;

        let has_object_trailing_trivia =
            object_syntax.last_trailing_trivia().unwrap().pieces().len() > 0;
        let has_operator_leading_trivia =
            operator_token.clone()?.leading_trivia().pieces().len() > 0;

        let formatted_object = formatted![formatter, [object?.format()]]?;

        if is_object_number_literal && (has_object_trailing_trivia || has_operator_leading_trivia) {
            let (object_leading, object_content, object_trailing) = formatted_object.split_trivia();

            Ok(group_elements(formatted![
                formatter,
                [
                    object_leading,
                    token("("),
                    object_content,
                    token(")"),
                    object_trailing,
                    operator_token.format(),
                    member.format(),
                ]
            ]?))
        } else {
            Ok(group_elements(formatted![
                formatter,
                [formatted_object, operator_token.format(), member.format(),]
            ]?))
        }
    }
}
