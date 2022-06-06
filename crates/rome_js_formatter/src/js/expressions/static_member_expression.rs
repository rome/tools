use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write, VecBuffer};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyName, JsStaticMemberExpression,
    JsStaticMemberExpressionFields, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::AstNode;
use std::ops::Deref;

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn fmt_fields(node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
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

        let operator_token = operator_token?;
        let formatted_object =
            FormatMemberStaticExpression::from_expression(object?, &operator_token).fmt(f)?;

        let content = format_once(|f| {
            write!(f, [formatted_object, operator_token.format()])?;

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

                write!(
                    f,
                    [group_elements(&format_args![
                        FormatMemberStaticExpression::from_name(previous_member, &operator_token),
                        operator_token.format(),
                    ])]
                )?;

                previous_member = member?;
                current = parent_static_member_expression;
            }

            write!(f, [previous_member.format()])
        });

        write!(f, [group_elements(&content)])
    }
}

enum FormatMemberStaticExpression<'t> {
    JsAnyName {
        name: JsAnyName,
        operator: &'t JsSyntaxToken,
    },
    JsAnyExpression {
        expression: JsAnyExpression,
        operator: &'t JsSyntaxToken,
    },
}

impl<'t> FormatMemberStaticExpression<'t> {
    fn from_expression(node: JsAnyExpression, operator: &'t JsSyntaxToken) -> Self {
        Self::JsAnyExpression {
            expression: node,
            operator,
        }
    }
    fn from_name(node: JsAnyName, operator: &'t JsSyntaxToken) -> Self {
        Self::JsAnyName {
            name: node,
            operator,
        }
    }

    fn is_number_literal_expression(&self) -> bool {
        matches!(
            self,
            FormatMemberStaticExpression::JsAnyExpression {
                expression: JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(_)
                ),
                ..
            }
        )
    }

    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            FormatMemberStaticExpression::JsAnyName { name, .. } => name.syntax(),
            FormatMemberStaticExpression::JsAnyExpression { expression, .. } => expression.syntax(),
        }
    }

    fn operator(&self) -> &JsSyntaxToken {
        match self {
            FormatMemberStaticExpression::JsAnyName { operator, .. } => operator,
            FormatMemberStaticExpression::JsAnyExpression { operator, .. } => operator,
        }
    }
}

impl Deref for FormatMemberStaticExpression<'_> {
    type Target = JsSyntaxNode;

    fn deref(&self) -> &Self::Target {
        self.syntax()
    }
}

impl Format<JsFormatContext> for FormatMemberStaticExpression<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let is_member_number_literal = self.is_number_literal_expression();

        let object_has_trailing_trivia = self
            .last_trailing_trivia()
            .map_or(false, |trivia| trivia.pieces().len() > 0);

        let operator_has_leading_trivia = self.operator().leading_trivia().pieces().len() > 0;

        let format_node = format_with(|f| match self {
            FormatMemberStaticExpression::JsAnyName { name, .. } => {
                write!(f, [name.format()])
            }
            FormatMemberStaticExpression::JsAnyExpression { expression, .. } => {
                write!(f, [expression.format()])
            }
        });

        if is_member_number_literal && (object_has_trailing_trivia || operator_has_leading_trivia) {
            let mut buffer = VecBuffer::new(f.state_mut());
            write!(buffer, [format_node])?;
            let formatted_member = buffer.into_element();

            let (object_leading, object_content, object_trailing) = formatted_member.split_trivia();

            write!(
                f,
                [format_once(|f| {
                    f.write_element(object_leading)?;
                    write!(f, [token("(")])?;
                    f.write_element(object_content)?;
                    write!(f, [token(")")])?;
                    f.write_element(object_trailing)
                })]
            )
        } else {
            write!(f, [format_node])
        }
    }
}
