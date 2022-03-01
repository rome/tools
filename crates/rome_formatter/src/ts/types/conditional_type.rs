use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, hard_line_break, indent, soft_line_break, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsConditionalType;
use rslint_parser::ast::TsConditionalTypeFields;
use rslint_parser::{AstNode, JsSyntaxKind};

impl ToFormatElement for TsConditionalType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(self, formatter, false)
    }
}

fn format_conditional(
    current_node: &TsConditionalType,
    formatter: &Formatter,
    parent_is_conditional: bool,
) -> FormatResult<FormatElement> {
    let TsConditionalTypeFields {
        check_type,
        extends_token,
        extends_type,
        question_mark_token,
        true_type,
        colon_token,
        false_type,
    } = current_node.as_fields();

    let true_type = true_type?;
    let false_type = false_type?;

    let is_true_type_conditional = true_type.syntax().kind() == JsSyntaxKind::TS_CONDITIONAL_TYPE;
    let is_false_type_conditional = false_type.syntax().kind() == JsSyntaxKind::TS_CONDITIONAL_TYPE;

    let true_type = if is_true_type_conditional || parent_is_conditional {
        if is_true_type_conditional {
            let true_type = TsConditionalType::cast(true_type.syntax().to_owned()).unwrap();
            let true_type = format_conditional(&true_type, formatter, true)?;
            format_elements![
                question_mark_token.format(formatter)?,
                space_token(),
                true_type
            ]
        } else {
            format_elements![
                question_mark_token.format(formatter)?,
                space_token(),
                true_type.format(formatter)?
            ]
        }
    } else {
        format_elements![
            question_mark_token.format(formatter)?,
            space_token(),
            true_type.format(formatter)?,
        ]
    };

    let false_type = if is_false_type_conditional || parent_is_conditional {
        if is_false_type_conditional {
            let false_type = TsConditionalType::cast(false_type.syntax().to_owned()).unwrap();
            let false_type = format_conditional(&false_type, formatter, true)?;
            format_elements![colon_token.format(formatter)?, space_token(), false_type]
        } else {
            format_elements![
                colon_token.format(formatter)?,
                space_token(),
                false_type.format(formatter)?
            ]
        }
    } else {
        format_elements![
            colon_token.format(formatter)?,
            space_token(),
            false_type.format(formatter)?
        ]
    };

    let body = if is_true_type_conditional || is_false_type_conditional || parent_is_conditional {
        indent(format_elements![
            hard_line_break(),
            format_elements![soft_line_break(), true_type, soft_line_break(), false_type]
        ])
    } else {
        group_elements(format_elements![
            space_token(),
            true_type,
            space_token(),
            false_type
        ])
    };

    Ok(format_elements![
        check_type.format(formatter)?,
        space_token(),
        extends_token.format(formatter)?,
        space_token(),
        extends_type.format(formatter)?,
        space_token(),
        body
    ])
}
