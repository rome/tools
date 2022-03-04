use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::is_simple_function_expression;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplateElementFields;
use rslint_parser::ast::{JsAnyExpression, JsAnyFunction, JsAnyName, JsTemplateElement};
use rslint_parser::{AstNode, SyntaxNode, SyntaxNodeExt};

impl ToFormatElement for JsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsTemplateElementFields {
            dollar_curly_token,
            expression,
            r_curly_token,
        } = self.as_fields();

        let expression = expression?;
        let r_curly_token = r_curly_token?;
        let dollar_curly_token = dollar_curly_token?;
        let has_comments = expression.syntax().contains_comments()
            || r_curly_token.has_leading_comments()
            || dollar_curly_token.has_trailing_comments();
        let expression_is_simple = is_plain_expression(&expression)?;
        let expression = expression.format(formatter)?;

        if expression_is_simple && !has_comments {
            Ok(format_elements![
                dollar_curly_token.format(formatter)?,
                expression,
                r_curly_token.format(formatter)?
            ])
        } else {
            formatter.format_delimited_soft_block_indent(
                &dollar_curly_token,
                expression,
                &r_curly_token,
            )
        }
    }
}

/// We want to break the template element only when we have articulated expressions inside it.
///
/// We a plain expression is when it's one of the following:
/// - `loreum ${this.something} ipsum`
/// - `loreum ${a.b.c} ipsum`
/// - `loreum ${a} ipsum`
fn is_plain_expression(current_expression: &JsAnyExpression) -> FormatResult<bool> {
    match current_expression {
        JsAnyExpression::JsStaticMemberExpression(_)
        | JsAnyExpression::JsComputedMemberExpression(_)
        | JsAnyExpression::JsIdentifierExpression(_)
        | JsAnyExpression::JsAnyLiteralExpression(_)
        | JsAnyExpression::JsCallExpression(_)
        | JsAnyExpression::JsParenthesizedExpression(_) => Ok(true),
        JsAnyExpression::JsFunctionExpression(function_expression) => {
            Ok(is_simple_function_expression(
                JsAnyFunction::JsFunctionExpression(function_expression.clone()),
            )?)
        }
        JsAnyExpression::JsArrowFunctionExpression(function_expression) => {
            Ok(is_simple_function_expression(
                JsAnyFunction::JsArrowFunctionExpression(function_expression.clone()),
            )?)
        }
        _ => Ok(false),
    }
}
