use crate::prelude::*;

use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_rowan::AstNode;

impl FormatNodeFields<JsStringLiteralExpression> for FormatNodeRule<JsStringLiteralExpression> {
    fn format_fields(
        node: &JsStringLiteralExpression,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let syntax_node = node.syntax();
        let parent = syntax_node.parent();

        let needs_parenthesis = parent.and_then(JsExpressionStatement::cast).is_some();

        let formatted_element = format_string_literal_token(value_token, formatter);
        if needs_parenthesis {
            let (leading_trivia, content, trailing_trivia) = formatted_element.split_trivia();
            Ok(format_elements![
                leading_trivia,
                format_elements![token("("), content, token(")"),],
                trailing_trivia,
            ])
        } else {
            Ok(formatted_element)
        }
    }
}
