use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, hard_line_break, indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsConditionalExpression;
use rslint_parser::ast::JsConditionalExpressionFields;
use rslint_parser::{AstNode, JsSyntaxKind, SyntaxNode};

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(self, formatter, false)
    }
}

fn format_conditional(
    current_node: &JsConditionalExpression,
    formatter: &Formatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    let JsConditionalExpressionFields {
        test,
        question_mark_token,
        consequent,
        colon_token,
        alternate,
    } = current_node.as_fields();

    let alternate = alternate?;
    let consequent = consequent?;

    let is_alternate_conditional =
        alternate.syntax().kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION;
    let is_consequent_conditional =
        consequent.syntax().kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION;

    let test = test.format(formatter)?;
    let consequent =
        if is_consequent_conditional || (parent_is_conditional && is_consequent_conditional) {
            let consequent = JsConditionalExpression::cast(consequent.syntax().to_owned()).unwrap();
            let consequent = format_conditional(&consequent, formatter, true)?;
            format_elements![
                question_mark_token.format(formatter)?,
                space_token(),
                consequent
            ]
        } else {
            format_elements![
                question_mark_token.format(formatter)?,
                space_token(),
                consequent.format(formatter)?
            ]
        };
    let alternate =
        if is_alternate_conditional || (parent_is_conditional && is_alternate_conditional) {
            let alternate = JsConditionalExpression::cast(alternate.syntax().to_owned()).unwrap();
            let alternate = format_conditional(&alternate, formatter, true)?;
            format_elements![colon_token.format(formatter)?, space_token(), alternate]
        } else {
            format_elements![
                colon_token.format(formatter)?,
                space_token(),
                alternate.format(formatter)?
            ]
        };

    let body = if is_alternate_conditional || is_consequent_conditional || parent_is_conditional {
        indent(format_elements![
            hard_line_break(),
            consequent,
            hard_line_break(),
            alternate
        ])
    } else {
        group_elements(format_elements![
            space_token(),
            consequent,
            space_token(),
            alternate
        ])
    };
    Ok(format_elements![test, space_token(), body])
}
