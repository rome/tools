use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, hard_line_break, indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsConditionalExpression, JsConditionalExpressionFields, TsConditionalType,
    TsConditionalTypeFields,
};
use rslint_parser::{AstNode, JsSyntaxKind, SyntaxNode, SyntaxToken};

pub struct FormatConditionalPayload<'f, Node: AstNode + ToFormatElement> {
    pub question_mark: SyntaxToken,
    pub colon: SyntaxToken,
    pub consequent: Node,
    pub alternate: Node,
    pub formatter: &'f Formatter,
}

pub fn format_conditional(
    current_node: &SyntaxNode,
    formatter: &Formatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    let (head, body) = match current_node.kind() {
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
            let conditional_expression =
                JsConditionalExpression::cast(current_node.to_owned()).unwrap();
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
                    |node| {
                        JsConditionalExpression::cast(node)
                            .unwrap()
                            .syntax()
                            .to_owned()
                    },
                )?,
            )
        }
        JsSyntaxKind::TS_CONDITIONAL_TYPE => {
            let conditional_type = TsConditionalType::cast(current_node.to_owned()).unwrap();
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
                    |node| TsConditionalType::cast(node).unwrap().syntax().to_owned(),
                )?,
            )
        }

        _ => unreachable!(
            "This function should be used only for JsConditionalExpression and TsConditionalType"
        ),
    };

    Ok(format_elements![head, body])
}

fn format_conditional_body<Node: AstNode + ToFormatElement, Cast>(
    payload: FormatConditionalPayload<Node>,
    parent_is_conditional: bool,
    cast: Cast,
) -> FormatResult<FormatElement>
where
    Cast: Fn(SyntaxNode) -> SyntaxNode,
{
    let FormatConditionalPayload {
        colon,
        question_mark,
        formatter,
        consequent,
        alternate,
    } = payload;

    let is_alternate_conditional = matches!(
        alternate.syntax().kind(),
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION | JsSyntaxKind::TS_CONDITIONAL_TYPE
    );

    let is_consequent_conditional = matches!(
        consequent.syntax().kind(),
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION | JsSyntaxKind::TS_CONDITIONAL_TYPE
    );

    let consequent = if is_consequent_conditional || parent_is_conditional {
        if is_consequent_conditional {
            let consequent = cast(consequent.syntax().to_owned());
            let consequent = format_conditional(&consequent, formatter, true)?;
            format_elements![question_mark.format(formatter)?, space_token(), consequent]
        } else {
            indent(format_elements![
                question_mark.format(formatter)?,
                space_token(),
                consequent.format(formatter)?,
            ])
        }
    } else {
        format_elements![
            question_mark.format(formatter)?,
            space_token(),
            consequent.format(formatter)?
        ]
    };
    let alternate = if is_alternate_conditional || parent_is_conditional {
        if is_alternate_conditional {
            let alternate = cast(alternate.syntax().to_owned());
            let alternate = format_conditional(&alternate, formatter, true)?;
            format_elements![colon.format(formatter)?, space_token(), alternate]
        } else {
            indent(format_elements![
                colon.format(formatter)?,
                space_token(),
                alternate.format(formatter)?
            ])
        }
    } else {
        format_elements![
            colon.format(formatter)?,
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
    Ok(body)
}
