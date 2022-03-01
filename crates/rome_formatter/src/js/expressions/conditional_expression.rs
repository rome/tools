use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, hard_line_break, indent, join_elements, soft_line_break, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsConditionalExpression;
use rslint_parser::ast::JsConditionalExpressionFields;
use rslint_parser::{AstNode, JsSyntaxKind};

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsConditionalExpressionFields {
            test,
            question_mark_token,
            consequent,
            colon_token,
            alternate,
        } = self.as_fields();

        let alternate = alternate?;
        let consequent = consequent?;

        let is_alternate_conditional =
            alternate.syntax().kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION;
        let is_consequent_conditional =
            consequent.syntax().kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION;
        let parent_is_conditional = self.syntax().parent().map_or(false, |n| {
            n.kind() == JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
        });

        let test = test.format(formatter)?;
        let consequent = format_elements![
            question_mark_token.format(formatter)?,
            space_token(),
            consequent.format(formatter)?,
        ];
        let alternate = format_elements![
            colon_token.format(formatter)?,
            space_token(),
            alternate.format(formatter)?
        ];

        let body = if is_alternate_conditional || is_consequent_conditional || parent_is_conditional
        {
            indent(format_elements![
                hard_line_break(),
                join_elements(soft_line_break(), vec![consequent, alternate],)
            ])
        } else {
            join_elements(space_token(), vec![consequent, alternate])
        };
        Ok(format_elements![test, space_token(), body])
    }
}
