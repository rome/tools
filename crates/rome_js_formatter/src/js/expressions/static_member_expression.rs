use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::ConcatBuilder;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyName, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::AstNode;
use std::ops::Deref;

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn format_fields(
        node: &JsStaticMemberExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let mut current = node.clone();

        while let Some(static_member_expression) =
            JsStaticMemberExpression::cast(current.object()?.into_syntax())
        {
            current = static_member_expression;
        }

        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = current.as_fields();

        let mut formatted = ConcatBuilder::default();

        let operator_token = operator_token?;
        let formatted_object =
            FormatMemberStaticExpression::from_expression(object?, &operator_token)
                .format(formatter)?;

        formatted.entry(formatted![
            formatter,
            [formatted_object, operator_token.format()]
        ]?);

        let mut previous_member = member?;

        while let Some(parent_static_member_expression) = current
            .syntax()
            .parent()
            .and_then(JsStaticMemberExpression::cast)
        {
            let JsStaticMemberExpressionFields {
                object: _,
                operator_token,
                member,
            } = parent_static_member_expression.as_fields();

            let operator_token = operator_token?;

            formatted.entry(group_elements(formatted![
                formatter,
                [
                    FormatMemberStaticExpression::from_name(previous_member, &operator_token),
                    operator_token.format(),
                ]
            ]?));

            previous_member = member?;
            current = parent_static_member_expression;
        }

        formatted.entry(formatted![formatter, [previous_member.format(),]]?);
        Ok(group_elements(formatted.finish()))
    }
}

enum FormatMemberStaticExpression<'t> {
    JsAnyName(JsAnyName, &'t JsSyntaxToken),
    JsAnyExpression(JsAnyExpression, &'t JsSyntaxToken),
}

impl<'t> FormatMemberStaticExpression<'t> {
    fn from_expression(node: JsAnyExpression, operator: &'t JsSyntaxToken) -> Self {
        Self::JsAnyExpression(node, operator)
    }
    fn from_name(node: JsAnyName, operator: &'t JsSyntaxToken) -> Self {
        Self::JsAnyName(node, operator)
    }

    fn is_number_literal_expression(&self) -> bool {
        matches!(
            self,
            FormatMemberStaticExpression::JsAnyExpression(
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                ),
                ..
            )
        )
    }

    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            FormatMemberStaticExpression::JsAnyName(node, _) => node.syntax(),
            FormatMemberStaticExpression::JsAnyExpression(node, _) => node.syntax(),
        }
    }

    fn operator(&self) -> &JsSyntaxToken {
        match self {
            FormatMemberStaticExpression::JsAnyName(_, operator) => operator,
            FormatMemberStaticExpression::JsAnyExpression(_, operator) => operator,
        }
    }
}

impl<'t> Deref for FormatMemberStaticExpression<'t> {
    type Target = JsSyntaxNode;

    fn deref(&self) -> &Self::Target {
        self.syntax()
    }
}

impl<'t> Format for FormatMemberStaticExpression<'t> {
    type Context = JsFormatContext;

    fn format(&self, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let formatted_member = formatted![formatter, [self.syntax().format()]]?;

        let is_member_number_literal = self.is_number_literal_expression();

        let object_has_trailing_trivia = self
            .last_trailing_trivia()
            .map_or(false, |trivia| trivia.pieces().len() > 0);

        let operator_has_leading_trivia = self.operator().leading_trivia().pieces().len() > 0;

        if is_member_number_literal && (object_has_trailing_trivia || operator_has_leading_trivia) {
            let (object_leading, object_content, object_trailing) = formatted_member.split_trivia();

            Ok(formatted![
                formatter,
                [
                    object_leading,
                    token("("),
                    object_content,
                    token(")"),
                    object_trailing,
                ]
            ]?)
        } else {
            Ok(formatted_member)
        }
    }
}
