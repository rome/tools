use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::{format_elements, token, FormatResult};
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;

impl FormatNode for JsStringLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = self.as_fields();

        let value_token = value_token?;
        let syntax_node = self.syntax();
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
