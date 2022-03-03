use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, hard_line_break, indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyExpression, JsConditionalExpression, JsConditionalExpressionFields, TsConditionalType,
    TsConditionalTypeFields, TsType,
};
use rslint_parser::{AstNode, SyntaxToken};

pub struct FormatConditionalPayload<'f, Node: AstNode + ToFormatElement> {
    pub question_mark: SyntaxToken,
    pub colon: SyntaxToken,
    pub consequent: Node,
    pub alternate: Node,
    pub formatter: &'f Formatter,
}

pub enum Conditional {
    Expression(JsConditionalExpression),
    Type(TsConditionalType),
}

impl Conditional {
    pub fn from_type(ts_type: TsType) -> Option<Self> {
        if let Some(TsType::TsConditionalType(conditional)) = TsType::cast(ts_type.syntax().clone())
        {
            Some(Self::Type(conditional))
        } else {
            None
        }
    }

    pub fn from_expression(any_expression: JsAnyExpression) -> Option<Self> {
        if let Some(JsAnyExpression::JsConditionalExpression(conditional)) =
            JsAnyExpression::cast(any_expression.syntax().clone())
        {
            Some(Self::Expression(conditional))
        } else {
            None
        }
    }
}

/// Utility function to use to format ternary operators
///
/// # Panics
///
/// It panics if it's used with nodes that are different from:
/// - [rslint_parser::ast::TsConditionalType]
/// - [rslint_parser::ast::JsConditionalExpression]
pub fn format_conditional(
    conditional: Conditional,
    formatter: &Formatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    let (head, body) = match conditional {
        Conditional::Expression(conditional_expression) => {
            let JsConditionalExpressionFields {
                consequent,
                colon_token,
                test,
                alternate,
                question_mark_token,
            } = conditional_expression.as_fields();

            (
                format_elements![test.format(formatter)?, space_token(),],
                format_conditional_body(
                    FormatConditionalPayload {
                        colon: colon_token?,
                        question_mark: question_mark_token?,
                        consequent: consequent?,
                        alternate: alternate?,
                        formatter,
                    },
                    parent_is_conditional,
                    |node| node.and_then(Conditional::from_expression),
                )?,
            )
        }
        Conditional::Type(conditional_type) => {
            let TsConditionalTypeFields {
                check_type,
                extends_token,
                extends_type,
                question_mark_token,
                true_type,
                colon_token,
                false_type,
            } = conditional_type.as_fields();

            (
                format_elements![
                    check_type.format(formatter)?,
                    space_token(),
                    extends_token.format(formatter)?,
                    space_token(),
                    extends_type.format(formatter)?,
                    space_token(),
                ],
                format_conditional_body(
                    FormatConditionalPayload {
                        colon: colon_token?,
                        question_mark: question_mark_token?,
                        consequent: true_type?,
                        alternate: false_type?,
                        formatter,
                    },
                    parent_is_conditional,
                    |node| node.and_then(Conditional::from_type),
                )?,
            )
        }
    };

    Ok(format_elements![head, body])
}

fn format_conditional_body<Node: AstNode + ToFormatElement, ToConditional>(
    payload: FormatConditionalPayload<Node>,
    parent_is_conditional: bool,
    to_conditional: ToConditional,
) -> FormatResult<FormatElement>
where
    ToConditional: Fn(Option<Node>) -> Option<Conditional>,
{
    let FormatConditionalPayload {
        colon,
        question_mark,
        formatter,
        consequent,
        alternate,
    } = payload;

    let mut left_or_right_is_conditional = false;
    let conditional = Node::cast(consequent.syntax().clone());
    let consequent = if let Some(consequent) = to_conditional(conditional) {
        left_or_right_is_conditional = true;
        let consequent = format_conditional(consequent, formatter, true)?;
        format_elements![question_mark.format(formatter)?, space_token(), consequent]
    } else {
        format_elements![
            question_mark.format(formatter)?,
            space_token(),
            consequent.format(formatter)?
        ]
    };

    let conditional = Node::cast(alternate.syntax().clone());
    let alternate = if let Some(alternate) = to_conditional(conditional) {
        left_or_right_is_conditional = true;
        let alternate = format_conditional(alternate, formatter, true)?;
        format_elements![colon.format(formatter)?, space_token(), alternate]
    } else {
        format_elements![
            colon.format(formatter)?,
            space_token(),
            alternate.format(formatter)?
        ]
    };

    let body = if left_or_right_is_conditional || parent_is_conditional {
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
    Ok(body)
}
